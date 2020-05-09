pub fn reply(message: &str) -> &str {
    let is_question = message.trim().chars().last().unwrap_or_else(|| 'n') == '?';
    let alpha_msg: String = message
        .chars()
        .filter(|&c| char::is_alphabetic(c))
        .collect();
    let is_yelling = !alpha_msg.is_empty() && alpha_msg.to_ascii_uppercase() == alpha_msg;
    let is_empty = message.trim().is_empty();

    match message {
        _ if is_empty => "Fine. Be that way!",
        _ if is_question && is_yelling => "Calm down, I know what I'm doing!",
        _ if is_question => "Sure.",
        _ if is_yelling => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
