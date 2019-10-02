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

    let mut connection_string: String = config.csms_url.to_owned();
    connection_string.push_str("/");
    connection_string.push_str(&config.station_id);

    connect(connection_string, |out| {
        // TODO Send BootNotification request.
        let boot_notification_msg = "{\"reason\":\"PowerUp\",\"chargingStation\":{\"serialNumber\":\"emu2.0\",\"model\":\"Model\",\"vendorName\":\"Vendor name\",\"firmwareVersion\":\"0.1.0\",\"modem\":{\"iccid\":\"\",\"imsi\":\"\"}}}";

        out.send(boot_notification_msg).unwrap();

        move |msg| {
            println!("Got message: {}", msg);
            out.close(CloseCode::Normal)
        }
    }).unwrap()
}
