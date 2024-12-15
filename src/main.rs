mod algo;
use algo::shell_sort::shell_sort;
use text_io::read;
fn main() {
    println!("Welcome!! chose an algo to visualize:");
    println!("1 - shell sort");

    loop {
        let input: String = read!("{}");
        match input.as_str() {
            "1" => {
                let nums = vec![12, 10, 14, 2, 3, 4, 1, 5];
                println!("{:?}", shell_sort(nums));
            }
            _ => {
                println!("choose a number end try again")
            }
        }
    }
}
