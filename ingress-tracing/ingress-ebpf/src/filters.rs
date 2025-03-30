

pub fn port_filter(lower: &u16, higher: &u16, input_port: &u16) -> bool {
    if input_port > lower && input_port < higher {
        return true
    }
    false
}


