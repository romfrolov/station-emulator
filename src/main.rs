#[macro_use]
extern crate lazy_static;
extern crate dotenv;
extern crate mio_extras;
extern crate time;

use std::env;
use std::collections::HashMap;
use std::sync::Mutex;
use url;
use ws::util::Token;
use ws::{connect, Handler, Sender, Handshake, Result, Message, Request, Error, ErrorKind, CloseCode};
use uuid::Uuid;

const HEARTBEAT: Token = Token(1);
const CALL: &str = "2";
const CALLRESULT: &str = "3";
const CALLERROR: &str = "4";

lazy_static! {
    static ref MESSAGES: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

fn set_message(key: String, value: String) {
    MESSAGES.lock().unwrap().insert(key, value);
}

fn get_message(key: String) -> String {
    match MESSAGES.lock().unwrap().get(&key) {
        Some(value) => value.to_string(),
        None => "".to_string()
    }
}

trait StringUtils {
    fn slice(&self, begin: usize, end: isize) -> Self;
}

impl StringUtils for String {
    fn slice(&self, begin: usize, mut end: isize) -> Self {
        if end < 0 {
            end *= -1;
            self[begin..self.chars().count() - end as usize].chars().collect()
        } else {
            self[begin..end as usize].chars().collect()
        }
    }
}

#[derive(Debug)]
struct Config {
    csms_url: String,
    station_id: String,
    serial_number: String,
}

// Websocket Handler struct.
struct Client {
    out: Sender,
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
        // Get serial number from environment.
        let serial_number = match env::var("SERIAL_NUMBER") {
            Ok(var) => var,
            Err(e) => panic!("Couldn't read SERIAL_NUMBER ({})", e),
        };

        // Send BootNotification request.

        let msg_type_id = CALL;
        let msg_id = Uuid::new_v4();
        let msg_action = "BootNotification";
        let msg_payload = format!("{{\"reason\":\"PowerUp\",\"chargingStation\":{{\"serialNumber\":\"{}\",\"model\":\"Model\",\"vendorName\":\"Vendor name\",\"firmwareVersion\":\"0.1.0\",\"modem\":{{\"iccid\":\"\",\"imsi\":\"\"}}}}}}", serial_number);

        let msg = format!("[{}, \"{}\", \"{}\", {}]", msg_type_id, msg_id, msg_action, msg_payload);

        set_message(msg_id.to_string(), msg.to_owned());

        self.out.send(msg)
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Raw message: {}", msg);

        let text_msg = match msg.as_text() {
            Ok(text) => &text[1..text.chars().count() - 1],
            Err(e) => panic!("Couldn't convert a message to text ({})", e),
        };

        // BUG Payload is being split which is undesired.
        let parsed_msg: Vec<&str> = text_msg.split(",").collect();

        let msg_type_id = parsed_msg[0];
        let msg_id = parsed_msg[1].to_string().slice(1, -1);

        println!("Message type ID: {}", msg_type_id);
        println!("Message ID: {}", msg_id);

        match msg_type_id {
            CALL => {
                let action = &parsed_msg[2].to_string().slice(1, -1);
                let payload = parsed_msg[3];

                println!("CALL Action: {}", action);
                println!("CALL Payload: {}", payload);

                match action.as_str() {
                    "SetVariables" => {
                        // Send SetVariables response.

                        let response_msg_type_id = CALLRESULT;
                        let response_msg_payload = "{\"setVariableResult\":[{\"attributeStatus\":\"Accepted\",\"component\":\"AuthCtrlr\",\"variable\":{\"name\":\"AuthorizeRemoteStart\"}}]}"; // TODO Unmock.

                        let response_msg = format!("[{}, \"{}\", {}]", response_msg_type_id, msg_id, response_msg_payload);

                        self.out.send(response_msg)?;
                    },
                    _ => println!("No request handler for action: {}", action),
                }
            },
            CALLRESULT => {
                let payload = parsed_msg[2];

                println!("CALLRESULT Payload: {}", payload);

                let msg_from_map = get_message(msg_id.to_string());

                println!("Message from map: {:?}", msg_from_map);

                // TODO Parse message from map and fix the lines below.
                let msg_from_map_action = &msg_from_map.to_string().slice(1, -1);
                // let msg_from_map_payload = parsed_msg[3];

                match msg_from_map_action.as_str() {
                    "BootNotification" => {
                        // TODO Get status from payload.

                        // TODO Activate connectors when received response on BootNotification.

                        // Send StatusNotification message.
                        let status_notification_msg_type_id = CALL;
                        let status_notification_msg_id = Uuid::new_v4();
                        let status_notification_msg_action = "StatusNotification";
                        let status_notification_msg_payload = "{\"timestamp\":\"2019-10-03T15:48:20+00:00\",\"connectorStatus\":\"Available\",\"evseId\":0,\"connectorId\":1}"; // TODO Unmock.

                        let status_notification_msg = format!("[{}, \"{}\", \"{}\", {}]", status_notification_msg_type_id, status_notification_msg_id, status_notification_msg_action, status_notification_msg_payload);

                        self.out.send(status_notification_msg)?;

                        // Schedule a timeout to send Heartbeat once per day.
                        self.out.timeout(86000_000, HEARTBEAT)?; // TODO Unmock.
                    },
                    _=> println!("No response handler for action: {}", msg_from_map_action),
                }

                // TODO Handler for BootNotification response:


                //statusNotificationMsgId, 'Available', evse_id, connnector.id
            },
            CALLERROR => {
                let error_code = parsed_msg[2];
                let error_description = parsed_msg[3];
                let error_details = parsed_msg[4];

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
                let msg_type_id = CALL;
                let msg_id = Uuid::new_v4();
                let msg_action = "Heartbeat";
                let msg_payload = "{}";

                self.out.send(format!("[{}, \"{}\", \"{}\", {}]", msg_type_id, msg_id, msg_action, msg_payload))?;

                // Schedule next message.
                self.out.timeout(86000_000, HEARTBEAT)?;
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

fn main() {
    dotenv::dotenv().expect("Failed to read .env file");

    let csms_url = match env::var("CSMS_URL") {
        Ok(var) => var,
        Err(e) => panic!("Couldn't read CSMS_URL ({})", e),
    };

    let station_id = match env::var("STATION_ID") {
        Ok(var) => var,
        Err(e) => panic!("Couldn't read STATION_ID ({})", e),
    };

    let serial_number = match env::var("SERIAL_NUMBER") {
        Ok(var) => var,
        Err(e) => panic!("Couldn't read SERIAL_NUMBER ({})", e),
    };

    let config = Config {
        csms_url: csms_url,
        station_id: station_id,
        serial_number: serial_number
    };

    println!("OCPP version: 2.0");
    println!("Serial number: {:?}", config.serial_number);
    println!("Station id: {:?}", config.station_id);

    let mut connection_string: String = config.csms_url.to_owned();
    connection_string.push_str("/");
    connection_string.push_str(&config.station_id);

    connect(connection_string, |out| { Client { out: out } }).unwrap()
}
