/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// use wine_test_dummy::add;
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// C-compatible add function for FFI.
#[no_mangle]
pub extern "C" fn add_c(a: i32, b: i32) -> i32 {
    add(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }
}
