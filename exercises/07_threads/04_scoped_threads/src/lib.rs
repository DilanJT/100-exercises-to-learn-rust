// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    
    let mid = v.len() / 2;
    let (left, right) = v.split_at(mid);

    thread::scope(|s| {
        let left_handle = s.spawn(|| {
            left.iter().sum::<i32>()
        });

        let right_hande = s.spawn(|| {
            right.iter().sum::<i32>()
        });

        match (left_handle.join(), right_hande.join()) {
            (Ok(left_handle), Ok(right_handle)) => left_handle + right_handle,
            (Err(_), Err(_)) => 0,
            (Ok(_), Err(_)) => 0,
            (Err(_), Ok(_)) => 0
        }
    })

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
