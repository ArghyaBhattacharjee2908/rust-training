/// Convers Celsius to Farenheit
/// ```
///use p22::calc::celsius2farenheit;
///
/// assert_eq!(celsius2farenheit(0), 32);
/// ```

pub fn celsius2farenheit(celsius: i32) -> i32 {
    (celsius / 5) * 9 + 32
}

/// Convers Farenheit to Celsius
/// ```
///use p22::calc::farenheit2celsius;
///
/// assert_eq!(farenheit2celsius(-40), -40);
/// ```

pub fn farenheit2celsius(farenheit: i32) -> i32 {
    5 * ((farenheit - 32) / 9)
}

/// Calculates n-th Fibonacchi Number by Loop
/// ```
///use p22::calc::fibonacci_loop;
///
/// assert_eq!(fibonacci_loop(5), 5);
/// ```

pub fn fibonacci_loop(n: u32) -> u32 {
    let mut count: u32 = 1;
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    let mut c: u32 = 0;
    if n == 0 {
        c = a;
    } else if n == 1 {
        c = b;
    } else {
        loop {
            count += 1;
            if count <= n {
                c = a + b;
                a = b;
                b = c;
                continue;
            } else {
                break;
            }
        }
    }
    c
}

/// Calculates n-th Fibonacchi Number by Recursion
/// ```
///use p22::calc::fibonacci_rec;
///
/// assert_eq!(fibonacci_rec(6), 8);
/// ```

pub fn fibonacci_rec(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        m => fibonacci_rec(m - 1) + fibonacci_rec(m - 2),
    }
}

#[cfg(test)]

mod tests {
    use super::celsius2farenheit;

    #[test]
    fn test_celcius2farenheit() {
        assert_eq!(celsius2farenheit(0), 32);
    }

    use super::farenheit2celsius;

    #[test]
    fn test_farenheit2celcius() {
        assert_eq!(farenheit2celsius(-40), -40);
    }

    use super::fibonacci_loop;

    #[test]
    fn test_fibonacci_loop() {
        assert_eq!(fibonacci_loop(5), 5);
    }

    use super::fibonacci_rec;

    #[test]
    fn test_fibonacci_rec() {
        assert_eq!(fibonacci_rec(6), 8);
    }
}
