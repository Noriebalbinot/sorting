struct Gap {
    i: usize,
}

impl Iterator for Gap {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.i = self.i / 2;
        match self.i.cmp(&0) {
            std::cmp::Ordering::Greater => Some(self.i),
            _ => None,
        }
    }
}

pub fn shell_sort(mut nums: Vec<i32>) -> Vec<i32> {
    let s = nums.len();
    println!("first of all we generate a gap who gona decrease until 1");
    println!("from the length of the array dividing by 2 at each iteration of the main loop");
    println!("now we run a for loop in a range of the gap..lenght");
    println!("and check if the number that we on now (i) is >= gap and ");
    println!("if the number located in nums[i-gap] is bigger than nums[i]");
    println!("and if they are we perform a swap on the 2 elements");
    let gap = Gap { i: s };

    for g in gap {
        println!("gap:{g}");
        for mut i in g..s {
            while i >= g && nums[i - g] > nums[i] {
                // xor
                nums[i - g] = nums[i - g] ^ nums[i];
                nums[i] = nums[i] ^ nums[i - g];
                nums[i - g] = nums[i] ^ nums[i - g];
                println!("swap: {} and {}: {:?}", i - g, i, nums);
                i -= g;
            }
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
        println!("{:?}", shell_sort(nums));
    }
}
