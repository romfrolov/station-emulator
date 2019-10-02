extern crate dotenv;

use std::env;
use url;
use ws::{connect, Handler, Sender, Handshake, Result, Message, CloseCode, Request};
use uuid::Uuid;

#[derive(Debug)]
struct Config {
    csms_url: String,
    station_id: String,
    serial_number: String,
}

// Our Handler struct.
// Here we explicity indicate that the Client needs a Sender,
// whereas a closure captures the Sender for us automatically.
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

    // `on_open` will be called only after the WebSocket handshake is successful
    // so at this point we know that the connection is ready to send/receive messages.
    // We ignore the `Handshake` for now, but you could also use this method to setup
    // Handler state or reject the connection based on the details of the Request
    // or Response, such as by checking cookies or Auth headers.
    fn on_open(&mut self, _: Handshake) -> Result<()> {
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

        let parsed_msg: Vec<&str> = text_msg.split(",").collect();

        let msg_type_id = parsed_msg[0];
        let msg_id = parsed_msg[1];

        println!("Message type ID: {}", msg_type_id);
        println!("Message ID: {}", msg_id);

        match msg_type_id {
            "2" => {
                println!("CALL");

                // TODO Get action and payload of the message.

                // TODO Handler for SetVariables request.
            },
            "3" => {
                println!("CALLRESULT")

                // TODO Get payload of the message.

                // TODO Handler for BootNotification response:
                // - Activate connectors when received response on BootNotification.
                // - Start sending Heartbeat after receiving response on BootNotification.
            },
            "4" => {
                println!("CALLERROR")
            },
            _ => println!("Unknown message type ID"),
        }

        // Close the connection when we get a response from the server.
        self.out.close(CloseCode::Normal)
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
