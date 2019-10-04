use chrono::prelude::*;

// OCPP constants.
const CALL: u8 = 2;
const CALLRESULT: u8 = 3;
const CALLERROR: u8 = 4;

pub fn create_boot_notification_request(msg_id: String, serial_number: String, model: &str, vendor_name: &str) -> String {
    let action = "BootNotification";
    let payload = format!("{{\"reason\":\"PowerUp\",\"chargingStation\":{{\"serialNumber\":\"{}\",\"model\":\"{}\",\"vendorName\":\"{}\",\"firmwareVersion\":\"0.1.0\",\"modem\":{{\"iccid\":\"\",\"imsi\":\"\"}}}}}}", serial_number, model, vendor_name);

    format!("[{}, \"{}\", \"{}\", {}]", CALL, msg_id, action, payload)
}

pub fn create_status_notification_request(msg_id: String, evse_id: u8, connector_id: u8, status: &str) -> String {
    let action = "StatusNotification";
    let now = match Utc::now().with_nanosecond(0) {
        Some(res) => res.to_rfc3339(),
        None => panic!("Current date is empty."),
    };
    let payload = format!("{{\"timestamp\":\"{}\",\"connectorStatus\":\"{}\",\"evseId\":{},\"connectorId\":{}}}", now, status, evse_id, connector_id);

    format!("[{}, \"{}\", \"{}\", {}]", CALL, msg_id, action, payload)
}

pub fn create_heartbeat_request(msg_id: String) -> String {
    let action = "Heartbeat";
    let payload = "{}";

    format!("[{}, \"{}\", \"{}\", {}]", CALL, msg_id, action, payload)
}

pub fn create_set_variables_response(msg_id: String) -> String {
    let payload = "{\"setVariableResult\":[{\"attributeStatus\":\"Accepted\",\"component\":\"AuthCtrlr\",\"variable\":{\"name\":\"AuthorizeRemoteStart\"}}]}"; // TODO Unmock.

    format!("[{}, \"{}\", {}]", CALLRESULT, msg_id, payload)
}
