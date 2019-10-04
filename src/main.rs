#[macro_use]
extern crate lazy_static;
extern crate dotenv;
extern crate mio_extras;
extern crate time;
extern crate json;
extern crate chrono;

use std::env;
use std::collections::HashMap;
use std::sync::Mutex;

use url;
use ws::util::Token;
use ws::{connect, Handler, Sender, Handshake, Result, Message, Request, Error, ErrorKind, CloseCode};
use uuid::Uuid;

mod messages;

macro_rules! block {
    ($xs:block) => {
        loop { let _ = $xs; break; }
    };
}

const HEARTBEAT: Token = Token(1);
// OCPP constants.
const CALL: u8 = 2;
const CALLRESULT: u8 = 3;
const CALLERROR: u8 = 4;
// Station constants.
const MODEL: &str = "Model";
const VENDOR_NAME: &str = "Vendor name";

lazy_static! {
    static ref MESSAGES: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

fn set_message(key: String, value: String) {
    MESSAGES.lock().unwrap().insert(key, value);
}

fn get_message(key: String) -> String {
    match MESSAGES.lock().unwrap().get(&key) {
        Some(value) => value.to_string(),
        None => "".to_string(),
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

        let msg_id = Uuid::new_v4();
        let msg = messages::create_boot_notification_request(msg_id.to_string(), serial_number, MODEL, VENDOR_NAME);

        set_message(msg_id.to_string(), msg.to_owned());

        self.out.send(msg)
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Raw message: {}", msg);

        let parsed_msg = match json::parse(msg.as_text()?) {
            Ok(result) => result,
            Err(e) => panic!("Error during parsing: {:?}", e),
        };

        let msg_type_id = match parsed_msg[0].as_u8() {
            Some(res) => res,
            None => panic!("Parsed message has no value."),
        };
        let msg_id = parsed_msg[1].to_string();

        println!("Message type ID: {}", msg_type_id);
        println!("Message ID: {}", msg_id);

        match msg_type_id {
            CALL => {
                let action = &parsed_msg[2].to_string();
                let payload = &parsed_msg[3];

                println!("CALL Action: {}", action);
                println!("CALL Payload: {}", payload);

                match action.as_str() {
                    "SetVariables" => {
                        // Send SetVariables response.

                        let response_msg = messages::create_set_variables_response(msg_id);

                        self.out.send(response_msg)?;
                    },
                    _ => println!("No request handler for action: {}", action),
                }
            },
            CALLRESULT => block!({
                let payload = &parsed_msg[2];

                println!("CALLRESULT Payload: {}", payload);

                let msg_from_map = get_message(msg_id);

                if msg_from_map == "" {
                    break;
                }

                let parsed_msg_from_map = match json::parse(&msg_from_map.to_owned()) {
                    Ok(result) => result,
                    Err(e) => panic!("Error during parsing: {:?}", e),
                };

                println!("Parsed message from map: {:?}", parsed_msg_from_map);

                let msg_from_map_action = &parsed_msg_from_map[2].to_string();
                let msg_from_map_payload = &parsed_msg_from_map[3];

                println!("Message from map payload: {:?}", msg_from_map_payload);

                match msg_from_map_action.as_str() {
                    "BootNotification" => {
                        // Check status of the response.
                        if payload["status"].to_string() == "Accepted" {
                            println!("BootNotification was accepted.");

                            // Send StatusNotification message.

                            let status_notification_msg_id = Uuid::new_v4();
                            let status_notification_msg = messages::create_status_notification_request(status_notification_msg_id.to_string(), 0, 1, "Available");

                            set_message(status_notification_msg_id.to_string(), status_notification_msg.to_owned());

                            self.out.send(status_notification_msg)?;

                            // Schedule a timeout to send Heartbeat once per day.
                            self.out.timeout(86000_000, HEARTBEAT)?; // TODO Unmock timeout.
                        }
                    },
                    _=> println!("No response handler for action: {}", msg_from_map_action),
                }
            }),
            CALLERROR => {
                let error_code = &parsed_msg[2];
                let error_description = &parsed_msg[3];
                let error_details = &parsed_msg[4];

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
                let msg = messages::create_heartbeat_request(msg_id.to_string());

                set_message(msg_id.to_string(), msg.to_owned());

                self.out.send(msg)?;

                // Schedule next message.
                self.out.timeout(86000_000, HEARTBEAT)?; // TODO Unmock timeout.

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
