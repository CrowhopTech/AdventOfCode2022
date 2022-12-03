use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let lines = contents.split("\n");

    let mut sums: Vec<i32> = vec![];
    let mut current_sum = 0;

    for line in lines {
        if line == "" {
            sums.push(current_sum);
            current_sum = 0;
            continue;
        }
        // This is a number, parse and add
        let val: i32 = line
            .parse()
            .expect("Should have been able to parse the line");
        current_sum += val;
    }
    sums.sort();
    let largest_three = &sums[sums.len() - 3..];
    let largest_three_sum: i32 = largest_three.iter().sum();
    println!(
        "The sum of the largest three values is {}",
        largest_three_sum
    );
}
