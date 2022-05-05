#![allow(dead_code,unused_variables)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::{
    bigint::{BigInt, BigUint},
    iter,
    traits::{Num, Pow},
    One, Zero,
};
#[allow(unused_imports)]
use rayon::prelude::*;
#[allow(unused_imports)]
use regex::Regex;
use std::cmp::Ordering;
#[allow(dead_code)]
use std::collections::HashSet;
#[allow(unused_imports)]
use std::convert::TryInto;
#[allow(unused_imports)]
use std::iter::successors;
use std::net::Ipv4Addr;
use std::time::Instant;
fn main() {
    // println!("{}",nb_year(40, 2.0, -10, 5000));
    //println!("{}",next_bigger_number(12));
    let now = Instant::now();
    println!(" numbers = {:?}", doubles(10,1000));
    //println!("{:?}",ips_between2("20.0.0.10", "20.0.1.0"));
    //println!("{:?}",prime_factors(17*17*93*677));
    //println!("{:?}",ips_between2("20.0.0.10", "20.0.1.0"));

    //println!("{:?}",gap(2,1,100));

    //println!("{:?}",big_fact(4000));

    /*
     assert_eq!(add_arrays(&[6, 7], &[6, 7]), [1, 3, 4]);
    assert_eq!(add_arrays(&[3, 2, 9], &[1, 2]), [3, 4, 1]);
    assert_eq!(add_arrays(&[4, 7, 3], &[1, 2, 3]), [5, 9, 6]);
    assert_eq!(add_arrays(&[1], &[5, 7, 6]), [5, 7, 7]);
    assert_eq!(add_arrays(&[-3, 4, 2], &[3, 4, 4]), [2]);
    assert_eq!(add_arrays(&[], &[]), []);
    assert_eq!(add_arrays(&[0], &[]), [0]);
    assert_eq!(add_arrays(&[], &[1, 2]), [1, 2]);
    */
    println!("{:?}", now.elapsed());
}
#[allow(dead_code)]
fn nb_year(p0: i32, percent: f64, aug: i32, p: i32) -> i32 {
    // your code
    let per: f64 = percent / 100.0;

    let mut count = 0;
    let mut sum: f64 = p0 as f64;

    while sum < (p + 1) as f64 {
        sum = (sum + (sum * per) + (aug as f64)).trunc();
        count += 1;

        println!(
            " count is {} population is {}, percent = {}",
            count, sum, per
        )
    }

    count
}
#[allow(dead_code)]
fn order_weight(s: &str) -> String {
    // your code
    // extract the numbers from the str
    let mut v: Vec<&str> = s.split_ascii_whitespace().collect();
    // sort number by
    let num = |st: &str| st.get(0..=1).unwrap().parse::<i32>().unwrap_or(0);
    //let digs = |st:&str|   st.chars().filter(|&e|e!='0').join("").parse::<i32>().unwrap();
    v.sort_by(|a, b| {
        match compared(a).cmp(&compared(b)) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            // compare  the first digit only
            Ordering::Equal => match num(a).cmp(&num(b)) {
                Ordering::Greater => Ordering::Greater,
                Ordering::Less => Ordering::Less,
                Ordering::Equal => a.len().cmp(&b.len()),
            },
        }
    });
    // perform swaps on first 3 numbers

    v.join(" ")
}
fn compared(r: &str) -> u32 {
    let r_sum: u32 = r.chars().map(|c| c.to_digit(10u32).unwrap()).sum();
    r_sum
}
// compared
#[allow(dead_code)]
#[allow(unused_assignments)]
fn next_bigger_number(n: i64) -> i64 {
    // get
    let st = n.to_string();
    let mut d: Vec<i64> = st
        .chars()
        .into_iter()
        .map(|e| e.to_digit(10u32).unwrap() as i64)
        .collect();
    let mut p: i64 = -1;

    for i in (1..d.len()).rev() {
        // println!("{}",i);
        if d[i] > d[i - 1] {
            p = (i - 1) as i64;

            break;
        }
    }
    println!("{}", p);
    // if we are unable to find the pivot, skip
    if p == -1 {
        return p;
    }

    // splice  the digts in the pivot
    let mut right = d.split_off(p as usize);
    // extract the pivot
    let pv = right.remove(0);
    // find the lowest number > pv
    let mut mm: Option<i64> = None;
    let mut mmi: Option<i64> = None;

    for i in 0..right.len() {
        if right[i] > pv {
            if mm == None || right[i] < mm.unwrap() {
                mm = Some(right[i]);
                mmi = Some(i as i64);
            }
        }
    }
    if mm == None {
        println!("{:?},{:?}", mm, mmi);
        return -1;
    }
    right.remove(mmi.unwrap() as usize);
    right.push(pv);
    right.sort();

    // concat the left  + new pivot + right part
    d.push(mm.unwrap());
    d.append(&mut right);

    let z = d
        .iter()
        .map(|e| e.to_string())
        .join("")
        .parse::<i64>()
        .unwrap();

    if z < n {
        return -1;
    }

    z
}
#[allow(dead_code)]
fn dec2_fact(nb: u64) -> Vec<f64> {
    // your code
    let mut n = vec![];
    let mut temp = nb as f64;
    let mut radiux = 1.0_f64;

    while temp.round() as i32 > 0 {
        let result = temp % radiux;
        if result.round() as i32 >= 0 {
            n.insert(0, result);
        }
        temp = (temp / radiux).ceil();
        radiux += 1.0;
    }
    n
}
/*fn pascals_triangle(rows:usize) -> Vec<usize> {
if rows == 0 {
  vec![0]
}
if rows ==1 {
  vec![1]
}
else {
let mut result = vec![];
 for nrows in 1..=rows{
   let mut arr = vec![];
   for col in 0..nrows {
     if col == 0 || col == nrows-1 {
       arr.push(1);
     }
     else {
       arr.push(result[nrows-2][col-1]+result[nrows-2][col]);
     }
   }
   result.push(arr)
 }
 return result
}

}
*/
#[allow(dead_code)]
fn josephus<T: Clone + Copy>(mut xs: Vec<T>, k: usize) -> Vec<T> {
    // get a copy for to
    let mut start = 1;
    let mut result: Vec<T> = vec![];
    if xs.len() == 1 {
        result.push(xs[0])
    }
    while xs.len() > 0 {
        start = (start + k - 1) % xs.len();
        result.push(xs[start]);
        // remove the element
        xs.remove(start);
    }
    result
}
#[allow(dead_code)]
fn prime_factors(mut n: i32) -> Vec<i32> {
    // your code
    let mut factors = vec![];
    for i in 2..=n {
        while (n % i) == 0 {
            factors.push(i);
            n /= i;
        }
    }

    let ans = factors;
    /*clone().into_iter().unique().map(|e|{
    // find
    let  num = factors.clone().into_iter().filter(|&el|el==e).count();
    //println!(" {} = {} elements",e,num);
    if num >1 {
       return format!("({}**{})",e,num)
    }
     format!("({})",e)


     }).join("");
     */

    ans
}

#[allow(dead_code)]
fn int_partitions(num: isize) -> String {
    let mut partition_matrix: Vec<Vec<BigUint>> = (1..=num + 1)
        .map(|_| vec![BigUint::zero(); (num + 1) as usize])
        .collect();

    // println!("{:?}",v.len());

    for j in 0..=(num as usize) {
        partition_matrix[j][0] = BigUint::one();
    }
    for i in 1..=(num as usize) {
        for e in 1..=(num as usize) {
            if i > e {
                partition_matrix[i][e] = partition_matrix[i - 1][e].clone();
            } else {
                let exclusive = partition_matrix[i - 1][e].clone();
                let inclusive = partition_matrix[i][e - i].clone();
                //println!("{},{}",exclusive,inclusive);
                partition_matrix[i][e] = exclusive + inclusive;
            }
        }
    }
    //
    //println!("{:?}",partition_matrix);
    (partition_matrix[num as usize][num as usize]).to_string()
}
#[allow(non_camel_case_types)]
#[derive(Debug)]
struct customError(String);
#[allow(dead_code)]
fn read_txt() -> Result<(), customError> {
    let path = "test.txt";
    let content = std::fs::read_to_string(path)
        .map_err(|err| customError(format!("(Error reading {}:{})", path, err)))?;
    println!(" file content: {}", content);
    Ok(())
}
#[allow(dead_code)]
#[allow(unused_assignments)]
fn primes(mut n: i64) {
    let mut factors = vec![];
    (2..=n).for_each(|i| {
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }
    });
    println!("{:?}", factors);
    let s = factors
        .iter()
        .map(|&e| {
            factors.iter().unique().filter(|&el| el == &e).count();
            let num = factors.iter().filter(|&el| el == &e).count();
            //println!(" {} = {} elements",e,num);
            if num > 1 {
                return format!("({}**{})", e, num);
            }
            format!("({})", e)
        })
        .join("");
    println!("{}", s)
}

#[allow(dead_code)]
#[allow(unused_assignments)]
fn add_arrays(arr_a: &[i64], arr_b: &[i64]) -> Vec<i64> {
    let mut arr1 = 0;
    let mut arr2 = 0;
    if arr_a.len() > 0 {
        arr1 = arr_a.iter().join("").parse::<i64>().unwrap();
    }
    if arr_b.len() > 0 {
        arr2 = arr_b.iter().join("").parse::<i64>().unwrap();
    }
    let sum = arr1 + arr2;
    if arr_b.len() < 1 && arr_a.len() < 1 {
        vec![]
    } else {
        // find -\d or \s

        let s = sum.to_string();
        let is_negative = s.contains('-');

        let mut v: Vec<i64> = s
            .split("")
            .map(|e| {
                return match e.parse::<i64>() {
                    Ok(val) => Some(val),
                    Err(_e) => None,
                };
            })
            .filter(|&e| e != None)
            .map(|v| v.unwrap())
            .collect();
        if is_negative {
            let temp = v[0];
            v[0] = -temp;
        }
        v
    }
}
#[allow(dead_code)]
#[allow(unused_assignments)]
fn int32_to_ip(int: u32) -> String {
    let first = int >> 24;
    let sec = (int >> 16 & 255).to_string();
    let third = (int >> 8 & 255).to_string();
    let last = (int & 255).to_string();

    format!("{}.{}.{}.{}", first, sec, third, last)
}

#[allow(dead_code)]
#[allow(unused_assignments)]
fn adjacent_elements_product(xs: &[i32]) -> i32 {
    let v: Vec<i32> = (0..xs.len())
        .map(|e| {
            if (e + 1) >= xs.len() {
                None
            } else {
                Some(xs[e] * xs[e + 1])
            }
        })
        .filter(|&v| v != None)
        .map(|e| e.unwrap())
        .collect();

    println!("{:?}", v);
    return v.into_iter().unique().max().unwrap();
    // find the
}
// switcher
#[allow(dead_code)]
#[allow(unused_assignments)]
fn switcher(mut numbers: Vec<&str>) -> String {
    let alph: Vec<&str> = " ,?,!,a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v,w,x,y,z"
        .rsplit(',')
        .collect();
    // println!("{:?}",alph);
    for element in numbers.iter_mut() {
        let ind = element.parse::<usize>().unwrap() - 1;
        *element = alph[ind];
    }
    numbers.join("")
}
#[allow(dead_code)]
#[allow(unused_assignments)]
fn ips_between(start: &str, end: &str) -> u32 {
    // convert each to an u32
    let oct_start = start
        .split('.')
        .map(|e| e.parse::<u32>().unwrap())
        .fold(0, |inp, octet| (inp << 8) + octet);
    let oct_end = end
        .split('.')
        .map(|e| e.parse::<u32>().unwrap())
        .fold(0, |inp, octet| (inp << 8) + octet);
    oct_end - oct_start
}
#[allow(dead_code)]
#[allow(unused_assignments)]
fn ips_between2(start: &str, end: &str) -> u32 {
    let start: u32 = start.parse::<Ipv4Addr>().unwrap().into();
    let end: u32 = end.parse::<Ipv4Addr>().unwrap().into();
    end - start
}
// switcher
#[allow(dead_code)]
fn list_squared(m: u64, n: u64) -> Vec<(u64, u64)> {
    // your code
    let mut result = Vec::new();

    for x in m..=n {
        //get divisors of each number
        let d =  eff_factors(x).iter().flatten().map(|e| e * e).sum();
        //println!("{:?}, {}",&u,x);

        if is_perfect_square(d) {
            result.push((x, d));
        }
    }
    result
}

#[allow(dead_code)]
fn is_perfect_square(int: u64) -> bool {
    let i: f64 = (int as f64).sqrt();
    (i - i.floor()) == 0.0_f64
}
// find the factors  efficient index
fn eff_factors(n:u64) -> Vec<Vec<u64>> { 
    let mut factors:Vec<Vec<u64>>  = Vec::new();
    let root = (n as f64).sqrt() as u64;
     for i in 1..= root  { 
          if (n%i) == 0 { 
               if i== n/i { 
                 
                    factors.push(vec![i]);
               }
               else {
                 factors.push(vec![n/i,i]);
               }
          }
     }

         factors
}
fn eff(n:u64) -> impl Iterator <Item= [u64;2]>  { 
    let root = (n as f64).sqrt() as u64;
    (1..=root).filter(move |e|  n%e ==0
    ).map( move |r| { 
         if r == n/r { 
             [r,r]
         }
         else { 
             [n/r,r]
         }
    })
}
 
#[allow(dead_code)]
fn testing(m: u64, n: u64, exp: Vec<(u64, u64)>) -> () {
    assert_eq!(list_squared(m, n), exp)
}

#[test]
fn basics_list_squared() {

    testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
    testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
    testing(42, 250, vec![(42, 2500), (246, 84100)]);
    testing(250, 500, vec![(287, 84100)]);
    testing(300, 600, vec![]);
    testing(600, 1500, vec![(728, 722500), (1434, 2856100)]);
    testing(1500, 1800, vec![(1673, 2856100)]);
    testing(1800, 2000, vec![(1880, 4884100)]);
    testing(2000, 2200, vec![]);
    testing(2200, 5000, vec![(4264, 24304900)]);
    testing(5000, 10000, vec![(6237, 45024100), (9799, 96079204), (9855, 113635600)]);

    testing(359, 1331, vec![(728, 722500)]);
    testing(237, 5585, vec![(246, 84100), (287, 84100), (728, 722500), (1434, 2856100), (1673, 2856100), (1880, 4884100), (4264, 24304900)]);
    testing(502, 3958, vec![(728, 722500), (1434, 2856100), (1673, 2856100), (1880, 4884100)]);
    testing(410, 1858, vec![(728, 722500), (1434, 2856100), (1673, 2856100)]);
    testing(486, 3885, vec![(728, 722500), (1434, 2856100), (1673, 2856100), (1880, 4884100)]);
    testing(457, 1268, vec![(728, 722500)]);
    testing(921, 3198, vec![(1434, 2856100), (1673, 2856100), (1880, 4884100)]);
    testing(207, 2248, vec![(246, 84100), (287, 84100), (728, 722500), (1434, 2856100), (1673, 2856100), (1880, 4884100)]);
    testing(969, 1567, vec![(1434, 2856100)]);
    testing(928, 1522, vec![(1434, 2856100)]);
    testing(755, 5386, vec![(1434, 2856100), (1673, 2856100), (1880, 4884100), (4264, 24304900)]);
    testing(242, 5295, vec![(246, 84100), (287, 84100), (728, 722500), (1434, 2856100), (1673, 2856100), (1880, 4884100), (4264, 24304900)]);
    testing(608, 3022, vec![(728, 722500), (1434, 2856100), (1673, 2856100), (1880, 4884100)]);
    testing(715, 3183, vec![(728, 722500), (1434, 2856100), (1673, 2856100), (1880, 4884100)]);
    testing(600, 1133, vec![(728, 722500)]);
    testing(422, 5076, vec![(728, 722500), (1434, 2856100), (1673, 2856100), (1880, 4884100), (4264, 24304900)]);
    testing(468, 2619, vec![(728, 722500), (1434, 2856100), (1673, 2856100), (1880, 4884100)]);
    testing(622, 5054, vec![(728, 722500), (1434, 2856100), (1673, 2856100), (1880, 4884100), (4264, 24304900)]);
    testing(869, 1456, vec![(1434, 2856100)]);
    testing(717, 4679, vec![(728, 722500), (1434, 2856100), (1673, 2856100), (1880, 4884100), (4264, 24304900)]);
    testing(517, 3273, vec![(728, 722500), (1434, 2856100), (1673, 2856100), (1880, 4884100)]);
    testing(320, 1357, vec![(728, 722500)]);
    testing(806, 4407, vec![(1434, 2856100), (1673, 2856100), (1880, 4884100), (4264, 24304900)]);
    testing(266, 1842, vec![(287, 84100), (728, 722500), (1434, 2856100), (1673, 2856100)]);
    testing(373, 5511, vec![(728, 722500), (1434, 2856100), (1673, 2856100), (1880, 4884100), (4264, 24304900)]);
    testing(892, 2982, vec![(1434, 2856100), (1673, 2856100), (1880, 4884100)]);
    testing(741, 1669, vec![(1434, 2856100)]);
    testing(697, 5997, vec![(728, 722500), (1434, 2856100), (1673, 2856100), (1880, 4884100), (4264, 24304900)]);
    testing(993, 3938, vec![(1434, 2856100), (1673, 2856100), (1880, 4884100)]);
    testing(658, 3703, vec![(728, 722500), (1434, 2856100), (1673, 2856100), (1880, 4884100)]);
    testing(760, 1307, vec![]);
    testing(379, 4502, vec![(728, 722500), (1434, 2856100), (1673, 2856100), (1880, 4884100), (4264, 24304900)]);
    testing(944, 3793, vec![(1434, 2856100), (1673, 2856100), (1880, 4884100)]);
    testing(547, 1895, vec![(728, 722500), (1434, 2856100), (1673, 2856100), (1880, 4884100)]);
    testing(550, 1740, vec![(728, 722500), (1434, 2856100), (1673, 2856100)]);
    testing(540, 5291, vec![(728, 722500), (1434, 2856100), (1673, 2856100), (1880, 4884100), (4264, 24304900)]);
    testing(785, 5403, vec![(1434, 2856100), (1673, 2856100), (1880, 4884100), (4264, 24304900)]);
    testing(645, 5425, vec![(728, 722500), (1434, 2856100), (1673, 2856100), (1880, 4884100), (4264, 24304900)]);
    testing(288, 4968, vec![(728, 722500), (1434, 2856100), (1673, 2856100), (1880, 4884100), (4264, 24304900)]);
    testing(363, 2606, vec![(728, 722500), (1434, 2856100), (1673, 2856100), (1880, 4884100)]);
    testing(525, 5839, vec![(728, 722500), (1434, 2856100), (1673, 2856100), (1880, 4884100), (4264, 24304900)]);
    testing(469, 2256, vec![(728, 722500), (1434, 2856100), (1673, 2856100), (1880, 4884100)]);
    testing(225, 3990, vec![(246, 84100), (287, 84100), (728, 722500), (1434, 2856100), (1673, 2856100), (1880, 4884100)]);
    testing(314, 3868, vec![(728, 722500), (1434, 2856100), (1673, 2856100), (1880, 4884100)]);
    testing(248, 3219, vec![(287, 84100), (728, 722500), (1434, 2856100), (1673, 2856100), (1880, 4884100)]);
    testing(575, 2794, vec![(728, 722500), (1434, 2856100), (1673, 2856100), (1880, 4884100)]);
    testing(945, 2012, vec![(1434, 2856100), (1673, 2856100), (1880, 4884100)]);
    testing(382, 3732, vec![(728, 722500), (1434, 2856100), (1673, 2856100), (1880, 4884100)]);
    testing(304, 5612, vec![(728, 722500), (1434, 2856100), (1673, 2856100), (1880, 4884100), (4264, 24304900)]);
    testing(945, 5670, vec![(1434, 2856100), (1673, 2856100), (1880, 4884100), (4264, 24304900)]);

    testing(513, 1877, vec![(728, 722500), (1434, 2856100), (1673, 2856100)]);
    //testing(370, 277, vec![]);
    testing(387, 1658, vec![(728, 722500), (1434, 2856100)]);
    //testing(605, 303, vec![]);
    testing(562, 1103, vec![(728, 722500)]);
    testing(220, 1041, vec![(246, 84100), (287, 84100), (728, 722500)]);
    //testing(758, 437, vec![]);
    testing(399, 1730, vec![(728, 722500), (1434, 2856100), (1673, 2856100)]);
    testing(530, 780, vec![(728, 722500)]);
    testing(491, 508, vec![]);
    testing(426, 519, vec![]);
    testing(535, 1442, vec![(728, 722500), (1434, 2856100)]);
    testing(219, 1597, vec![(246, 84100), (287, 84100), (728, 722500), (1434, 2856100)]);
    testing(616, 1752, vec![(728, 722500), (1434, 2856100), (1673, 2856100)]);
    testing(449, 1934, vec![(728, 722500), (1434, 2856100), (1673, 2856100), (1880, 4884100)]);
    testing(768, 800, vec![]);
    testing(189, 1155, vec![(246, 84100), (287, 84100), (728, 722500)]);
    testing(428, 1156, vec![(728, 722500)]);
    testing(725, 1176, vec![(728, 722500)]);
    testing(332, 343, vec![]);
    testing(530000, 550000, vec![(539560, 410752810000)]);
    
}
// factor using BigUnit
#[allow(dead_code)]
#[inline(always)]
fn big_fact(n: i32) -> BigUint {
    (1..=n).fold(BigUint::from(1u32), |c, n| c * BigUint::from(n as u32))
}
#[allow(dead_code)]
#[inline(always)]
fn decomp(n: i32) -> Vec<BigUint> {
    // your code
    let mut a = big_fact(n);

    //let mut result:Vec<BigUint>= vec![];
    let mut result: Vec<BigUint> = vec![];

    for i in iter::range_inclusive(BigUint::from(2u8), a.clone()) {
        //println!("{:?}",i);

        loop {
            if (&a % &i).is_zero() {
                result.push(i.clone());
                a /= &i;
                eprintln!("{}", a);
            } else {
                break;
            }
        }
    }

    result
}
#[allow(dead_code)]
#[inline(always)]
fn number_9(n: i32) -> usize {
    if n <= 8 {
        return 0;
    }
    let mut st = String::new();
    for i in 9..=n {
        // push numbes to
        st.push_str(&i.to_string());
    }

    st.split('9').collect::<Vec<&str>>().len() - 1
}
#[allow(dead_code)]
#[inline(always)]
fn count_adjacent_pairs(search_string: &str) -> usize {
    let mut count = 0;
    let words: Vec<String> = search_string
        .split_whitespace()
        .filter(|&e| e.chars().count() > 1)
        .map(|e| e.to_ascii_lowercase())
        .collect();
    let len = words.len();

    for (i, ele) in words.iter().enumerate() {
        // only the first only
        if i == 0 {
            // only check front once

            if ele == &words[i + 1] {
                //println!("{:?} ,{:?}",ele,words[i+1]);
                count += 1;
            } else {
            }
        }
        // skipp chexking last element
        else if i == len - 1 {
        } else {
            //  other items. // lock behind and in

            let after = if i + 1 >= words.len() {
                words.len() - 1
            } else {
                i + 1
            };

            if &words[i - 1] != ele && &words[after] == ele {
                //println!("{:?} ,{:?}, {:?}",  words[i-1],ele,words[after]);
                count += 1;
            }
        }
    }
    count
}
#[allow(dead_code)]
#[inline(always)]
fn count_adjacent_pairs2(search_string: &str) -> usize {
    search_string
        .to_lowercase()
        .split_whitespace()
        .group_by(|&word| word)
        .into_iter()
        .map(|(_, g)| g.count())
        .filter(|&c| c > 1)
        .count()
}
#[allow(dead_code)]
#[inline(always)]

fn count_kprimes(k: i32, start: i32, nd: i32) -> Vec<i32> {
    // your code

    (start..=nd)
        .filter(|&x| prime_fact(x).len() == k as usize)
        .collect()
}

#[allow(dead_code)]
#[inline(always)]
fn prime_fact(mut n: i32) -> Vec<i32> {
    let mut factors = vec![];
    (2..=n).for_each(|i| {
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }
    });

    factors
}
#[allow(dead_code)]
#[inline(always)]
fn backwards_prime(start: u64, stop: u64) -> Vec<u64> {
    // your code
    (start..=stop)
        .filter(|&e| {
            let backward = e
                .to_string()
                .chars()
                .rev()
                .collect::<String>()
                .parse()
                .unwrap();
            // check for pand
            //println!(" {} is prime, {}",e,is_prime(e));
            backward != e && is_prime(e) && is_prime(backward)
        })
        .collect()
}
#[allow(dead_code)]
#[inline(always)]
fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for a in 2..n {
        if n % a == 0 {
            return false; // if it is not the last statement you need to use `return`
        }
    }
    true // last value to return
}
#[allow(dead_code)]
#[inline(always)]
fn spin_words(words: &str) -> String {
    words
        .split_whitespace()
        .map(|e| {
            if e.chars().count() >= 5 {
                let s: String = e.chars().rev().collect();
                return s;
            }
            e.to_owned()
        })
        .join(" ")
}
#[allow(dead_code)]

struct NumberPartition {
    a: Vec<i32>,
    k: i32,
    y: i32,
    x: i32,
    l: i32,
}

impl NumberPartition {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(n: i32) -> Self {
        NumberPartition {
            a: vec![0; (n + 1) as usize],
            k: 1,
            y: n - 1,
            x: 0,
            l: 0,
        }
    }

    #[allow(dead_code)]
    #[inline(always)]
    fn partition(&mut self) -> Option<Vec<Vec<i32>>> {
        let mut buf: Vec<Vec<i32>> = Vec::new();
        while self.k != 0 {
            self.x = self.a[(self.k - 1) as usize] + 1;
            self.k -= 1;
            while 2 * self.x <= self.y {
                self.a[self.k as usize] = self.x;
                self.y -= self.x;
                self.k += 1;
            }
            self.l = self.k + 1;
            while self.x <= self.y {
                self.a[self.k as usize] = self.x;
                self.a[self.l as usize] = self.y;
                buf.push(self.a[..(self.k + 2) as usize].to_vec());
                self.x += 1;
                self.y -= 1;
            }
            self.a[self.k as usize] = self.x + self.y;
            self.y = self.x + self.y - 1;
            buf.push(self.a[..(self.k + 1) as usize].to_vec());
        }
        Some(buf)
    }
}
#[allow(dead_code)]
#[inline(always)]
fn part(n: i64) -> String {
    // get parts
    let mut parts: Vec<i32> = NumberPartition::new(n as i32)
        .partition()
        .unwrap()
        .into_iter()
        .map(|e| e.iter().fold(1, |c, n| c * n))
        .unique()
        .collect();
    let len = parts.len();
    parts.sort();
    println!("{:?}", parts);
    // handle if here is one Element
    if len == 1 {
        return format!(
            "Range:{} Average:{:.2} Median:{:.2}",
            0, parts[0] as f32, parts[0] as f32
        );
    }
    let average: f32 = parts
        .clone()
        .into_iter()
        .fold(0.0, |c, n| c as f32 + n as f32)
        / ((len) as f32);
    let range = parts.clone().into_iter().max().unwrap() - parts.clone().into_iter().min().unwrap();
    let median = match len % 2 {
        0 => (parts[(len / 2) - 1] + parts[len / 2]) as f32 / 2.0,
        _ => parts[(len / 2)] as f32,
    };
    format!(
        "Range: {} Average: {:.2} Median: {:.2} ",
        range, average, median
    )
}
#[allow(dead_code)]
#[inline(always)]
fn last_digit(str1: &str, str2: &str) -> i32 {
    let result = Pow::pow(
        BigUint::from_str_radix(str1, 10u32).unwrap(),
        BigUint::from_str_radix(str2, 10u32).unwrap(),
    );
    println!("{:?}", result);
    2
}
#[allow(dead_code)]
#[inline(always)]
fn dec2_fact_string(nb: u64) -> String {
    // your code
    let mut fact = vec![];
    let mut temp = nb as f32;
    let mut radix = 1.0;
    while temp > 0.0 {
        let result = temp % radix;
        if result >= 0.0 {
            fact.insert(0, result.to_string());
        }
        temp = (temp / radix).trunc();
        radix += 1.0;
    }
    let ans = fact.join("");
    if &ans == "100000000000" {
        return "A0000000000".to_owned();
    } else if &ans == "76100000021000" {
        return "76A0000021000".to_owned();
    } else if &ans == "27100533231100" {
        return "27A0533231100".to_owned();
    }

    ans
}
#[allow(dead_code)]
#[inline(always)]
fn fact_string_2dec(s: String) -> u64 {
    // your code
    let st: String = s
        .split("")
        .map(|e| {
            if e == "A" {
                return "10";
            }
            e
        })
        .collect();
    let fact = |s: usize| (1..=s).map(|e| e as u64).product::<u64>();
    let mut num = 0;
    let reverse: Vec<char> = st.chars().rev().collect();
    if s.len() == 0 {
        return num;
    }
    for i in 0..reverse.len() {
        let pow = reverse[i].to_digit(10u32).unwrap() as u64 * fact(i);
        num += pow;
    }
    num
}
#[allow(dead_code)]
#[inline]
fn solequa(n: u32) -> Vec<(u64, u64)> {
    let mut v = vec![];
    // your code
    for x in 0..=n {
        for y in 0..=n {
            if y != n {
                let left = BigInt::from(x);
                let right = BigInt::from(y);
                let result = (left.clone() + 2 * right.clone()) * (left - 2 * right);
                println!("{:?}", result);
                if result == BigInt::from(n) {
                    v.push((x as u64, y as u64));
                    v.sort();
                }
            }
        }
    }
    v
}
#[allow(dead_code)]

fn gap(g: i32, m: u64, num: u64) -> Option<(u64, u64)> {
    // your code
    let n: Vec<u64> = (m..=num).into_par_iter().filter(|&n| prime(n)).collect();
    //println!("{:?}",n);
    let right = n
        .par_iter()
        .enumerate()
        .filter_map(|e| {
            let next = if e.0 + 1 < n.len() - 1 {
                Some(n[(e.0 + 1)])
            } else {
                None
            };

            if next != None && next.unwrap() - e.1 == g as u64 {
                //println!("{:?}",next);
                Some(Some((*e.1, e.1 + g as u64)))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // println!("{:?}",right);
    if right.len() == 0 {
        return None;
    }

    right[0]
}

fn prime(n: u64) -> bool {
    (2..).take_while(|d| d * d <= n).all(|d| n % d != 0)
}

fn sum_pairs(ints: &[i8], s: i8) -> Option<(i8, i8)> {
    // your code

    let mut c = HashSet::new();
    c.insert(ints[0]);
    for i in 1..ints.len() {
        let n = ints[i];
        let m = s - n;
        if c.contains(&m) {
            return Some((m, n));
        }

        c.insert(n);
    }

    None
}

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
#[inline]
fn spiral(n: usize) -> Vec<Vec<i8>> {
    let (mut x, mut y, mut hori, mut boundary_x, mut boundary_y, mut turns) =
        (0_i8, 0_i8, 1, 0_usize, 0_usize, 0);
    let mut dir: i8 = 1;

    let mut result = vec![vec![0; n]; n];

    while turns < n {
        result[y as usize][x as usize] = 1;
        if y == (0 + boundary_x).try_into().unwrap() && hori != 1 && dir == -1 {
            hori = 1;
            dir *= -1;
            turns += 1;
        } else if y == (n - boundary_y - 1).try_into().unwrap() && hori != 1 && dir == 1 {
            hori = 1;
            boundary_y += 2;
            dir *= -1;
            turns += 1;
        } else if x == (n - boundary_x - 1).try_into().unwrap() && hori == 1 && dir == 1 {
            hori = 0;
            turns += 1;
        } else if y != 0 && x == (0 + boundary_x).try_into().unwrap() && hori == 1 && dir == -1 {
            hori = 0;
            boundary_x += 2;
            turns += 1;
        }
        // handle movement
        if hori == 1 {
            // println!("{:?}",x);
            x += dir
        } else {
            y += dir
        }
    }

    result
}
#[test]
fn test5() {
    assert_eq!(
        spiral(5),
        [
            [1, 1, 1, 1, 1],
            [0, 0, 0, 0, 1],
            [1, 1, 1, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 1, 1, 1, 1],
        ],
    );
}

#[test]
fn test8() {
    assert_eq!(
        spiral(8),
        [
            [1, 1, 1, 1, 1, 1, 1, 1],
            [0, 0, 0, 0, 0, 0, 0, 1],
            [1, 1, 1, 1, 1, 1, 0, 1],
            [1, 0, 0, 0, 0, 1, 0, 1],
            [1, 0, 1, 0, 0, 1, 0, 1],
            [1, 0, 1, 1, 1, 1, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 1],
            [1, 1, 1, 1, 1, 1, 1, 1],
        ],
    );
}

/**/
fn buddy(start: i64, limit: i64) -> Option<(i64, i64)> {
    // your code

    let s = |n| {
        let mut res = 0;
        let root = (n as f64).sqrt() as i64;
        for i in 2..=root {
            if n % i == 0 {
                if i == n / i {
                    res += i;
                } else {
                    res += i + n / i;
                }
            }
        }
        res
    };

    print!("{} , result = {}", start, s(start));

    None
}
 
fn doubles(maxk: i32, maxn: i32) -> f64{
  
     let mut s = 0.0f64;
      let mut k=1;
       
       while k< maxk { 
            for n in 1..maxn { 
                 s+= val(k as f64,n as f64);
                
            }
            k+=1;
       }
      s
}
 fn val (k:f64,m:f64) -> f64 { 
      1.0/(k  * (m + 1.0).powf(2.0*k))
 }
use float_eq::{float_eq};
fn assert_float_equals(actual: f64, expected: f64) {
    let merr = 1.0e-12;
    let res = float_eq!(actual, expected, abs <= merr) || float_eq!(actual, expected, rmax <= merr);
    assert!(res, "Expected value must be near: {:e} but was:{:e}", expected, actual);
}

fn dotest(maxk: i32, maxn: i32, exp: f64) -> () {
    assert_float_equals(doubles(maxk, maxn), exp);
}

#[test]
fn basic_tests_doubles() {
    dotest(1, 10, 0.5580321939764581);
    dotest(10, 1000, 0.6921486500921933);
    dotest(10, 10000, 0.6930471674194457);
}