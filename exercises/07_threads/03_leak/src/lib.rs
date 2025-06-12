// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let leaked_slice = v.leak();
    let mid = leaked_slice.len() / 2;
    let (left, right) = leaked_slice.split_at(mid);

    let left_handle = thread::spawn(move || {
        left.iter().sum::<i32>() // i32 type annotated
    });

    let right_handle = thread::spawn(move || {
        right.iter().sum::<i32>()
    });

    let left_sum = left_handle.join();
    let right_sum = right_handle.join();

    match (left_sum, right_sum) {
        (Ok(left), Ok(right)) => left + right,
        (Err(e1), Err(e2)) => {
            eprintln!("Error in left thread: {:?}", e1);
            eprintln!("Error in right thread: {:?}", e2);
            0 // or handle the error as needed
        }
        (Ok(left), Err(e)) => {
            eprintln!("Error in right thread: {:?}", e);
            0
        }
        (Err(e), Ok(right)) => {
            eprintln!("Error in left thread: {:?}", e);
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
