use ch_05_tests::add; // นำเข้า add จาก src/lib.rs

#[test]
fn test_add_from_another_file() {
    assert_eq!(add(10, 20), 30);
}

#[test]
fn test_add_negative_numbers() {
    assert_eq!(add(-5, -10), -25);
}

#[test]
fn test_add_zero() {
    assert_eq!(add(0, 0), 0);
}
