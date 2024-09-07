use crypto_sphere::verification::{verify_caesar_key, verify_xor_key};

#[test]
fn test_verify_caesar_key() {
    let result = verify_caesar_key("3");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 3);

    let result_invalid = verify_caesar_key("abc");
    assert!(result_invalid.is_err());
}

#[test]
fn test_verify_xor_key() {
    let result = verify_xor_key("A");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), b'A');

    let result_invalid = verify_xor_key("AA");
    assert!(result_invalid.is_err());
}
