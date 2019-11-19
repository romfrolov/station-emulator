use json::stringify;
use json::JsonValue;

// OCPP constant.
const CALLRESULT: u8 = 3;

fn wrap_call_result(msg_id: &str, payload: &str) -> String {
    format!("[{}, \"{}\", {}]", CALLRESULT, msg_id, payload)
}

pub fn set_variables(msg_id: &str, variables: JsonValue) -> String {
    let payload = object!{
        "setVariableResult" => variables,
    };

    wrap_call_result(msg_id, &stringify(payload))
}

pub fn get_variables(msg_id: &str, variables: JsonValue) -> String {
    let payload = object!{
        "getVariableResult" => variables,
    };

    wrap_call_result(msg_id, &stringify(payload))
}

pub fn request_start_transaction(msg_id: &str, remote_start_id: u64, status: &str) -> String {
    let payload = object!{
        "remoteStartId" => remote_start_id,
        "status" => status,
    };

    wrap_call_result(msg_id, &stringify(payload))
}

pub fn request_stop_transaction(msg_id: &str, status: &str) -> String {
    let payload = object!{
        "status" => status,
    };

    wrap_call_result(msg_id, &stringify(payload))
}
