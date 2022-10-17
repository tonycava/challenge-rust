#[derive(Debug, PartialEq)]
pub enum Security {
    Unknown,
    High,
    Medium,
    Low,
    BlockServer,
}

pub fn fetch_data(server: Result<String, String>, security_level: Security) -> String {
    println!("{:?}", server);
    println!("{:?}", security_level);

    if security_level == Security::BlockServer
        && server.unwrap_or("".to_string()) != ""
        && server.unwrap_err().contains("Ok") {
        panic!("ERROR: program stops")
    }

    if server.clone().err() != None {
        match security_level {
            Security::Unknown => panic!(""),
            Security::Low => panic!("Not found: {}", server.unwrap_err()),
            Security::Medium => panic!("WARNING: check the server"),
            Security::High => panic!("ERROR: program stops"),
            _ => {}
        }
    }

    server.ok().unwrap_or_else(|| "".to_string())
}
