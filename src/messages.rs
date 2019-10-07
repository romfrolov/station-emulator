use chrono::prelude::*;
use json::stringify;

// OCPP constants.
const CALL: u8 = 2;
const CALLRESULT: u8 = 3;

pub fn create_boot_notification_request(msg_id: String, serial_number: String, model: &str, vendor_name: &str) -> String {
    let action = "BootNotification";
    let payload = object!{
        "reason" => "PowerUp",
        "chargingStation" => object!{
            "serialNumber" => serial_number,
            "model" => model,
            "vendorName" => vendor_name,
            "firmwareVersion" => "0.1.0",
            "modem" => object!{
                "iccid" => "",
                "imsi" => "",
            },
        },
    };

    format!("[{}, \"{}\", \"{}\", {}]", CALL, msg_id, action, stringify(payload))
}

pub fn create_status_notification_request(msg_id: String, evse_id: u8, connector_id: u8, status: &str) -> String {
    let action = "StatusNotification";
    let now = match Utc::now().with_nanosecond(0) {
        Some(res) => res.to_rfc3339(),
        None => panic!("Current date is empty."),
    };
    let payload = object!{
        "timestamp" => now,
        "connectorStatus" => status,
        "evseId" => evse_id,
        "connectorId" => connector_id,
    };

    format!("[{}, \"{}\", \"{}\", {}]", CALL, msg_id, action, stringify(payload))
}

pub fn create_heartbeat_request(msg_id: String) -> String {
    let action = "Heartbeat";
    let payload = "{}";

    format!("[{}, \"{}\", \"{}\", {}]", CALL, msg_id, action, payload)
}

pub fn create_transaction_event_request(msg_id: String, transaction_id: String, event_type: String, trigger_reason: String, charging_state: Option<String>, remote_start_id: Option<u64>, stopped_reason: Option<String>) -> String {
    let action = "TransactionEvent";
    let now = match Utc::now().with_nanosecond(0) {
        Some(res) => res.to_rfc3339(),
        None => panic!("Current date is empty."),
    };
    let mut payload = object!{
        "eventType" => event_type,
        "timestamp" => now,
        "triggerReason" => trigger_reason,
        "seqNo" => 0,
        "transactionData" => object!{
            "id" => transaction_id,
        },
    };

    match charging_state {
        Some(data) => payload["transactionData"]["chargingState"] = data.into(),
        _ => (),
    };

    match remote_start_id {
        Some(data) => payload["transactionData"]["remoteStartId"] = data.into(),
        _ => (),
    };

    match stopped_reason {
        Some(data) => payload["transactionData"]["stoppedReason"] = data.into(),
        _ => (),
    };

    format!("[{}, \"{}\", \"{}\", {}]", CALL, msg_id, action, stringify(payload))
}

pub fn create_set_variables_response(msg_id: String, attribute_status: String, component: String, variable: String) -> String {
    let payload = object!{
        "setVariableResult" => array![
            object!{
                "attributeStatus" => attribute_status.to_string(),
                "component" => component.to_string(),
                "variable" => object!{
                    "name" => variable.to_string(),
                },
            }
        ],
    };

    format!("[{}, \"{}\", {}]", CALLRESULT, msg_id, stringify(payload))
}

pub fn create_request_start_transaction_response(msg_id: String, remote_start_id: u64, status: String) -> String {
    let payload = object!{
        "remoteStartId" => remote_start_id,
        "status" => status,
    };

    format!("[{}, \"{}\", {}]", CALLRESULT, msg_id, stringify(payload))
}

pub fn create_request_stop_transaction_response(msg_id: String, status: String) -> String {
    let payload = object!{
        "status" => status,
    };

    format!("[{}, \"{}\", {}]", CALLRESULT, msg_id, stringify(payload))
}
