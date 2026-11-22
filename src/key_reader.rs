use std::io;

const MAX_LENGTH_KEY: usize = 50;
const MIN_LENGTH_KEY: usize = 10;

pub fn create() -> Result<String, String> {
    let mut input = String::new();

    println!("Secret key:");

    loop {
        input.clear();

        if let Err(e) = io::stdin().read_line(&mut input) {
            return Err(format!("Error to read line: {}", e));
        }

        match validate_key(&input) {
            Ok(value) => return Ok(value),
            Err(e) => {
                println!("{}. Try again:", e);
            }
        };
    }
}

fn validate_key(key: &str) -> Result<String, String> {
    let trimed = key.trim();
    let len = trimed.chars().count();

    if len > MAX_LENGTH_KEY {
        Err(format!("Maximum number of characters: {}", MAX_LENGTH_KEY))
    } else if len < MIN_LENGTH_KEY {
        Err(format!("Minimum number of characters: {}", MIN_LENGTH_KEY))
    } else {
        Ok(trimed.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_key_short_is_err() {
        assert!(validate_key("k").is_err());
    }

    #[test]
    fn validate_key_long_is_err() {
        let key = "u".repeat(51);
        assert!(validate_key(&key).is_err());
    }

    #[test]
    fn validate_key_is_ok() {
        let key = "k".repeat(15);
        assert!(validate_key(&key).is_ok());
    }

    #[test]
    fn validate_key_min_is_ok() {
        let key = "k".repeat(MIN_LENGTH_KEY);
        assert!(validate_key(&key).is_ok());
    }

    #[test]
    fn validate_key_max_is_ok() {
        let key = "k".repeat(MAX_LENGTH_KEY);
        assert!(validate_key(&key).is_ok());
    }

    #[test]
    fn validate_key_trim_is_ok() {
        let key = "    kkkkkkkkkkkkkkk ";
        assert_eq!(validate_key(key).unwrap(), "kkkkkkkkkkkkkkk");
    }

    #[test]
    fn validate_key_chars_not_bytes() {
        let key = "ё".repeat(10);
        assert!(validate_key(&key).is_ok());
    }
}
