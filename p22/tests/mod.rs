use p22::calc::celsius2farenheit;

#[test]
fn integration_test_celsius2farenheit() {
    assert_eq!(celsius2farenheit(0), 32);
}

use p22::calc::farenheit2celsius;

#[test]
fn integration_test_farenheit2celsius() {
    assert_eq!(farenheit2celsius(-40), -40);
}

use p22::calc::fibonacci_loop;

#[test]
fn integration_test_fibonacci_loop() {
    assert_eq!(fibonacci_loop(5), 5);
}

use p22::calc::fibonacci_rec;

#[test]
fn integration_test_fibonacci_rec() {
    assert_eq!(fibonacci_rec(6), 8);
}
