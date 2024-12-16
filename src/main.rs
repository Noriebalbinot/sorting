mod algo;
use algo::{
    bubble_sort::bubble_sort, heap_sort::heap_sort, selection_sort::selection_sort,
    shell_sort::shell_sort,
};
use text_io::read;
fn main() {
    println!("Welcome!! chose an algo to explain:");
    println!("1 - shell sort");
    println!("2 - bubble sort");
    println!("3 - selection sort");
    println!("4 - heap sort");

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
            "3" => {
                println!("{:?}", selection_sort(nums));
            }
            "4" => {
                println!("{:?}", heap_sort(nums));
            }
            _ => {
                println!("choose a number end try again")
            }
        }
    }
}
