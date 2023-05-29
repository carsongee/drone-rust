
pub fn answer() -> i32 {
    return 42
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn validate_answer() {
        assert_eq!(answer(), 42);
    }
}
