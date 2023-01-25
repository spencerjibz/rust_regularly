#![allow(dead_code, unused_variables)]
use std::time::Instant;

fn main() {
    let now = Instant::now();
    println!("{:?}", sum_intervals(&[(1, 5), (10, 15), (-1, 3)]));
    println!("{:?}",now.elapsed());
}



 fn sum_intervals(intervals: &[(i32, i32)]) -> i32 {
   let mut v = intervals.to_vec();
    v.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    let mut x = v[0].0;
    let mut res = 0;
     for i in v { 
        if i.1 > x {
            let f =  if i.0>x  {i.0} else {x};
            res += i.1 - f;
            x = i.1;
        
    
     }
    }
    res
}



//

#[cfg(test)]
mod fixed_tests {
    use super::sum_intervals;
    
    const ERR_MSG: &str = "\nYour result (left) did not match expected output (right) for this input:\n";

    #[test]
    fn non_overlapping_intervals() {
        let mut input = vec![(1,6)];
        assert_eq!(sum_intervals(&input), 5, "{}{:?}", ERR_MSG, &input);
        input = vec![(1, 4), (6, 10)];
        assert_eq!(sum_intervals(&input), 7, "{}{:?}", ERR_MSG, &input);
        input = vec![(11, 15), (6, 10), (1, 2)];
        assert_eq!(sum_intervals(&input), 9, "{}{:?}", ERR_MSG, &input);
    }
    
    #[test]
    fn overlapping_intervals() {
        let mut input = vec![(1,5), (1,5)];
        assert_eq!(sum_intervals(&input), 4, "{}{:?}", ERR_MSG, &input);
        input = vec![(1,5), (5,10)];
        assert_eq!(sum_intervals(&input), 9, "{}{:?}", ERR_MSG, &input);
        input = vec![(1,4), (3,6), (5,8), (7,10), (9,12)];
        assert_eq!(sum_intervals(&input), 11, "{}{:?}", ERR_MSG, &input);
        input = vec![(1,4), (7,10), (3,5)];
        assert_eq!(sum_intervals(&input), 7, "{}{:?}", ERR_MSG, &input);
        input = vec![(1,20), (2,19), (5,15), (8,12)];
        assert_eq!(sum_intervals(&input), 19, "{}{:?}", ERR_MSG, &input);
        input = vec![(1,5), (10, 20), (1, 6), (16, 19), (5, 11)];
        assert_eq!(sum_intervals(&input), 19, "{}{:?}", ERR_MSG, &input);
        input = vec![(2, 3), (2, 6), (2, 4), (2, 9), (2, 5)];
        assert_eq!(sum_intervals(&input), 7, "{}{:?}", ERR_MSG, &input);
    }
}

#[cfg(test)]
mod random_tests {
    use super::sum_intervals;
    use itertools::Itertools;
    use rand::{thread_rng, Rng};

    fn ref_solution(intervals: &[(i32, i32)]) -> i32 {
        let mut sum = 0;
        let mut end = i32::MIN;
        for (a, b) in intervals.iter().sorted() {
            if end < *a {
                end = *a;
            }
            if end < *b {
                sum += b - end;
                end = *b;
            }
        }
        sum
    }
    
    
    #[test]
    fn small_intervals() {
        let mut rng = thread_rng();
        for _ in 0..250 {
            let intervals: Vec<(i32, i32)> = (0..rng.gen_range(1..50))
                .map(|_| {
                    let start: i32 = rng.gen_range(-100..99);
                    let end: i32 = rng.gen_range((start + 1)..100);
                    (start, end)
                })
                .collect();
            
            assert_eq!(
                sum_intervals(&intervals),
                ref_solution(&intervals),
                "\nYour result (left) did not match the expected output (right) for this input:\n{:?}",
                &intervals,
            );
        }
    }
    
    #[test]
    fn large_intervals() {
        let mut rng = thread_rng();
        for _ in 0..100 {
            let intervals: Vec<(i32, i32)> = (0..rng.gen_range(1000..10000))
                .map(|_| {
                    let start: i32 = rng.gen_range(-100000..99999);
                    let end: i32 = rng.gen_range((start + 1)..100000);
                    (start, end)
                })
                .collect();
            
            assert_eq!(
                sum_intervals(&intervals),
                ref_solution(&intervals),
                "\nYour result (left) did not match the expected output (right) for this input:\n{:?}",
                &intervals,
            );
        }
    }
}
