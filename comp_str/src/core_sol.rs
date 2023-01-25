
use std::{cmp, iter};
fn chop<'a>(input: &mut &'a str, predicate: impl Fn(char) -> bool) -> &'a str {
    //         &str      |c: char| -> bool {...}
    let idx = input.find(|c| !predicate(c)).unwrap_or(input.len());// !predicate !|false
   	// "..." => ".." ".."
    let (head, tail) = input.split_at(idx); // mut start .. index , index -t end
    *input = tail; // 
    head // 
}

// 
fn chunkify(mut string: &str) -> impl Iterator<Item = &str> {
    iter::from_fn(move || {
        string.chars().next().map(|c| {
            if c.is_ascii_digit() {
              //				  |c: char| -> bool { c.is_ascii_digit()}	
                chop(&mut string, |c| c.is_ascii_digit())
            } else {
                chop(&mut string, |c| !c.is_ascii_digit()) // !ftrue
            }
        })
    })
}

fn compare_n(lhs: &str, rhs: &str) -> cmp::Ordering {
    match Ord::cmp(&lhs.len(), &rhs.len()) {
        cmp::Ordering::Less => cmp::Ordering::Less,
        cmp::Ordering::Equal => Ord::cmp(lhs, rhs), // here wiith  Ord trait, Partial (same |Ordering(line 47) return here )
        cmp::Ordering::Greater => cmp::Ordering::Greater,
    }
}

// "Item 123" "Item 3" => cmp::Ordering 

// "Item 4"
// "Item 123"
// "Item 531"

pub fn compare(lhs: &str, rhs: &str) -> cmp::Ordering {
    let mut lhs_chunks = chunkify(lhs);
    let mut rhs_chunks = chunkify(rhs); // (Some('item'), Some('1123'))
  //  


    loop {
        match (lhs_chunks.next(), rhs_chunks.next()) {
            (Some(lhs), Some(rhs)) => { // here 
                let ord = match (
                    lhs.contains(|c: char| c.is_ascii_digit()), // check if a digit is contained 
                    rhs.contains(|c: char| c.is_ascii_digit()),// here, ascii - 
                ) {
                    (true, true) => compare_n(lhs, rhs), // line by line
                    (false, false) => Ord::cmp(lhs, rhs), // here 
                    (true, false) => cmp::Ordering::Less,
                    (false, true) => cmp::Ordering::Greater,
                }; // here Optional of Some->

                match ord {
                    cmp::Ordering::Less => return cmp::Ordering::Less,
                    cmp::Ordering::Equal => continue, // next 
                    cmp::Ordering::Greater => return cmp::Ordering::Greater,
                }
            }
            (Some(_), None) => return cmp::Ordering::Greater,
            (None, Some(_)) => return cmp::Ordering::Less,
            (None, None) => return cmp::Ordering::Equal, // Equal (not digits )
        }
    }
}
