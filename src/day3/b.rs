use regex::Regex;

fn main() {
    let input = include_str!("input");

    let regex = Regex::new(r"don't\(\)|do\(\)|mul\(\d{1,3},\d{1,3}\)").unwrap();
    let instructions = regex.find_iter(input);

    let mut execute = true;
    let sol: i32 = instructions
        .filter_map(|ins| match ins.as_str() {
            "don't()" => {
                execute = false;
                None
            }
            "do()" => {
                execute = true;
                None
            }
            mul if execute => mul
                .strip_prefix("mul(")
                .and_then(|mul| mul.strip_suffix(')'))
                .and_then(|mul| mul.split_once(','))
                .and_then(|(x, y)| match (x.parse::<i32>(), y.parse::<i32>()) {
                    (Ok(x), Ok(y)) => Some(x * y),
                    _ => None,
                }),
            _ => None,
        })
        .sum();

    println!("{sol}");
}
