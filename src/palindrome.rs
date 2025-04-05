#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("rust_cxx_class/include/palindrome.h");

        type Palindrome_Class;

        fn new_Palindrome_Class() -> UniquePtr<Palindrome_Class>;
        fn is_palindrome(self: &Palindrome_Class,x:i32) -> bool;
    }
}

use rstest::rstest;

#[rstest]
#[case(121, true)]  // Basic palindrome
#[case(-121, false)]  // Negative numbers can't be palindromes
#[case(0, true)]  // Zero is a palindrome
#[case(12321, true)]  // Odd-length palindrome
#[case(1221, true)]  // Even-length palindrome
#[case(10, false)]  // Non-palindrome
#[case(i32::MAX, false)]  // Edge case
#[case(i32::MIN, false)]  // Edge case
#[case(1234567899, false)]  // Large non-palindrome
#[case(1234554321, true)]  // Large palindrome
fn test_is_palindrome(#[case] input: i32, #[case] expected: bool) {
    let palindrome = ffi::new_Palindrome_Class();
    let result = palindrome.is_palindrome(input);
    assert_eq!(result, expected);
}