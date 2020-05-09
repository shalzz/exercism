pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    for ch in string.chars() {
        match ch {
            '{' | '[' | '(' => stack.push(ch),
            '}' | ']' | ')' => match stack.pop() {
                Some('{') if ch != '}' => return false,
                Some('[') if ch != ']' => return false,
                Some('(') if ch != ')' => return false,
                None => return false,
                _ => continue,
            },
            _ => continue,
        }
    }

    stack.is_empty()
}
