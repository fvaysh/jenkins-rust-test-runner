/// Adds two numbers together.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Subtracts one number from another.
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

/// Multiplies two numbers together.
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Divides two numbers (handles division by zero).
pub fn divide(a: i32, b: i32) -> Option<f64> {
    if b == 0 {
        None
    } else {
        Some(a as f64 / b as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(10, 4), 6);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(3, 5), 15);
    }

    #[test]
    fn test_divide_valid() {
        assert_eq!(divide(10, 2), Some(5.0));
    }

    #[test]
    fn test_divide_by_zero() {
        assert_eq!(divide(5, 0), None);
    }
}
