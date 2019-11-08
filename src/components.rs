pub fn get_variable(component_name: &str, variable_name: &str) -> (&'static str, Option<&'static str>) {
    match component_name {
        "AuthCtrlr" => {
            match variable_name {
                "AuthorizeRemoteStart" => ("Accepted", Some("false")),
                _ => ("UnknownVariable", None),
            }
        },
        _ => ("UnknownComponent", None),
    }
}
