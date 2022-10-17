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
        let form = format!("Not found: {}", server.err().clone().unwrap_or("[SERVER_URL]".to_string()));
        return match security_level {
            Security::Unknown => panic!("called `Result::unwrap()` on an `Err` value: \"ERROR CRITICAL\""),
            Security::Medium => "WARNING: check the server",
            Security::High => panic!("ERROR: program stops"),
            _ => &form,
        }.to_string();
    }

    server.clone().ok().unwrap_or("".to_string())
}
