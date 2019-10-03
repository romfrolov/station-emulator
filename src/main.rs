extern crate dotenv;
extern crate mio_extras;
extern crate time;

use std::env;
use url;
use ws::util::Token;
use ws::{connect, Handler, Sender, Handshake, Result, Message, Request, Error, ErrorKind, CloseCode};
use uuid::Uuid;

const HEARTBEAT: Token = Token(1);

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
        // Schedule a timeout to send Heartbeat once per day.
        self.out.timeout(1_000, HEARTBEAT)?;

        // Send BootNotification request.

        let msg_type_id = "2";
        let msg_id = Uuid::new_v4();
        let msg_action = "BootNotification";
        let msg_payload = "{\"reason\":\"PowerUp\",\"chargingStation\":{\"serialNumber\":\"emu2.0\",\"model\":\"Model\",\"vendorName\":\"Vendor name\",\"firmwareVersion\":\"0.1.0\",\"modem\":{\"iccid\":\"\",\"imsi\":\"\"}}}";

        self.out.send(format!("[{}, \"{}\", \"{}\", {}]", msg_type_id, msg_id, msg_action, msg_payload))
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
        let msg_id = parsed_msg[1];

        println!("Message type ID: {}", msg_type_id);
        println!("Message ID: {}", msg_id);

        match msg_type_id {
            "2" => {
                println!("CALL");

                let action = parsed_msg[2];
                let payload = parsed_msg[3];

                println!("Action: {}", action);
                println!("Payload: {}", payload);

                // TODO Handler for SetVariables request.
            },
            "3" => {
                println!("CALLRESULT");

                let payload = parsed_msg[2];

                println!("Payload: {}", payload);

                // TODO Handler for BootNotification response:
                // - Activate connectors when received response on BootNotification.
                // - Start sending Heartbeat after receiving response on BootNotification.
            },
            "4" => {
                println!("CALLERROR");
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
                println!("DEBUG: Heartbeat");
                // TODO Send Heartbeat message.

                // Schedule next message.
                self.out.timeout(1_000, HEARTBEAT)?;
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
