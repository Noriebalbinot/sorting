mod algo;
use algo::{bubble_sort::bubble_sort, shell_sort::shell_sort};
use text_io::read;
fn main() {
    println!("Welcome!! chose an algo to visualize:");
    println!("1 - shell sort");
    println!("2 - bubble sort");

    loop {
        let input: String = read!("{}");
        let nums = vec![12, 10, 14, 2, 3, 4, 1, 5];
        match input.as_str() {
            "1" => {
                println!("{:?}", shell_sort(nums));
            }
            "2" => {
                println!("{:?}", bubble_sort(nums));
            }
            _ => {
                println!("choose a number end try again")
            }
        }
    }
}
