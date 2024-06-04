use p22::calc::*;

#[test]
fn integration_test_celsius2farenheit() {
    assert_eq!(celsius2farenheit(0), 32);
}

#[test]
fn integration_test_farenheit2celsius() {
    assert_eq!(farenheit2celsius(-40), -40);
}

#[test]
fn integration_test_fibonacci_loop() {
    assert_eq!(fibonacci_loop(5), 5);
}

#[test]
fn integration_test_fibonacci_rec() {
    assert_eq!(fibonacci_rec(6), 8);
}
