use regex::Regex;

fn main() {
    let input = include_str!("input");

    let regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let muls = regex.find_iter(input);
    
    let sol: i32 = muls.map(|mul| {
        mul.as_str()
            .strip_prefix("mul(")
            .and_then(|mul| mul.strip_suffix(')'))
            .and_then(|mul| mul.split_once(','))
            .and_then(|(x, y)| {
                match (x.parse::<i32>(), y.parse::<i32>()) {
                    (Ok(x), Ok(y)) => Some(x * y),
                    _ => None
                }
            }).expect("Matched string should respect the pattern")
    }).sum();

    println!("{sol}");
}
