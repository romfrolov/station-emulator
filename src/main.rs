extern crate dotenv;

use std::env;
use ws::{connect, CloseCode};

#[derive(Debug)]
struct Config {
    csms_url: String,
    ocpp_version: String,
    station_id: String,
    serial_number: String
}

fn main() {
    dotenv::dotenv().expect("Failed to read .env file");

    let csms_url = match env::var("CSMS_URL") {
        Ok(var) => var,
        Err(e) => panic!("Couldn't read CSMS_URL ({})", e),
    };

    let ocpp_version = match env::var("OCPP_VERSION") {
        Ok(var) => var,
        Err(e) => panic!("Couldn't read OCPP_VERSION ({})", e),
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
        ocpp_version: ocpp_version,
        station_id: station_id,
        serial_number: serial_number
    };

    println!("OCPP version: {}",  config.ocpp_version);
    println!("Serial number: {:?}", config.serial_number);
    println!("Station id: {:?}",    config.station_id);

    connect(config.csms_url, |out| {
        // TODO Send BootNotification request.
        // out.send(bootNotificationMsg).unwrap();

        move |msg| {
            println!("Got message: {}", msg);
            out.close(CloseCode::Normal)
        }
    }).unwrap()
}
