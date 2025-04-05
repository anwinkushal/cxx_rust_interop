// src/main.rs

use cxx::CxxVector;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("rust_cxx_class/include/e1_addition.h");

        type Addition_Class;

        fn new_Addition_Class() -> UniquePtr<Addition_Class>;
        fn add(self: &Addition_Class, a: &u64, b: &u64) -> u64;
        fn add_vector(self: &Addition_Class, data: &Vec<u32>) -> u64;
    }
}

use rstest::*;

#[rstest]
// Test cases for add() with two numbers
#[case(5, 6, 11)]
#[case(0, 0, 0)]
fn test_add_two_numbers(#[case] a: u64, #[case] b: u64, #[case] expected: u64) {
    let client = ffi::new_Addition_Class();
    let output = client.add(&a, &b);
    assert_eq!(output, expected);
}

#[rstest]
// Test cases for add_vector() with vectors
#[case(vec![], 0)] // Empty vector
#[case(vec![1], 1)] // Single element
#[case(vec![1, 2, 3, 4], 10)] // Multiple elements
#[case(vec![u32::MAX, 1], (u32::MAX as u64) + 1)] // Test overflow
fn test_add_vector(#[case] input: Vec<u32>, #[case] expected: u64) {
    let client = ffi::new_Addition_Class();
    let output = client.add_vector(&input);
    assert_eq!(output, expected);
}
