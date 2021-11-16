use std::cmp::Ordering;
use itertools::Itertools;

fn main() {
    let now = std::time::Instant::now();
    let square = create_3n(15);
   println!("ans = {:?}",snail(&square));
    println!("{:?}",sum_of_divided(vec![107, 158, 204, 100, 118, 123, 126, 110, 116, 100]));
    println!("{:?}",convert_fracts(vec![(690, 1300), (87, 1310), (30, 40)]));
      //println!("{:?}",);//disemvowel("This website is for losers LOL!"));
     println!("{:?}",now.elapsed())

}

#[allow(dead_code)]
static  VOWEL:&str = "aeiou";

#[allow(dead_code)]#[inline(always)]
fn disemvowel(s: &str) -> String {
  let vowel = "aeiou";
    s.chars().filter(|&e|!vowel.contains(e.to_ascii_lowercase())).join("")
}
#[allow(dead_code)]
fn closest(s: &str) -> String {
    // your code
 let  strng:Vec<&str>= s.split_whitespace().collect();
 let  mut weights:Vec<(u32,u32)> = strng.iter().map(|e| {
   let r = e.chars().map(|c|c.to_digit(10u32).unwrap()).reduce(|cur,nex|cur+nex).unwrap();
  (r,e.parse::<u32>().unwrap())
 }
).collect();
 
weights.sort();
let   diffs:Vec<Option<(u32,u32,u32)>> = weights.iter().enumerate().map(|(ind,(n,e))|  { 
 let next = ind+1;
  if next >= weights.len()  { 
      return  None
  }
  else {
  Some( ((weights[next].0 - n),*n,*e))
  }
}


).filter(|&x|x!=None).collect();
 
 // weight
  //weights.sort_by(|a,b|(a.0 as i32 -b.0 as i32).cmp(&(b.0 as i32-a.0 as i32)));
  // weight 
  // print 
  //println!("{:?}",diffs);
  let fina: Vec<(u32,usize,u32)> = diffs.iter().take(1).map(|n| { 
// the positon of the orginal
   let (_,t,ans) = n.unwrap();
   // print t
   // 
 
let post = strng.iter().position(|&x| x==format!("{:?}",ans)).unwrap();

  (t,post,ans)
  }).collect();
   format!("{:?}",fina)
}

 #[inline(always)]

 #[allow(dead_code)]
 #[allow(unused_variables)]
fn sum_pairs(ints: &[i8], s: i8) -> Option<(i8, i8)> {

 let mut ind:Vec<(usize,usize)> = vec![];
  // your code
  // use half the length
 for  i in 0..ints.len() { 
   for x in 0..(ints.len()-1) { 
   if x!=i  && (ints[i]+ints[x])==s { 
     // push the indexes
       let t:(usize,usize) =  match i.cmp(&x)  { 
            Ordering::Less => (i,x),
            Ordering::Greater => (x,i),
            _ => (i,x)

       };
       // no duplicates
       if !ind.contains(&t) { 
        ind.push(t);
       }
        
   } 

   }
 }
 // find comp the last index
 ind.sort_unstable_by(|a,b|a.1.cmp(&b.1));

 
 //println!("{:?}",ind);

 if  ind.len()>0 { 
   Some((ints[ind[0].0],ints[ind[0].1]))
 }
  else { 
    None
  }
 
  
}
 
use std::collections::HashMap;
#[allow(dead_code)]
#[allow(unused_must_use)]
fn sum_pairs2(ints: &[i8], s: i8) -> Option<(i8, i8)> {
    // your code
    let mut pair: HashMap<i64, Option<(i8, i8)>> = HashMap::new();
    let mut c = 0_i64;
    (&ints)
        .into_iter()
        .filter(|&x| {
            c += 1_i64;
            let mut indice = 0_64;
            (&ints[c as usize..])
                .into_iter()
                .filter(|&y| {
                    indice += 1_i64;
                    match (*y) as i64  + (*x) as i64 == s as i64 {
                        true => {
                            pair.insert(indice + c, Some((*x, *y)));
                            return true;
                        }
                        _ => return false,
                    }
                })
                .collect::<Vec<_>>()
                .len() > 0usize
        })
        .collect::<Vec<_>>();
    match pair.len() > 0 {
        true => {
            let mut indices: Vec<i64> = pair.keys().into_iter().map(|&x| x).collect();
            indices.sort();
            let min_indice = &indices.first().unwrap();
            //println!("pair:{:?}", pair);
            //println!("indices:{:?} min_indice :{:?}", indices, min_indice);
            return *(pair.get(&min_indice).unwrap());
        }
        _ => return None,
    }
}

#[allow(dead_code)]
#[inline(always)]
fn sum_of_divided(l: Vec<i64>) -> Vec<(i64,i64)> {
  // your code
  let mut result:Vec<(i64,i64)> =   l.iter().flat_map(|e|prime_factors(e.abs())).unique().map(|e| { 
 let res = l.iter().filter(|&x|x%&e== 0).sum();
 (e,res)

   }).collect();
   result.sort();
   result
}

#[allow(dead_code)]

fn prime_factors( mut n: i64) -> Vec<i64> {
 
  // your code
  let mut factors = vec![];
for i in 2..=n {
  while (n%i)==0 {
factors.push(i);
n /=i;
  }
}

   factors
}
#[allow(dead_code,unused_variables)]
#[inline(always)]
fn dec2_fact_string(mut nb: u64) -> String {
let  mut base = 1;
let mut res = String::new();
let  base36:Vec<char> = "0123456789ABCDEFGHIGKLMNOPQRSTUWXYZ".chars().collect();
while nb > 0 { 
 res.insert(0,base36[(nb%base) as usize]);
   
 // increment base
  nb = nb/base; 
  base += 1;
 
}

res
  // your code
}
#[allow(dead_code,unused_variables)]
#[inline(always)]
fn fact_string_2dec(s: String) -> u64 {
  let mut  res = 0;
  let mut fact =1; 
  let  base36:Vec<char> = "0123456789ABCDEFGHIGKLMNOPQRSTUWXYZ".chars().collect();
   for i in (0..=s.len()-2).rev() { 
    fact *=s.len()-i-1;
  res += base36.iter().position(|&x| x== s.chars().nth(i).unwrap()).unwrap() * fact;
  
   }
  // your code
 res as u64
}

#[allow(dead_code,unused_variables)]
#[inline(always)]
fn solve(arr: Vec<i128>) -> (i128, i128) {
  // your code
  let num:i128 = arr.chunks(2).map(|e|e.to_vec().into_iter().reduce(|c,n|c*c+n*n).unwrap()).product();
  //println!("{}",num);
  
   let mut res:Vec<(i128,i128)> = vec![];
     for i in 0..num/10 { 
       for x in 0..num/10 { 
          let p = i*i + x*x;
            // println!("{:?}",p);
          if p == num { 
          
          match i<x { 
            true => { 
             
              res.push((i,x));
              
            },
            false => { 
              // do nothing
            }
          }
          }
       }
     }
   res[0]
}
#[allow(dead_code,unused_variables)]
#[inline(always)]
fn convert_fracts(l: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
  // your code

  let demon = l.iter().fold(1,|a,b| lcm(b.1,a));

    l.iter().map(|n|(n.0*demon/n.1,n.1*demon/n.1)).collect()
}
#[allow(dead_code,unused_variables)]
#[inline(always)]
fn gcd (a:i64,b:i64) -> i64 { 
 if b == 0 { 
  a
  }
  else {
gcd(b,a%b)
  }
}
#[allow(dead_code,unused_variables)]
#[inline(always)]
fn lcm(d1:i64,d2:i64) -> i64 { 
  d1*d2 /gcd(d1, d2)
}
/*
#[allow(dead_code,unused_variables,non_snake_case)]
fn choose_best_sum(t: i32, k: i32, ls: &Vec<i32>) -> i32 {
  // your code
  let mut biggestCount = 0;
  let recurseTowns = |townsofar:&Vec<i32>,lastIndex:usize|  { 
    if townsofar.len() ==k as usize  { 
      let sumDistance:i32 = townsofar.iter().sum();
      if sumDistance <=t && sumDistance > biggestCount { 
        biggestCount = sumDistance;
      }
    }
     
    for i in (lastIndex+1)..= ls.len() { 
      let mut v = townsofar.to_vec();
      v.push(ls[i]);
      recurseTowns(&v,i);
    }
  recurseTowns();
  return biggestCount 
  };
  
  1
}
*/

#[allow(dead_code)]
#[inline]
fn solequa(n: u64) -> Vec<(u64, u64)> {
  let mut v = vec![];
  // your code
  for x in 1..n {
      
       for y in 1..n {
            if y !=n { 
            let left = x as f64;
            let right = y as f64;
             let result = (left-2.0*right) * (left+2.0*right);
           //println!("{:?} {}",x,y);
           if result ==  (n as f64) {
               v.push((x,y))
           }
          }
      }
  }
    v.sort_by(|a,b|b.1.cmp(&a.1));
  v 
}
// using iterators
#[allow(dead_code)]
#[inline] 
fn solequa_two(n:u64) -> Vec<(u64, u64)> { 
  (1..=n).map(|x| {
    
    if let Some(y)= (1..n).filter(|&y| {
  
      let left = x as f64;
      let right = y as f64;
       let result = (left-2.0*right) * (left+2.0*right);
     //println!("{:?} {}",x,y);
     return  result ==  (n as f64) 
    
    
  


  }).nth(0) { 
    (x,y)
  } 
  else { 
    (0,0)
  }


} ).filter(|&e|e!=(0,0)).collect()
}

fn snail(matrix: &[Vec<i32>]) -> Vec<i32> {
  // enjoy

  let mut fina = vec![];
  let mut arr = matrix.to_vec();

  while  arr.len() >0 { 
 fina.append(&mut arr.remove(0));
    for i in 0..arr.len() { 
       if let Some (val) = arr[i].pop() { 
        fina.push(val);
       }
     
    }

    arr.reverse();
   
   for i in 0..arr.len() { 
     arr[i].reverse();
   }
   //println!(" arr = {:?}, final = {:?}",arr,fina);
  }
  fina
}

fn create_3n(n:i32) ->Vec<Vec<i32>> { 
    let arr:Vec<i32> = (1..=n).collect();
     arr.as_slice().chunks(3).map(|e|e.to_vec()).collect()

}

#[cfg(test)]

#[test]
fn returns_expected() {
    let l1 = [1, 4, 8, 7, 3, 15];
    let l2 = [1, -2, 3, 0, -6, 1];
    let l3 = [20, -13, 40];
    let l4 = [1, 2, 3, 4, 1, 0];
    let l5 = [10, 5, 2, 3, 7, 5];
    let l6 = [4, -2, 3, 3, 4];
    let l7 = [0, 2, 0];
    let l8 = [5, 9, 13, -3];
    assert_eq!(sum_pairs(&l1, 8), Some((1, 7)));
    assert_eq!(sum_pairs(&l2, -6), Some((0, -6)));
    assert_eq!(sum_pairs(&l3, -7), None);
    assert_eq!(sum_pairs(&l4, 2), Some((1, 1)));
    assert_eq!(sum_pairs(&l5, 10), Some((3, 7)));
    assert_eq!(sum_pairs(&l6, 8), Some((4, 4)));
    assert_eq!(sum_pairs(&l7, 0), Some((0, 0)));
    assert_eq!(sum_pairs(&l8, 10), Some((13, -3)));
}
#[allow(dead_code)]
fn testing(l: Vec<i64>, exp: Vec<(i64, i64)>) -> () {
  assert_eq!(sum_of_divided(l), exp)
}

#[test]
fn basics_sum_of_divided() {
  
  testing(vec![12, 15], vec![ (2, 12), (3, 27), (5, 15) ]);
  testing(vec![15,21,24,30,45], vec![ (2, 54), (3, 135), (5, 90), (7, 21) ]);
  testing(vec![15,21,24,30,-45], vec![ (2, 54), (3, 45), (5, 0), (7, 21) ]);
  testing(vec![107, 158, 204, 100, 118, 123, 126, 110, 116, 100], 
      vec![ (2, 1032), (3, 453), (5, 310), (7, 126), (11, 110), (17, 204), (29, 116), (41, 123), (59, 118), (79, 158), (107, 107) ]);
  testing(vec![-29804, -4209, -28265, -72769, -31744], 
      vec![ (2, -61548), (3, -4209), (5, -28265), (23, -4209), (31, -31744), (53, -72769), (61, -4209), (1373, -72769), (5653, -28265), (7451, -29804) ]);
  testing(vec![], vec![]);
  testing(vec![1070, 1580, 2040, 1000, 1180, 1230, 1260, 1100, 1160, 1000], 
      vec![ (2, 12620), (3, 4530), (5, 12620), (7, 1260), (11, 1100), (17, 2040), (29, 1160), (41, 1230), (59, 1180), (79, 1580), (107, 1070) ]);
  testing(vec![17, 34, 51, 68, 102], vec![ (2, 204), (3, 153), (17, 272) ]);
  testing(vec![17, -17, 51, -51], vec![ (3, 0), (17, 0) ]);
  testing(vec![173471], vec![ (41, 173471), (4231, 173471) ]);

  testing(vec![24,18,92,88,99], vec![ ( 2, 222 ), ( 3, 141 ), ( 11, 187 ), ( 23, 92 ) ]);
  testing(vec![26,54,65,70,63,93], vec![ ( 2, 150 ), ( 3, 210 ), ( 5, 135 ), ( 7, 133 ), ( 13, 91 ), ( 31, 93 ) ] ); 
  testing(vec![75,20,-23,86,21], vec![ ( 2, 106 ), ( 3, 96 ), ( 5, 95 ), ( 7, 21 ), ( 23, -23 ), ( 43, 86 ) ] );
  testing(vec![70,-44,90,76], vec![ ( 2, 192 ), ( 3, 90 ), ( 5, 160 ), ( 7, 70 ), ( 11, -44 ), ( 19, 76 ) ] ); 
  testing(vec![56,-49,53,56,-35,49,100], vec![ ( 2, 212 ), ( 5, 65 ), ( 7, 77 ), ( 53, 53 ) ]);

  testing(vec![-50,73,55,90,35,46], vec![ ( 2, 86 ), ( 3, 90 ), ( 5, 130 ), ( 7, 35 ), ( 11, 55 ), ( 23, 46 ), ( 73, 73 ) ] ); 
  testing(vec![93,96,-13,96,41,-18,19], vec![ ( 2, 174 ), ( 3, 267 ), ( 13, -13 ), ( 19, 19 ), ( 31, 93 ), ( 41, 41 ) ] ); 
  testing(vec![37,13,19,33,-18,79,26], vec![ ( 2, 8 ), ( 3, 15 ), ( 11, 33 ), ( 13, 39 ), ( 19, 19 ), ( 37, 37 ), ( 79, 79 ) ] );
  testing(vec![-10,22,86,33,-19,86,48], vec![ ( 2, 232 ), ( 3, 81 ), ( 5, -10 ), ( 11, 55 ), ( 19, -19 ), ( 43, 172 ) ] );
  testing(vec![42,20,43,-44], vec![ ( 2, 18 ), ( 3, 42 ), ( 5, 20 ), ( 7, 42 ), ( 11, -44 ), ( 43, 43 ) ] );
}
#[allow(dead_code)]
fn testing1(nb: u64, exp: &str) -> () {
  assert_eq!(&dec2_fact_string(nb), exp)
}
#[allow(dead_code)]
fn testing2(s: &str, exp: u64) -> () {
  assert_eq!(fact_string_2dec(s.to_string()), exp)
}
#[test]
fn basics_dec2_fact_string() {

    testing1(2982, "4041000");
    testing1(463, "341010");
    testing1(36288000, "A0000000000");
    testing1(3628800054, "76A0000021000");
    testing1(1273928000, "27A0533231100");
    testing1(220, "140200");
    testing1(1936295, "5301133210");
    testing1(81440635, "204365543010");
    testing1(14808485, "40721200210");
    testing1(3395, "4411210");
    testing1(92, "33100");
    testing1(4881, "6431110");
    testing1(174720, "424400000");
    testing1(5897, "11102210");
    testing1(1947, "2410110");
    testing1(1575088, "4303332200");
    testing1(49124, "115113100");
    testing1(9376317, "25742343110");
    testing1(449, "332210");
    testing1(112, "42200");
    testing1(64269, "145123110");
    testing1(6742089, "18515001110");
    testing1(86565208, "218474232200");
    testing1(1806792694, "3929024133200");
    testing1(12942219, "35576140110");
    
}
#[test]
fn basics_fact_string_2dec() {

    testing2("4041000", 2982);
    testing2("27A0533231100", 1273928000);
    testing2("341010", 463);
    testing2("65341010", 34303);
    testing2("1461330110", 555555);
    testing2("13573044440000", 7890123456);
    testing2("1630614043233100", 1849669781372);
    testing2("150636011110", 58322193);
    testing2("1662032340200", 741017980);
    testing2("194B99466413200", 145612691086);
    testing2("51465021000", 18702054);
    testing2("445212100", 185318);
    testing2("1000100", 722);
    testing2("522200", 664);
    testing2("2000", 12);
    testing2("64C0048313011110", 8269501168833);
    testing2("2455221000", 916134);
    testing2("10A739302433010", 92262000091);
    testing2("1A20254533200", 885536614);
    testing2("3855031110", 1440081);
    testing2("14D4831766411000", 1739585710590);
    testing2("331703501110", 131284689);
    testing2("86CB45050343200", 740991913678);
    testing2("112086032303100", 94394539820);
    testing2("30A3700211000", 1474663950);
    testing2("156B92750141010", 121660182223);
    
}

#[cfg(test)]
    mod tests {
    use super::*;

    fn calc(arr: &Vec<i128>) -> i128 {
        let mut p:i128 = 1;
        let mut i = 0;
        while i < (arr.len() - 1) {
            p *= arr[i] * arr[i] + arr[i+1] * arr[i+1];
            i = i + 2;
        }
        return p;
    }
    fn check(arr: &Vec<i128>, res: (i128, i128)) -> bool {
        if res.0 < 0 || res.1 < 0 {
            println!("A and B should be nonnegative integers");
            return false;
        } else {
            let p = res.0*res.0 + res.1*res.1;
            let pp = calc(arr);
            if p != pp {
                println!("Incorrect sum of squares");
                return false;
            }
            else {
                return true;
            }
        }
    }
    fn dotest(arr: &Vec<i128>) -> () {
        let ans = solve(arr.to_vec());
        let bl: bool = check(arr, ans);
        assert_eq!(bl, true, "Testing array {:?}", arr);
    }

    #[test]
    fn basic_tests() {
        let mut a = vec![1, 3, 1, 2, 1, 5, 1, 9];
        dotest(&a);
        a = vec![0, 7, 0, 0];
        dotest(&a);
        a = vec![2, 1, 3, 4];
        dotest(&a);
        a = vec![3, 9, 8, 4, 6, 8, 7, 8, 4, 8, 5, 6, 6, 4, 4, 5];
        dotest(&a);

    }
}


    #[test]
    fn sample_test1() {
        let square = &[
            vec![1,2,3],
            vec![4,5,6],
            vec![7,8,9],
        ];
        let expected = vec![1,2,3,6,9,8,7,4,5];
        assert_eq!(snail(square), expected);
    }
    
    #[test]
    fn sample_test2() {
        let square = &[
            vec![1,2,3],
            vec![8,9,4],
            vec![7,6,5],
        ];
        let expected = vec![1,2,3,4,5,6,7,8,9];
        assert_eq!(snail(square), expected);
    }
    
    #[test]
    fn sample_test3() {
        let square: &[Vec<i32>; 1] = &[Vec::new()];
        let expected = Vec::new();
        assert_eq!(snail(square), expected, "Failed with empty input");
    }
    
    #[test]
    fn sample_test4() {
        let square = &[vec![1]];
        let expected = vec![1];
        assert_eq!(snail(square), expected);
    }

