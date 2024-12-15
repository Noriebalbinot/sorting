pub fn bubble_sort(mut nums: Vec<i32>) -> Vec<i32> {
    println!("in a bubble sort we first run a reverse loop in the array");
    println!("then we set a flag for us the se if the array has performed a swap");
    println!("and then run a second loop from 0 to the actual indicies of the first loop");
    println!("checking if the actual number in the second loop is bigger than the");
    println!("the actual number + 1 if it is we perform a swap an set the flag on");
    for i in (0..nums.len()).rev() {
        let mut swapflag = false;
        for j in 0..i {
            if nums[j] > nums[j + 1] {
                println!("swap: {} and {}", j, j + 1);
                nums[j] = nums[j] ^ nums[j + 1];
                nums[j + 1] = nums[j] ^ nums[j + 1];
                nums[j] = nums[j] ^ nums[j + 1];
                swapflag = true;
            }
            println!("{:?}", nums)
        }
        if !swapflag {
            break;
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
        println!("{:?}", bubble_sort(nums));
    }
}
