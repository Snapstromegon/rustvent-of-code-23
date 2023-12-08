pub fn read_input(day: usize, example: bool, part: u8) -> String {
    let filename = format!(
        "inputs/{:02}{}.txt",
        day,
        if example {
            format!("-example-{part}")
        } else {
            "".to_string()
        }
    );
    std::fs::read_to_string(filename).unwrap()
}
