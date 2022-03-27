use std::time::Instant;
fn main() {
    // let mut n = Rail::new("Hello, World!",3);
    let now = Instant::now();
    println!(
        "{:?}",
        decode_rail_fence_cipher("WECRLTEERDSOEEFEAOCAIVDEN", 3)
    );
    println!("{:?}", now.elapsed())
}

/*

Create two functions to encode and then decode a string using the Rail Fence Cipher. This cipher is used to encode a string by placing each character successively in a diagonal along a set of "rails". First start off moving diagonally and down. When you reach the bottom, reverse direction and move diagonally and up until you reach the top rail. Continue until you reach the end of the string. Each "rail" is then read left to right to derive the encoded string.

For example, the string "WEAREDISCOVEREDFLEEATONCE" could be represented in a three rail system as follows:

W       E       C       R       L       T       E
  E   R   D   S   O   E   E   F   E   A   O   C
    A       I       V       D       E       N
The encoded string would be:

WECRLTEERDSOEEFEAOCAIVDEN
Write a function/method that takes 2 arguments, a string and the number of rails, and returns the ENCODED string.

Write a second function/method that takes 2 arguments, an encoded string and the number of rails, and returns the DECODED string.

For both encoding and decoding, assume number of rails >= 2 and that passing an empty string will return an empty string.

Note that the example above excludes the punctuation and spaces just for simplicity. There are, however, tests that include punctuation. Don't filter out punctuation as they are a part of the string.


*/
#[allow(dead_code)]
fn rails(rn: isize, ln: isize) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::with_capacity(ln as usize);

    for rc in 0..rn {
        let mut rd = rc;
        let mut rv = rc;

        while rv < ln {
            result.push(rv as usize);
            let mid = if rn == (rd + 1) { 0 } else { rd };
            rv += 2 * (rn - 1 - mid);

            rd = rn - 1 - rd;
        }
    }
    result
}
#[allow(dead_code)]
fn encode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
    let v: Vec<char> = text.chars().collect();

    rails(num_rails as isize, text.len() as isize)
        .into_iter()
        .map(|e| v[e])
        .collect()
}
fn decode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
    let v: Vec<char> = text.chars().collect();
    let mut res = v.clone();

    rails(num_rails as isize, text.len() as isize)
        .into_iter()
        .enumerate()
        .for_each(|(ind, c)| {
            res[c] = v[ind];
        });

    String::from_iter(res)
}

#[test]
fn basic_tests() {
    assert_eq!(
        encode_rail_fence_cipher("WEAREDISCOVEREDFLEEATONCE", 3),
        "WECRLTEERDSOEEFEAOCAIVDEN"
    );
    assert_eq!(
        decode_rail_fence_cipher("WECRLTEERDSOEEFEAOCAIVDEN", 3),
        "WEAREDISCOVEREDFLEEATONCE"
    );
    assert_eq!(
        encode_rail_fence_cipher("Hello, World!", 3),
        "Hoo!el,Wrdl l"
    );
    assert_eq!(
        decode_rail_fence_cipher("Hoo!el,Wrdl l", 3),
        "Hello, World!"
    );
}
