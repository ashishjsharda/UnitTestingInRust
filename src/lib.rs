pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(3, add_two(5));
    }

    #[test]
    fn it_multiplies() {
        assert_eq!(multiply(6, 7), 42);
    }

    #[test]
    fn it_fails() {
        assert_eq!(multiply(6, 7), 43);
    }
}