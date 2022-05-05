#![deny(clippy::all)]
#![allow(dead_code)]

#[macro_use]
extern crate napi_derive;

#[napi]
fn sum(a: u32, b: u32) -> u32 {
  a + b
}

#[napi]
fn encode_rail_fence_cipher(text: String, num_rails: i32) -> String {
  let v: Vec<char> = text.chars().collect();

  rails(num_rails as isize, text.len() as isize)
    .into_iter()
    .map(|e| v[e])
    .collect()
}

#[napi]
fn decode_rail_fence_cipher(text: String, num_rails: i32) -> String {
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
