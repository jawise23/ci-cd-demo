fn main() {
    println!("{}", excited_greeting());
}

fn get_greeting() -> String {
    "Hello, Rustaceans".to_string()
}

fn excited_greeting() -> String {
    format!("{}!!!", get_greeting())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_greeting() {
        assert_eq!(get_greeting(), "Hello, Rustaceans!");
    }
}
