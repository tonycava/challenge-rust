#[derive(Debug)]
pub enum Security {
    Unknown,
    High,
    Medium,
    Low,
    BlockServer,
}
pub fn fetch_data(server: Result<String, String>, security_level: Security) -> String {
    println!("{:?}",server);
    println!("{:?}",security_level);

    if server.clone().err() != None {
        match security_level {
            Security::Unknown => panic!("called `Result::unwrap()` on an `Err` value: \"ERROR CRITICAL\""),
            Security::BlockServer => panic!("ERROR: program stops"),
            Security::Low => panic!("Not found: {}", server.unwrap_err()),
            Security::Medium => panic!("WARNING: check the server"),
            Security::High => panic!("ERROR: program stops"),
        }
    }
    server.ok().unwrap_or_else(|| "".to_string())
}
