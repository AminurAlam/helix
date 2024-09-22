/// Toggle a boolean value
pub fn increment(selected_text: &str, _amount: i64) -> Option<String> {
    match selected_text.trim() {
        // Common boolean values
        "true" => Some(String::from("false")),
        "false" => Some(String::from("true")),

        // Python, Haskell
        "True" => Some(String::from("False")),
        "False" => Some(String::from("True")),

        // bash, fish
        "&&" => Some(String::from("||")),
        "||" => Some(String::from("&&")),

        // lua
        "and" => Some(String::from("or")),
        "or" => Some(String::from("and")),

        // comparisons
        ">" => Some(String::from("<=")),
        "<=" => Some(String::from(">")),

        "<" => Some(String::from(">=")),
        ">=" => Some(String::from("<")),

        "==" => Some(String::from("!=")),
        "!=" => Some(String::from("==")),

        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_boolean_toggles() {
        let tests = [
            // true/false
            ("true", 1, "false"),
            ("true", -1, "false"),
            ("false", 1, "true"),
            ("false", -1, "true"),
            // True/False
            ("True", 1, "False"),
            ("True", -1, "False"),
            ("False", 1, "True"),
            ("False", -1, "True"),
            // TRUE/FALSE
            ("TRUE", 1, "FALSE"),
            ("TRUE", -1, "FALSE"),
            ("FALSE", 1, "TRUE"),
            ("FALSE", -1, "TRUE"),
        ];
        for (original, amount, expected) in tests {
            assert_eq!(increment(original, amount).unwrap(), expected);
        }
    }
}
