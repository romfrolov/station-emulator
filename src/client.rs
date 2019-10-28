use std::env;
use std::collections::HashMap;
use std::sync::Mutex;

use url;
use ws::util::Token;
use ws::{Handler, Sender, Handshake, Result, Message, Request, Error, ErrorKind, CloseCode};
use uuid::Uuid;
use queues::*;
use chrono::prelude::*;

use crate::requests;
use crate::responses;

macro_rules! block {
    ($xs:block) => {
        loop { let _ = $xs; break; }
    };
}

// Timeout events.
const HEARTBEAT: Token = Token(1);
const QUEUE_FETCH: Token = Token(2);
// OCPP constants.
const CALL: u8 = 2;
const CALLRESULT: u8 = 3;
const CALLERROR: u8 = 4;
// Message queue constants.
const QUEUE_FETCH_INTERVAL: u64 = 50;
const QUEUE_MESSAGE_EXPIRATION: u64 = 10;

// Websocket Handler struct.
pub struct Client {
    pub out: Sender,
}

// Connector struct.
#[derive(Clone, Debug)]
struct Connector {
    status: String,
    operational: bool,
}

// Basic information about sent message.
#[derive(Clone, Debug)]
struct SentMessage {
    id: Option<String>,
    timestamp: Option<u64>,
}

lazy_static! {
    // Array of EVSE each item of which contains an array of connectors.
    static ref EVSES: Mutex<[[Connector; 1]; 1]> = Mutex::new([[Connector { status: "Inoperative".to_string(), operational: true }]]);
    // Sent OCPP messages hash map: message id => stringified message.
    static ref MESSAGES: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    // Saved transactions. transaction id => stringified transaction.
    static ref TRANSACTIONS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    // Pending messages queue.
    static ref Q: Mutex<Queue<String>> = Mutex::new(queue![]);
    // Last sent message.
    static ref LAST_SENT_MESSAGE: Mutex<SentMessage> = Mutex::new(SentMessage { id: None, timestamp: None });
}

static mut HEARTBEAT_INTERVAL: u64 = 0;

fn set_message(key: String, value: String) {
    MESSAGES.lock().unwrap().insert(key, value);
}

fn get_message(key: String) -> String {
    match MESSAGES.lock().unwrap().get(&key) {
        Some(value) => value.to_string(),
        None => "".to_string(),
    }
}

fn set_transaction(key: String, value: String) {
    TRANSACTIONS.lock().unwrap().insert(key, value);
}

fn get_transaction(key: String) -> String {
    match TRANSACTIONS.lock().unwrap().get(&key) {
        Some(value) => value.to_string(),
        None => "".to_string(),
    }
}

fn delete_transaction(key: String) {
    TRANSACTIONS.lock().unwrap().remove(&key);
}

fn set_connector_status(evse_index: usize, connector_index: usize, value: String) {
    EVSES.lock().unwrap()[evse_index][connector_index].status = value;
}
// NOTE Unused.
// fn set_connector_operational_status(evse_index: usize, connector_index: usize, value: bool) {
//     EVSES.lock().unwrap()[evse_index][connector_index].operational = value;
// }

fn get_connector(evse_index: usize, connector_index: usize) -> Connector {
    EVSES.lock().unwrap()[evse_index][connector_index].clone()
}

fn queue_size() -> usize {
    Q.lock().unwrap().size()
}

fn queue_add(s: String) {
    match Q.lock().unwrap().add(s) {
        Err(e) => println!("{:?}", e),
        _ => (),
    };
}

fn queue_pop() -> String {
    match Q.lock().unwrap().remove() {
        Ok(res) => res,
        Err(_) => "".to_string(),
    }
}

fn set_last_sent_message(id: String, timestamp: u64) {
    LAST_SENT_MESSAGE.lock().unwrap().id = Some(id);
    LAST_SENT_MESSAGE.lock().unwrap().timestamp = Some(timestamp);
}

fn get_last_sent_message() -> SentMessage {
    LAST_SENT_MESSAGE.lock().unwrap().clone()
}

// We implement the Handler trait for Client so that we can get more
// fine-grained control of the connection.
impl Handler for Client {

    fn build_request(&mut self, url: &url::Url) -> Result<Request> {
        let mut req = Request::from_url(url).unwrap();
        req.add_protocol("ocpp2.0");
        Ok(req)
    }

    fn on_open(&mut self, _: Handshake) -> Result<()> {
        // Start queue worker.
        self.out.timeout(QUEUE_FETCH_INTERVAL, QUEUE_FETCH)?;

        // Get serial number from environment.
        let serial_number = match env::var("SERIAL_NUMBER") {
            Ok(var) => var,
            Err(e) => panic!("Couldn't read SERIAL_NUMBER ({})", e),
        };

        // Get model from environment.
        let model = match env::var("MODEL") {
            Ok(var) => if var == "" { "Model".to_string() } else { var },
            _ => "Model".to_string(),
        };

        // Get vendor name from environment.
        let vendor_name = match env::var("VENDOR_NAME") {
            Ok(var) => if var == "" { "Vendor name".to_string() } else { var },
            _ => "Vendor name".to_string(),
        };

        // Send BootNotification request.

        let msg_id = Uuid::new_v4();
        let msg = requests::boot_notification(msg_id.to_string(), serial_number, &model.to_owned(), &vendor_name.to_owned());

        set_message(msg_id.to_string(), msg.to_owned());

        queue_add(msg);

        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        let parsed_msg = match json::parse(msg.as_text()?) {
            Ok(result) => result,
            Err(e) => panic!("Error during parsing: {:?}", e),
        };

        let msg_type_id = match parsed_msg[0].as_u8() {
            Some(res) => res,
            None => panic!("Parsed message has no value."),
        };
        let msg_id = parsed_msg[1].to_string();

        println!("Message ID: {}", msg_id);

        match msg_type_id {
            CALL => block!({
                let action = &parsed_msg[2].to_string();
                let payload = &parsed_msg[3];

                println!("CALL Action: {}", action);
                println!("CALL Payload: {}", payload);

                match action.as_str() {
                    "SetVariables" => {
                        // Send SetVariables response.

                        let set_variable_data = &payload["setVariableData"][0];
                        let component = set_variable_data["component"].to_string();
                        let variable_name = set_variable_data["variable"]["name"].to_string();

                        let response_msg = match component == "AuthCtrlr" {
                            true => match variable_name == "AuthorizeRemoteStart" {
                                true => responses::set_variables(msg_id, "Rejected".to_string(), component, variable_name),
                                false => responses::set_variables(msg_id, "UnknownVariable".to_string(), component, variable_name),
                            },
                            false => responses::set_variables(msg_id, "UnknownComponent".to_string(), component, variable_name),
                        };

                        self.out.send(response_msg)?;
                    },
                    "GetVariables" => {
                        // Send GetVariables response.

                        let get_variable_data = &payload["getVariableData"][0];
                        let component = get_variable_data["component"].to_string();
                        let variable_name = get_variable_data["variable"]["name"].to_string();

                        let response_msg = match component == "AuthCtrlr" {
                            true => match variable_name == "AuthorizeRemoteStart" {
                                true => responses::get_variables(msg_id, "Accepted".to_string(), component, variable_name, Some("false".to_string())),
                                false => responses::get_variables(msg_id, "UnknownVariable".to_string(), component, variable_name, None),
                            },
                            false => responses::get_variables(msg_id, "UnknownComponent".to_string(), component, variable_name, None),
                        };

                        self.out.send(response_msg)?;
                    }
                    "RequestStartTransaction" => {
                        let remote_start_id: u64 = match payload["remoteStartId"].as_number() {
                            Some(res) => u64::from(res),
                            None => panic!("Parsed message has no value."),
                        };

                        // Generate transaction id.
                        let transaction_id = Uuid::new_v4();

                        // Check connector status.
                        let evse_id = match payload["evseId"].as_number() {
                            Some(res) => usize::from(res),
                            _ => panic!("Parsed EVSE ID has no value."),
                        };

                        // FIXME Magic number (connector index).
                        let connector = get_connector(evse_id - 1, 0);

                        let mut response_status = "Accepted";

                        if connector.status != "Available" {
                            response_status = "Rejected";
                        }

                        // Send RequestStartTransaction response.

                        let request_start_transaction_msg = responses::request_start_transaction(msg_id, remote_start_id, response_status.to_string());

                        self.out.send(request_start_transaction_msg)?;

                        if response_status == "Rejected" {
                            break;
                        }

                        // Set EVSE status to "Occupied" and send StatusNotification with updated status.

                        let connector_status = "Occupied";
                        let status_notification_msg_id = Uuid::new_v4();
                        let status_notification_msg = requests::status_notification(status_notification_msg_id.to_string(), 1, 1, connector_status);

                        set_message(status_notification_msg_id.to_string(), status_notification_msg.to_owned());

                        queue_add(status_notification_msg);

                        set_connector_status(0, 0, connector_status.to_string());

                        // Send "Started" TransactionEvent request to notify CSMS about the started transaction.

                        let mut transaction_event_msg_id = Uuid::new_v4();
                        let mut transaction_event_msg = requests::transaction_event(transaction_event_msg_id.to_string(), transaction_id.to_string(), "Started".to_string(), "RemoteStart".to_string(), None, Some(remote_start_id), None);

                        set_message(transaction_event_msg_id.to_string(), transaction_event_msg.to_owned());

                        queue_add(transaction_event_msg);

                        // Save transaction.
                        set_transaction(transaction_id.to_string(), payload.dump());

                        // Send "Updated" TransactionEvent request to notify CSMS about the plugged in cable.

                        transaction_event_msg_id = Uuid::new_v4();
                        transaction_event_msg = requests::transaction_event(transaction_event_msg_id.to_string(), transaction_id.to_string(), "Updated".to_string(), "CablePluggedIn".to_string(), Some("Charging".to_string()), None, None);

                        set_message(transaction_event_msg_id.to_string(), transaction_event_msg.to_owned());

                        queue_add(transaction_event_msg);
                    },
                    "RequestStopTransaction" => {
                        // Get transaction from hash map.
                        let transaction = get_transaction(payload["transactionId"].to_string());

                        let response_status = match transaction.as_str() {
                            "" => "Rejected".to_string(),
                            _ => "Accepted".to_string(),
                        };

                        // Send RequestStopTransaction response.

                        let request_stop_transaction_msg = responses::request_stop_transaction(msg_id, response_status.to_string());

                        self.out.send(request_stop_transaction_msg)?;

                        if response_status == "Rejected" {
                            break;
                        }

                        // Send "Updated" TransactionEvent request to notify CSMS about remote stop command.

                        let mut transaction_event_msg_id = Uuid::new_v4();
                        let mut transaction_event_msg = requests::transaction_event(transaction_event_msg_id.to_string(), payload["transactionId"].to_string(), "Updated".to_string(), "RemoteStop".to_string(), None, None, None);

                        set_message(transaction_event_msg_id.to_string(), transaction_event_msg.to_owned());

                        queue_add(transaction_event_msg);

                        // Send "Ended" TransactionEvent request.

                        transaction_event_msg_id = Uuid::new_v4();
                        transaction_event_msg = requests::transaction_event(transaction_event_msg_id.to_string(), payload["transactionId"].to_string(), "Ended".to_string(), "RemoteStop".to_string(), None, None, Some("Remote".to_string()));

                        set_message(transaction_event_msg_id.to_string(), transaction_event_msg.to_owned());

                        queue_add(transaction_event_msg);

                        // Delete transaction.
                        delete_transaction(payload["transactionId"].to_string());

                        // Set EVSE status to "Available" and send StatusNotification with updated status.

                        let connector_status = "Available";
                        let status_notification_msg_id = Uuid::new_v4();
                        let status_notification_msg = requests::status_notification(status_notification_msg_id.to_string(), 1, 1, connector_status);

                        set_message(status_notification_msg_id.to_string(), status_notification_msg.to_owned());

                        queue_add(status_notification_msg);

                        set_connector_status(0, 0, connector_status.to_string());
                    },
                    _ => println!("No request handler for action: {}", action),
                }
            }),
            CALLRESULT => block!({
                let payload = &parsed_msg[2];

                let msg_from_map = get_message(msg_id);

                if msg_from_map == "" {
                    break;
                }

                let parsed_msg_from_map = match json::parse(&msg_from_map.to_owned()) {
                    Ok(result) => result,
                    Err(e) => panic!("Error during parsing: {:?}", e),
                };

                let msg_from_map_action = &parsed_msg_from_map[2].to_string();
                // NOTE Unused.
                // let msg_from_map_payload = &parsed_msg_from_map[3];

                match msg_from_map_action.as_str() {
                    "BootNotification" => {
                        // Check status of the response.
                        if payload["status"].to_string() == "Accepted" {
                            println!("BootNotification was accepted.");

                            // Set EVSE status to "Available" and send StatusNotification with updated status.

                            let connector_status = "Available";
                            let status_notification_msg_id = Uuid::new_v4();
                            let status_notification_msg = requests::status_notification(status_notification_msg_id.to_string(), 1, 1, connector_status);

                            set_message(status_notification_msg_id.to_string(), status_notification_msg.to_owned());

                            queue_add(status_notification_msg);

                            set_connector_status(0, 0, connector_status.to_string());

                            // Schedule a Heartbeat using the interval from BootNotification.

                            unsafe {
                                match payload["interval"].as_number() {
                                    Some(res) => HEARTBEAT_INTERVAL = u64::from(res) * 1000,
                                    None => panic!("Parsed message has no value."),
                                };

                                self.out.timeout(HEARTBEAT_INTERVAL, HEARTBEAT)?;
                            }
                        }
                    },
                    _=> println!("No response handler for action: {}", msg_from_map_action),
                }
            }),
            CALLERROR => {
                let error_code = parsed_msg[2].to_string();
                let error_description = parsed_msg[3].to_string();
                let error_details = parsed_msg[4].to_string();

                println!("CALLERROR Error code: {}", error_code);
                println!("CALLERROR Error Description: {}", error_description);
                println!("CALLERROR Error details: {}", error_details);
            },
            _ => println!("Unknown message type ID"),
        }

        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
       println!("WebSocket closing for ({:?}) {}", code, reason);
       println!("Shutting down server after first connection closes.");
       self.out.shutdown().unwrap();
   }

   // Shutdown on any error.
   fn on_error(&mut self, err: Error) {
        println!("Shutting down server for error: {}", err);
        self.out.shutdown().unwrap();
    }

    fn on_timeout(&mut self, event: Token) -> Result<()> {
        match event {
            HEARTBEAT => {
                // Send Heartbeat message.

                let msg_id = Uuid::new_v4();
                let msg = requests::heartbeat(msg_id.to_string());

                set_message(msg_id.to_string(), msg.to_owned());

                queue_add(msg);

                // Schedule next message.
                unsafe {
                    self.out.timeout(HEARTBEAT_INTERVAL, HEARTBEAT)?;
                }

                Ok(())
            },
            QUEUE_FETCH => {
                let current_timestamp: u64 = Utc::now().timestamp() as u64;

                let last_sent_msg = get_last_sent_message();
                // Check whether last sent message exists or not.
                let last_sent_msg_exist: bool = last_sent_msg.id != None;
                // Check whether last sent message has expired or not.
                let last_sent_msg_expired: bool = match last_sent_msg.timestamp {
                    Some(timestamp) => timestamp + QUEUE_MESSAGE_EXPIRATION < current_timestamp,
                    None => true,
                };

                if queue_size() > 0 && (!last_sent_msg_exist || last_sent_msg_expired) {
                    let msg = queue_pop();

                    if msg != "" {
                        let parsed_msg = match json::parse(&msg.to_owned()) {
                            Ok(result) => result,
                            Err(e) => panic!("Error during parsing: {:?}", e),
                        };

                        let msg_id = &parsed_msg[1].to_string();
                        let msg_action = &parsed_msg[2].to_string();

                        self.out.send(msg)?;

                        println!("{} ({}) was sent.", msg_action, msg_id);

                        set_last_sent_message(msg_id.to_string(), current_timestamp);
                    }
                }

                self.out.timeout(QUEUE_FETCH_INTERVAL, QUEUE_FETCH)?;

                Ok(())
            },
            // No other events are possible.
            _ => Err(Error::new(
                ErrorKind::Internal,
                "Invalid timeout token encountered!",
            )),
        }
    }
}