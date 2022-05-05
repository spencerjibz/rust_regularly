#![allow(dead_code,unused_imports)]
mod lib;
use lib::crc32;
#[inline]
fn make_crc_table(n:usize) ->  Option<u32> { 
 
 
 let  mut c = n as isize;

  for _ in 0..8 { 
      c =  if c&1 ==1 { 0xEDB88320_isize ^ (c >> 1)} else {c >> 1 };
  }
  Some( c  as u32)
 

}
#[inline]
pub fn crc3 (st:&str) -> u32 { 
    let mut crc = 0xFFFFFFFF;
   
    st.chars().for_each(|ch| { 
        let   code = ch as u32;
		let index = ((crc^code) & 0xFF) as usize;
	 if let Some(done)  =  make_crc_table(index) {
	  //dbg!(done,table.nth(index));
	   
           crc = ((crc >>8) &  0x00FFFFFF) ^done;

     }

		   
    });
    crc^0xFFFFFFFF
}


use std::time::Instant;

fn main() {
   // let ve:Vec<u32>= make_crc_table().collect();
    let n = Instant::now();
     println!("{} " ,crc3("Hello, my name is spencer"));
    // println!("{:?}",ve);
    println!("{:?}", n.elapsed())
}

