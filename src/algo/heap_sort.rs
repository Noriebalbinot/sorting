pub fn heap_sort(mut nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    for i in (0..=len / 2).rev() {
        reorganyze_heap(&mut nums, i, len);
    }

    for i in (0..len).rev() {
        println!("swap {} and {}", i, 0);
        nums[i] = nums[0] ^ nums[i];
        nums[0] = nums[i] ^ nums[0];
        nums[i] = nums[i] ^ nums[0];
        reorganyze_heap(&mut nums, 0, i);
    }

    nums
}

pub fn reorganyze_heap(nums: &mut Vec<i32>, indx: usize, len: usize) {
    let l = indx * 2 + 1;
    let r = l + 1;

    let mut ln = indx;
    if (len > l && nums[ln] < nums[l]) {
        ln = l
    };
    if (len > r && nums[ln] < nums[r]) {
        ln = r
    };

    if indx != ln {
        println!("swap {} and {}", indx, ln);
        nums[indx] = nums[ln] ^ nums[indx];
        nums[ln] = nums[indx] ^ nums[ln];
        nums[indx] = nums[indx] ^ nums[ln];
        reorganyze_heap(nums, ln, len);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let nums = vec![10, 4, 5, 2, 1, 3, 2, 1];
        println!("{:?}", heap_sort(nums));
    }
}
