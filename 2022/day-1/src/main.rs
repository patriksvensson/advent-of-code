fn main() {
    let input = include_str!("../input.txt");

    let mut sum = 0;
    let mut highest = 0;

    for line in input.lines() {
        if line == "" {
            if sum > highest {
                highest = sum;
            }
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }

    println!("Result = {}", highest);
}
