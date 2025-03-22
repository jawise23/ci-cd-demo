//! 🧪 This file demonstrates a working Rust project with inline tests.
//! You can delete or expand this as needed for your application.

fn main() {
    // Entry point for your application logic
    println!("{}", get_greeting());
}

/// Replace this function with your own application logic.
///
/// This is just a placeholder used to demonstrate CI testing and formatting.
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
