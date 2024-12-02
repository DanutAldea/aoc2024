use std::convert::identity;

fn main() {
    let input = include_str!("input");

    // I was challenged to do this in an one-liner, so here it is
    let sol = input
        .lines()
        .map(|line| {
            let report: Vec<i32> = line
                .split_ascii_whitespace()
                .map(|n| n.parse::<i32>().expect("Input should be numbers"))
                .collect();

            (0..report.len())
                .map(|idx| {
                    let mut report = report.clone();
                    report.remove(idx);

                    report
                        .windows(2)
                        .map(|w| w[0].abs_diff(w[1]))
                        .all(|diff| (0..=3).contains(&diff))
                        && report
                            .windows(2)
                            .map(|w| w[0].cmp(&w[1]))
                            .fold((true, None), |(ok, ord), elem| {
                                (ok && ord.map_or(true, |ord| ord == elem), Some(elem))
                            })
                            .0
                })
                .any(identity)
        })
        .filter(|v| *v)
        .count();

    println!("{sol}");
}
