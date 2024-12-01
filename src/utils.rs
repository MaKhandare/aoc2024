pub fn read_input(day: u8) -> String {
    std::fs::read_to_string(format!("inputs/day{:02}.txt", day)).expect("Could not read input file")
}
