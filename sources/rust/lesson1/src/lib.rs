#[feature(globs)]
fn hello() -> str {
    return "hello world!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canary() {
    }
    #[test]
    fn test_hello() {
        assert_eq!("hello world!", hello());
    }
    fn
}
