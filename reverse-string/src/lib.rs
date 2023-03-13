pub fn reverse(input: &str) -> String {
    let mut reversed = String::new();

    for character in input.chars() {
        reversed.insert(0, character);
    }

    reversed
}
