use chrono::prelude::*;
use json::stringify;

// OCPP constant.
const CALL: u8 = 2;

fn wrap_call(msg_id: String, action: String, payload: String) -> String {
    format!("[{}, \"{}\", \"{}\", {}]", CALL, msg_id, action, payload)
}

pub fn boot_notification(msg_id: String, serial_number: String, model: &str, vendor_name: &str) -> String {
    let action = "BootNotification".to_string();
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

    wrap_call(msg_id, action, stringify(payload))
}

pub fn status_notification(msg_id: String, evse_id: u8, connector_id: u8, status: &str) -> String {
    let action = "StatusNotification".to_string();
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

    wrap_call(msg_id, action, stringify(payload))
}

pub fn heartbeat(msg_id: String) -> String {
    let action = "Heartbeat".to_string();
    let payload = "{}".to_string();

    wrap_call(msg_id, action, payload)
}

pub fn transaction_event(msg_id: String, transaction_id: String, event_type: String, trigger_reason: String, charging_state: Option<String>, remote_start_id: Option<u64>, stopped_reason: Option<String>) -> String {
    let action = "TransactionEvent".to_string();
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

    wrap_call(msg_id, action, stringify(payload))
}
