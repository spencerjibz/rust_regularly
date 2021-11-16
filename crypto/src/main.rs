
use itertools::Itertools;



fn polybius(s:&str) -> String { 
    let v:Vec<char> = ('A'..'Z').filter(|&e|e!='I').collect();

   let alpha:Vec<Vec<char>> = v.as_slice().chunks(5).map(|e|e.to_vec()).collect();

let  pairs:Vec<String>= alpha.into_iter().enumerate().map(|(ind,e)|{ 
 e.into_iter().enumerate().map(|(i,ch)| { 
     
     format!("{},{}{}",ch,ind+1,i+1)
    


} ).collect::<Vec<String>>()
}).flatten().collect();

//println!("{:?}",pairs);
    
s.chars().map(|e| {
 if e=='J'|| e =='I' { 
     "24".to_string()
 }
 else { 
      match pairs.clone().into_iter().find(|n|n.contains(e)) { 
          Some(ele) => ele.split(",").nth(1).unwrap().to_owned(),
          None => format!("{}",e)
      }
 }

    }).join("")
}
use std::time::Instant;
fn main() {
    let n = Instant::now();
     println!("{:?}",polybius("A"));
     println!("Executed in {:?}",n.elapsed())
   }