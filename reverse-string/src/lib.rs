pub fn reverse(input: &str) -> String {
    let rev : &str = input;
    return rev.chars().rev().collect::<String>();
}
