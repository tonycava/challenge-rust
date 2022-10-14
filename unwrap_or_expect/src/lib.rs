pub enum Security {
    Unknown,
    High,
    Medium,
    Low,
    BlockServer,
}

pub fn fetch_data(server: Result<String, String>, security_level: Security) -> String {
    if server.clone().err() != None {
        return match security_level {
            Security::Unknown => panic!(""),
            Security::BlockServer => panic!("ERROR: program stops"),
            Security::Low => panic!("Not found: [SERVER_URL]"),
            Security::Medium => panic!("WARNING: check the server"),
            Security::High => panic!("ERROR: program stops"),
        };
    }

    server.ok().unwrap_or_else(|| "".to_string())
}
