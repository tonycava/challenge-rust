use expected_variable::*;

fn main() {
    let mut result = expected_variable("it_is_done", "almost_there");
    assert!(
        result.is_none(),
        "{}", format!("Should have been None and not, {:?}", result)
    );

    result = expected_variable("frankenstein", "Dracula");
    assert!(
        result.is_none(),
        "{}", format!("Should have been None and not, {:?}", result)
    );
}