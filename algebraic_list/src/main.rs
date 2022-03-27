#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
#[allow(unused_variables)]
enum Cons<T: Clone + std::fmt::Debug> {
    Cons(T, Box<Cons<T>>),
    Null,
}

impl<T: Clone + std::fmt::Debug> Cons<T> {
    pub fn new(head: T, tail: Self) -> Self {
        Cons::Cons(head, Box::new(tail))
    }

    // fuse two togerther of
    #[allow(dead_code)]
    #[allow(unused_variables)]
    #[inline]
    pub fn fuse(&self, next: Cons<T>) -> Self {
        // just

        match &self {
            &Cons::Null => next,
            &Cons::Cons(head, ref tail) => Cons::new(head.clone(), tail.fuse(next)),
        }
    }
    #[allow(dead_code)]
    #[allow(unused_variables)]
    #[inline]
    pub fn to_vec(&self) -> Vec<T> {
        match self {
            &Cons::Null => vec![],
            &Cons::Cons(ref head, ref tail) => {
                let mut head = vec![head.clone()];
                head.extend(tail.to_vec());
                head
            }
        }
    }
    #[allow(dead_code)]
    #[allow(unused_variables)]
    #[inline]
    pub fn from_iter<I>(it: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        //TODO: provide a convenient method to convert any iterable
        //      to an algebraic list.
        //get the length of the list
        // create a vect with const
        // split iter into equal chunks
        let mut n = it.into_iter();
        // TODO split the into 2 chunks to them
        // let chunks=  n.chunks(n.len()/2).to_owned();

        match n.next() {
            None => Self::Null,
            Some(x) => Self::new(x, Self::from_iter(n)),
        }
    }
    #[allow(dead_code)]
    #[allow(unused_variables)]
    #[inline]
    pub fn map<F, S>(&self, fun: F) -> Cons<S>
    where
        F: Fn(T) -> S,
        S: Clone + std::fmt::Debug,
    {
        //TODO: return a new algebraic list containing all elements
        //      resulting from applying the mapper function to them.
        match self {
            &Cons::Null => Cons::Null,
            &Cons::Cons(ref head, ref tail) => Cons::new(fun(head.clone()), tail.map(fun)),
        }
    }

    #[allow(dead_code)]
    #[allow(unused_variables)]
    #[inline]
    pub fn filter<F>(&self, fun: F) -> Self
    where
        F: Fn(&T) -> bool,
    {
        //TODO: return a new algebraic list containing only elements
        //      that satisfy the predicate function.

        match self {
            &Cons::Null => Cons::Null,
            &Cons::Cons(ref head, ref tail) => match fun(head) {
                true => Cons::new(head.clone(), tail.filter(fun)),
                false => tail.filter(fun),
            },
        }
    }
}

fn run() {
    //let now = std::time::Instant::now();

    let numbers = Cons::from_iter(1..100000).filter(|&n| n > 8);

    println!("{:?}", numbers);

    //println!("{:?}",now.elapsed())
}

use std::thread;

const STACK_SIZE: usize = 100 * 1024 * 1024;

fn main() {
    // Spawn thread with explicit stack size
    let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(run)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_from_vec() {
        assert_eq!(Cons::from_iter(Vec::<i32>::new()), Cons::Null);

        assert_eq!(
            Cons::from_iter(vec![1, 2, 3, 4, 5]).to_vec(),
            vec![1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn should_filter() {
        assert_eq!(
            Cons::from_iter(vec![1, 2, 3, 4, 5])
                .filter(|&n| n > 3)
                .to_vec(),
            vec![4, 5]
        );

        assert_eq!(
            Cons::from_iter(vec![1, 2, 3, 4, 5]).filter(|&n| n > 5),
            Cons::Null
        );
    }

    #[test]
    fn should_map() {
        assert_eq!(
            Cons::from_iter(vec!["1", "2", "3", "4", "5"])
                .map(str::parse::<i32>)
                .map(Result::unwrap)
                .to_vec(),
            vec![1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn should_filter_map() {
        assert_eq!(
            Cons::from_iter(vec![1, 2, 3, 4, 5])
                .filter(|n| n % 2 == 0)
                .map(|x| x.to_string())
                .to_vec(),
            ["2", "4"]
        );
    }
}
