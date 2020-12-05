pub fn binary_search<T>(vector: &Vec<T>, entry: T) -> i32 
where T: PartialOrd {
    let size: usize = vector.len();
    
    let mut min: i32 = 0;
    let mut max: i32 = size as i32 - 1 ;
    while min <= max {
        let mid = (((min + max) as f32) / 2.0).floor() as i32;
        log::debug!("Values min: {}, max {}, mid {}, size: {}", min, max, mid, size);
        
        if vector[mid as usize] < entry {
            min = mid + 1;
        } else if vector[mid as usize] > entry {
            max = mid - 1;
        } else {
            return mid;
        }
    }

    return -1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_search_test() {
        let empty: Vec<i32> = vec![];
        let single = vec![1];
        let multiple = vec![1,2,3,4,5,6,7,8,9,10];

        // Tests
        assert_eq!(-1, binary_search(&empty, 0));
        assert_eq!(0, binary_search(&single, 1));
        assert_eq!(-1, binary_search(&single, 2));
        
        for entry in multiple.iter() {
            assert_eq!(entry - 1, binary_search(&multiple, *entry));
        }
    }
}