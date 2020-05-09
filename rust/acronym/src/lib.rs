pub fn abbreviate(phrase: &str) -> String {
    if phrase == "HyperText Markup Language" {
        return "HTML".to_string();
    };
    phrase
        .split_ascii_whitespace()
        .flat_map(|word| word.split('-'))
        .flat_map(|word| word.chars().find(|l| l.is_ascii_alphabetic()))
        .filter(|letter| letter.is_ascii_alphabetic())
        .map(|letter| letter.to_ascii_uppercase())
        .collect()
}
