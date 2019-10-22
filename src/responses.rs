use json::stringify;

// OCPP constant.
const CALLRESULT: u8 = 3;

fn wrap_call_result(msg_id: String, payload: String) -> String {
    format!("[{}, \"{}\", {}]", CALLRESULT, msg_id, payload)
}

// TODO Support of array of variables.
pub fn set_variables(msg_id: String, attribute_status: String, component: String, variable: String) -> String {
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

    wrap_call_result(msg_id, stringify(payload))
}

// TODO Support of array of variables.
pub fn get_variables(msg_id: String, attribute_status: String, component: String, variable: String, attribute_value: Option<String>) -> String {
    let mut payload = object!{
        "getVariableResult" => array![
            object!{
                "attributeStatus" => attribute_status.to_string(),
                "component" => component.to_string(),
                "variable" => object!{
                    "name" => variable.to_string(),
                },
            }
        ],
    };

    match attribute_value {
        Some(data) => payload["getVariableResult"][0]["attributeValue"] = data.into(),
        _ => (),
    };

    wrap_call_result(msg_id, stringify(payload))
}

pub fn request_start_transaction(msg_id: String, remote_start_id: u64, status: String) -> String {
    let payload = object!{
        "remoteStartId" => remote_start_id,
        "status" => status,
    };

    wrap_call_result(msg_id, stringify(payload))
}

pub fn request_stop_transaction(msg_id: String, status: String) -> String {
    let payload = object!{
        "status" => status,
    };

    wrap_call_result(msg_id, stringify(payload))
}
