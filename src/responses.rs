use json::stringify;

// OCPP constant.
const CALLRESULT: u8 = 3;

fn wrap_call_result(msg_id: &str, payload: &str) -> String {
    format!("[{}, \"{}\", {}]", CALLRESULT, msg_id, payload)
}

// TODO Support of array of variables.
pub fn set_variables(msg_id: &str, attribute_status: &str, component: &str, variable: &str) -> String {
    let payload = object!{
        "setVariableResult" => array![
            object!{
                "attributeStatus" => attribute_status,
                "component" => component,
                "variable" => object!{
                    "name" => variable,
                },
            }
        ],
    };

    wrap_call_result(msg_id, &stringify(payload))
}

// TODO Support of array of variables.
pub fn get_variables(msg_id: &str, attribute_status: &str, component: &str, variable: &str, attribute_value: Option<&str>) -> String {
    let mut payload = object!{
        "getVariableResult" => array![
            object!{
                "attributeStatus" => attribute_status,
                "component" => component,
                "variable" => object!{
                    "name" => variable,
                },
            }
        ],
    };

    match attribute_value {
        Some(data) => payload["getVariableResult"][0]["attributeValue"] = data.into(),
        _ => (),
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
