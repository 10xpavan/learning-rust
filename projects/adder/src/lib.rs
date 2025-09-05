pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]                             //thsi is called test annotation it indicates that it is a test function
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn another() {
        panic!("make this test fail");
    }
}
