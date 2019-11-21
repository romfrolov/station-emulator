use std::collections::HashMap;
use std::sync::Mutex;

use queues::*;

// Connector struct.
#[derive(Clone, Debug)]
pub struct Connector {
    pub status: &'static str,
    pub operational: bool,
}

// Basic information about sent message.
#[derive(Clone, Debug)]
pub struct SentMessage {
    pub id: Option<String>,
    pub timestamp: Option<u64>,
}

lazy_static! {
    // Array of EVSE each item of which contains an array of connectors.
    static ref EVSES: Mutex<[[Connector; 1]; 1]> = Mutex::new([[Connector { status: "Inoperative", operational: true }]]);
    // Sent OCPP messages hash map: message id => stringified message.
    static ref MESSAGES: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    // Saved transactions. transaction id => stringified transaction.
    static ref TRANSACTIONS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    // Pending messages queue.
    static ref QUEUE: Mutex<Queue<String>> = Mutex::new(queue![]);
    // Last sent message.
    static ref LAST_SENT_MESSAGE: Mutex<SentMessage> = Mutex::new(SentMessage { id: None, timestamp: None });
}

pub fn set_message(key: String, value: String) {
    MESSAGES.lock().unwrap().insert(key, value);
}

pub fn get_message(key: &str) -> String {
    match MESSAGES.lock().unwrap().get(key) {
        Some(value) => value.to_string(),
        None => String::from(""),
    }
}

pub fn set_transaction(key: String, value: String) {
    TRANSACTIONS.lock().unwrap().insert(key, value);
}

pub fn get_transaction(key: &str) -> String {
    match TRANSACTIONS.lock().unwrap().get(key) {
        Some(value) => value.to_string(),
        None => String::from(""),
    }
}

pub fn delete_transaction(key: &str) {
    TRANSACTIONS.lock().unwrap().remove(key);
}

pub fn set_connector_status(evse_index: usize, connector_index: usize, value: &'static str) {
    EVSES.lock().unwrap()[evse_index][connector_index].status = value;
}
// NOTE Unused.
// pub fn set_connector_operational_status(evse_index: usize, connector_index: usize, value: bool) {
//     EVSES.lock().unwrap()[evse_index][connector_index].operational = value;
// }

pub fn get_connector(evse_index: usize, connector_index: usize) -> Connector {
    EVSES.lock().unwrap()[evse_index][connector_index].clone()
}

pub fn queue_size() -> usize {
    QUEUE.lock().unwrap().size()
}

pub fn queue_add(s: String) {
    match QUEUE.lock().unwrap().add(s) {
        Err(e) => println!("{:?}", e),
        _ => (),
    };
}

pub fn queue_pop() -> String {
    match QUEUE.lock().unwrap().remove() {
        Ok(res) => res,
        Err(_) => String::from(""),
    }
}

pub fn set_last_sent_message(id: String, timestamp: u64) {
    LAST_SENT_MESSAGE.lock().unwrap().id = Some(id);
    LAST_SENT_MESSAGE.lock().unwrap().timestamp = Some(timestamp);
}

pub fn get_last_sent_message() -> SentMessage {
    LAST_SENT_MESSAGE.lock().unwrap().clone()
}
