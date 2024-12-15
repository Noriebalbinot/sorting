pub fn selection_sort(mut nums: Vec<i32>) -> Vec<i32> {
    println!("We wil run a for loop in the array and for each");
    println!("number we gonna search the smallest number in the rest of the array");
    println!("after we find we swap it");
    for i in 0..nums.len() {
        println!("for the iteration {}", i);
        let mut min = (nums[i], i);
        for j in i + 1..nums.len() {
            if nums[j] < min.0 {
                min = (nums[j], j)
            }
        }
        println!("the smallest that we find is {} and the indicies is {}", min.0, min.1);
        if i != min.1 {
          println!("swap {} and {}", i, min.1);
            nums[i] = nums[min.1] ^ nums[i];
            nums[min.1] = nums[i] ^ nums[min.1];
            nums[i] = nums[i] ^ nums[min.1];
        }
    }

    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let nums = vec![10, 4, 5, 2, 1, 3, 2, 1];
        println!("{:?}", selection_sort(nums));
    }
}
