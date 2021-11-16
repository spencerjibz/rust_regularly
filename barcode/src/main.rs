 #![feature(array_methods)]
use std::time::Instant;
use std::str;

#[allow(dead_code)]
static GROUP:[&str;10] = [ "LLLLLL",
"LLGLGG",
"LLGGLG",
"LLGGGL",
"LGLLGG",
"LGGLLG",
"LGGGLL",
"LGLGLG",
"LGLGGL",
"LGGLGL"];
#[allow(dead_code)]
 static LCODE:[&str;10]=["0001101",
 "0011001",
 "0010011",
       "0111101",
       "0100011","0110001",
       "0101111","0111011","0110111",
       "0001011"];
       #[allow(dead_code)]
      static GCODE:[&str;10]=["0100111","0110011","0011011",
       "0100001","0011101","0111001",
       "0000101","0010001","0001001",
       "0010111"];

       #[allow(dead_code)]
        static RCODE:[&str;10]= ["1110010","1100110","1101100",
       "1000010","1011100","1001110",
       "1010000","1000100","1001000",
       "1110100"];



#[allow(dead_code,unused_variables,unused_imports)]
fn read_barcode(first:usize,code:&str) -> String { 
// get the different groups
// 
let ( first_half,  second_half) =code.split_at(50);

 let second_half = &second_half[0..42];
 let first_half = &first_half[3..45];
//println!(" second {} first = {}",second_half,first_half);
// get the firstgroup of words#

let  bin_nums = first_half.as_bytes().chunks(7).map(str::from_utf8).collect::<Result<Vec<&str>,_>>().unwrap();
let last_numbs = second_half.as_bytes().chunks(7).map(str::from_utf8).collect::<Result<Vec<&str>,_>>().unwrap();
//let e = Instant::now();
//println!("{:?}, {:?}",bin_nums,last_numbs);
let word = GROUP[first];
let mut encod:Vec<(String,String)> = word.chars().enumerate().map(|(i,e)| { 
 
(format!("{}{}",e,i),bin_nums[i].to_string())
  
}).collect();


  for (key,val) in  encod .iter_mut() { 
  

if key.contains('L') { 

     match LCODE.iter().position(|e|e==val) { 

        Some(v) => { 
            *val  =  v.to_string();
        },
        _ => {}
     }
 
  
}
else { 
 
     match GCODE.iter().position(|e|e==val){ 
  Some(v) => { 
      *val = v.to_string();
  },
  _ => { 

  }
    }
}

}

//println!("{:?}",e.elapsed());
// find the last six digits

let last:String = last_numbs.iter().map(|e| RCODE.iter().position(|f|f == e).unwrap().to_string()).collect();


// 
 let fina :String= encod.into_iter().map(|e|e.1).collect();
 
 format!("{}{}{}",first,fina,last)


}


#[allow(dead_code,unused_variables,unused_imports)]
fn encode_barcode(code:&str) -> String { 
     
 let barcode =  if code.len() <=12 {format!("{}{}",code,check(code))} else { code.to_string()};
     
 let (first, first_six,last) = (barcode[..=0].parse::<usize>().unwrap(), &barcode[1..=6],&barcode[7..]);
//println!("{} {} {} {}",code,first,first_six,last);
//println!("{} {} {}",first_six,first,last);
let encoding:Vec<char>= GROUP[first].chars().collect();

let first_six_str = first_six.chars().enumerate().map(|(id,v)| {
    
    if encoding[id] == 'L' { 
        
        let index = v.to_digit(10).unwrap();
        LCODE[index as usize]
    }
     else {
        let index = v.to_digit(10).unwrap();
        GCODE[index as usize]
     }
});

// encoding the last six 
let last_six = last.chars().map(|e|RCODE[e.to_digit(10).unwrap() as usize]);

   format!("101{}01010{}101",first_six_str.collect::<String>(),last_six.collect::<String>())

}
#[allow(dead_code,unused_variables,unused_imports)] 
fn check(code:&str) -> u32 {
    let  digits = code.chars().filter(|&c|c.is_numeric()).map(|e|e.to_digit(10u32).unwrap());
    // get some 
    let pairs  = digits.enumerate().map(|(i,e)|(i+1,e));

    let sum = pairs.map(|(ind,n)| 
    {
 if ind%2!=0 { 
  n*1
 }
  else { 
      n*3
  }

    }
    
    ).reduce(|c,n|c+n).unwrap()*1;
    
  //let even = pairs.filter(|&c|c.0%2==0).map(|(_,n)|n).reduce(|c,n|c+n).unwrap()*3;
  let  result  =  10-(sum%10);

     match result { 
        1..=9 =>  result,
        _ => result%10
    }

}


fn main() {
    let now = Instant::now();
    let code = "10100010110100111011001100100110111101001110101010110011011011001000010101110010011101000100101";
     read_barcode(5, code);
  encode_barcode("5901234123457");
     
    println!("{:?}",now.elapsed())
}
