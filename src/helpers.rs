pub fn get_first_word(input: &String) -> &str {
    return input.trim().split(" ").collect::<Vec<&str>>()[0]
}
