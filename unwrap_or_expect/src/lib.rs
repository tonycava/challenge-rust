use std::fmt::format;

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
        && server.clone().unwrap_or("".to_string()) != ""
        && server.clone().unwrap_err().contains("Ok") {
        panic!("ERROR: program stops")
    }

    if server.clone().err() != None {
        match security_level {
            Security::Unknown => panic!("called `Result::unwrap()` on an `Err` value: \"ERROR CRITICAL\""),
            Security::Low => format!("Not found: {}", server.unwrap_or_else(|_| "[SERVER_URL]".to_string())),
            Security::Medium => "WARNING: check the server",
            Security::High => panic!("ERROR: program stops"),
            _ => {}
        }
    }

    server.clone().ok().unwrap_or("".to_string())
}
