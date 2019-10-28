#[macro_use]
extern crate lazy_static;
extern crate dotenv;
extern crate mio_extras;
extern crate time;
#[macro_use]
extern crate json;
extern crate chrono;
extern crate queues;

use std::env;

use ws::{connect};

mod requests;
mod responses;
mod client;

// Station configuration struct.
#[derive(Debug)]
struct Config {
    csms_url: String,
    station_id: String,
    serial_number: String,
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

    connect(connection_string, |out| { client::Client { out: out } }).unwrap()
}
