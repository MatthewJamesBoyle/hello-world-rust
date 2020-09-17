fn main() {
    println!("{}", get_message("Hello World"))
}

fn get_message(msg: &str) -> String {
    format!("Your message was: {}. Welcome to rust.", msg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_message() {
        assert_eq!(
            get_message("hello world"),
            "Your message was: hello world. Welcome to rust.",
        )
    }
}
