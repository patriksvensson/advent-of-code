fn main() {
    let input = include_str!("../input.txt");
    let mut results = Vec::new();

    let mut sum = 0;
    for line in input.lines() {
        if line == "" {
            results.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }

    results.sort();

    println!("First = {}", results.iter().rev().take(1).sum::<i32>());
    println!("Top 3 = {}", results.iter().rev().take(3).sum::<i32>());
}
