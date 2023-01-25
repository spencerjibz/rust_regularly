#![allow(dead_code,unused_variables)]
use std::cmp;
fn main() {
    println!("{:?}","this ".as_bytes());
} //

fn lcs(string1: &str, string2: &str) ->  String {
    let total_rows = string1.len() + 1;
    let total_columns = string2.len() + 1;
    // rust doesn't allow accessing string by index
    let string1_chars = string1.as_bytes();
    let string2_chars = string2.as_bytes();

    let mut table = vec![vec![0; total_columns]; total_rows];

    for row in 1..total_rows{
        for col in 1..total_columns {
            if string1_chars[row - 1] == string2_chars[col - 1]{
                table[row][col] = table[row - 1][col - 1] + 1;
            } else {
                table[row][col] = cmp::max(table[row][col-1], table[row-1][col]);
            }
        }
    }

    let mut common_seq = Vec::new();
    let mut x = total_rows - 1;
    let mut y = total_columns - 1;

    while x != 0 && y != 0 {
        // Check element above is equal
        if table[x][y] == table[x - 1][y] {
            x = x - 1;
        }
        // check element to the left is equal
        else if table[x][y] == table[x][y - 1] {
            y = y - 1;
        }
        else {
            // check the two element at the respective x,y position is same
            assert_eq!(string1_chars[x-1], string2_chars[y-1]);
            let char = string1_chars[x - 1];
            common_seq.push(char);
            x = x - 1;
            y = y - 1;
        }
    }
    common_seq.reverse();
    String::from_utf8(common_seq).unwrap()
}




#[cfg(test)]
mod tests {
    use super::lcs;

    #[test]
    fn fixed_tests() {
        assert_eq!(lcs("", ""), "");
        assert_eq!(lcs("abc", ""), "");
        assert_eq!(lcs("", "abc"), "");
        assert_eq!(lcs("a", "b"), "");
        assert_eq!(lcs("a", "a"), "a");
        assert_eq!(lcs("abc", "a"), "a");
        assert_eq!(lcs("abc", "ac"), "ac");
        assert_eq!(lcs("abcdef", "abc"), "abc");
        assert_eq!(lcs("abcdef", "acf"), "acf");
        assert_eq!(lcs("anothertest", "notatest"), "nottest");
        assert_eq!(lcs("132535365", "123456789"), "12356");
        assert_eq!(lcs("nothardlythefinaltest", "zzzfinallyzzz"), "final");
        assert_eq!(lcs("abcdefghijklmnopq", "apcdefghijklmnobq"), "acdefghijklmnoq");
    }
}
