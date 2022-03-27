//use std::time::Instant;
fn main() {
    // let now = Instant::now();
    let mut c = Vigenere::new("password", "abcdefghijklmnopqrstuvwxyz");
    let result = c.decode("This is me place".to_owned());
    println!(" ecodeded(reversed) = {}  ", result);

    //println!("{:?}",now.elapsed())
}

#[derive(Debug)]
struct Vigenere {
    alphabet: Vec<char>,
    key: &'static str,

    shifts: Vec<usize>,
}


impl Vigenere {
    fn new(key: &'static str, alphabet: &'static str) -> Vigenere {
        Vigenere {
            key,
            alphabet: alphabet.chars().collect(),
            shifts: vec![],
        }
    }
    #[allow(dead_code)]
    #[inline(always)]
    fn encode(&mut self, msg: String) -> String {
        /*fill the message with the key to generate full key
        Example
         msg -> "my secret code i want to secure"
         full key-> "passwordpasswordpasswordpasswor"
         */

        let v: Vec<char> = self.key.chars().collect();
        let repeats: Vec<char> = msg
            .chars()
            .collect::<Vec<char>>()
            .chunks(self.key.chars().count())
            .map(|e| e.iter().enumerate().map(|(i, _n)| v[i]))
            .flatten()
            .collect();
        // print repeats

        self.shifts = repeats
            .iter()
            .map(|e| self.alphabet.iter().position(|n| n == e).unwrap())
            .collect();
        // print

        msg.char_indices()
            .map(|(ind, c)| {
                if self.alphabet.contains(&c) {
                    let pos = self.alphabet.iter().position(|e| e == &c).unwrap(); // r+d*k
                    return self.alphabet[(pos + self.shifts[ind]) % self.alphabet.len()];
                }
                c
            })
            .collect()
    }
    #[allow(dead_code)]
    #[inline(always)]
    fn decode(&mut self, msg: String) -> String {
        // check if there is a fullkey
        if self.shifts.len() < 1 {
            let v: Vec<char> = self.key.chars().collect();
            let repeats: Vec<char> = msg
                .chars()
                .collect::<Vec<char>>()
                .chunks(self.key.chars().count())
                .map(|e| e.iter().enumerate().map(|(i, _n)| v[i]))
                .flatten()
                .collect();

            self.shifts = repeats
                .iter()
                .map(|e| self.alphabet.iter().position(|n| n == e).unwrap())
                .collect();
        }

        msg.char_indices()
            .map(|(ind, c)| {
                if self.alphabet.contains(&c) {
                    let pos = self.alphabet.iter().position(|e| e == &c).unwrap();
                    return self.alphabet
                        [(pos + self.alphabet.len() - self.shifts[ind]) % self.alphabet.len()];
                }
                c
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn should_work() {
        let mut c = Vigenere::new("password", "abcdefghijklmnopqrstuvwxyz");
        let result = c.encode("codewars".to_owned());
        assert_eq!(result, "rovwsoiv".to_owned())
    }
}
