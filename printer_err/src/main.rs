#![allow(dead_code, unused)]
fn main() {
    println!("{}",square_digits(9119));
}

fn printer_error(s: &str) -> String {
    // Your code here

    let count = s.chars().filter(|c| c > &'m').count();

    format!("{}/{}", count, s.len())
}

fn add_binary(a: u64, b: u64) -> String {
    format!("{:b}", a + b)
}

fn square_digits(num: u64) -> u64 {
    let mut s = String::new();
    num.to_string().chars().for_each(|e| {
        let n = e.to_digit(10).unwrap();

        s.push_str(&format!("{}", n * n));
    });


    s.parse::<u64>().unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_pass_all_the_tests_provided() {
        assert_eq!(
            &printer_error("aaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyz"),
            "3/56"
        );
        assert_eq!(
            &printer_error("kkkwwwaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyz"),
            "6/60"
        );
        assert_eq!(
            &printer_error("kkkwwwaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyzuuuuu"),
            "11/65"
        );
    }

    use super::add_binary;

    fn dotest(a: u64, b: u64, expected: &str) {
        let actual = add_binary(a, b);
        assert!(
            actual == expected,
            "With a = {a}, b = {b}\nExpected \"{expected}\" but got \"{actual}\""
        )
    }

    #[test]
    fn sample_tests() {
        dotest(1, 1, "10");
        dotest(0, 1, "1");
        dotest(1, 0, "1");
        dotest(2, 2, "100");
        dotest(51, 12, "111111");
    }
    use super::square_digits;

    #[test]
    fn test_square_digits() {
        assert_eq!(square_digits(9119), 811181, "\nFailed with num 9119");
    }
}
