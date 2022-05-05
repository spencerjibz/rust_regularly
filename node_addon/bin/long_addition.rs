
pub fn long_addition(right:&str,left:&str) -> String { 

    // make two vectors of each length
     let [rightarr, leftarr] = [right.chars(),left.chars()];
    
     // find the longest,   
     let mut longest:Vec<_> = if right.len() > left.len() { rightarr.clone().collect() } else if right.len() == left.len() { rightarr.clone().collect()} else {leftarr.clone().collect()};   
    
     let mut shortest:Vec<_>= if right.len() < left.len() { rightarr.collect() } else if right.len() == left.len() { leftarr.collect()} else {leftarr.collect()};
    
    // difference between longest and shortest
     let diff = longest.len() - shortest.len();
    
     // make the short one the same  size as the longest;
      if longest.len() == shortest.len() {
          longest.insert(0,'0');
          shortest.insert(0,'0');
      } else { 
    
      for _ in 0..diff {
          shortest.insert(0,'0');
      }
    
      // start addition       
    }
    
    longest.reverse();
    shortest.reverse();
          
    let mut result:Vec<char> = longest.clone();
   
     let mut longest: Vec<u32> = longest.iter().map(|x|x.to_digit(10).unwrap()).collect();
    
     let  mut  shortest:Vec<u32>= shortest.iter().map(|x|x.to_digit(10).unwrap()).collect();
     
     //println!("{:?},{:?}",shortest,longest);
     let mut remainder =0;
     for ind in 0.. longest.len() { 
           let sum = (remainder + longest[ind] + shortest[ind]).to_string();
           remainder = if sum.len()>1 {sum.chars().nth(0).unwrap().to_digit(10).unwrap()} else {0 };
    
           if sum.len()>2 { 
               result[ind]= sum.chars().nth(1).unwrap()
           }
           else {  
             result[ind] =sum.chars().nth(0).unwrap();
           }
     }
       result.reverse();
    
       // if  the length of result < length of longest, prefill with first digits of longest
        if remainder>0 {
      remainder.to_string().chars().for_each(|c| result.insert(0,c));
        }
        // remove  useless useless zeros
         if result[0]=='0' { 
           result.remove(0);
         }
    
        result.iter().collect()
    }
    
    

     fn  main() { 
         let ans = long_addition("823094582094385190384102934810293481029348123094818923749817", "234758927345982475298347523984572983472398457293847594193837");
         println!("{},{}",ans,'g')
     }