fn main() {
    println!("{}", get_greeting());
}

fn get_greeting() -> String {
    "Hello, Rustaceans".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_greeting() {
        assert_eq!(get_greeting(), "Hello, Rustaceans");
    }
}
