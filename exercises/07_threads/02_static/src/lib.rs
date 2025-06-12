// TODO: Given a static slice of integers, split the slice into two halves and
//  sum each half in a separate thread.
//  Do not allocate any additional memory!
use std::thread;

pub fn sum(slice: &'static [i32]) -> i32 {
    
    if slice.is_empty() {
        return 0;
    }

    let mid = slice.len() / 2;
    let (left, right) = slice.split_at(mid);

    let left_handle = thread::spawn(move || {
        left.iter().sum::<i32>()
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
        },
        (Ok(left), Err(e)) => {
            eprintln!("Error in right thread: {:?}", e);
            0
        },
        (Err(e), Ok(right)) => {
            eprintln!("Error in left thread: {:?}", e);
            0
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        static ARRAY: [i32; 0] = [];
        assert_eq!(sum(&ARRAY), 0);
    }

    #[test]
    fn one() {
        static ARRAY: [i32; 1] = [1];
        assert_eq!(sum(&ARRAY), 1);
    }

    #[test]
    fn five() {
        static ARRAY: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(sum(&ARRAY), 15);
    }

    #[test]
    fn nine() {
        static ARRAY: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(sum(&ARRAY), 45);
    }

    #[test]
    fn ten() {
        static ARRAY: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(sum(&ARRAY), 55);
    }
}
