const MAX_PAGE: usize = 100;

fn main() {
    let input = include_str!("input");
    let mut following_page = vec![vec![]; MAX_PAGE];
    let mut read_ord = true;

    let sol: usize = input
        .lines()
        .map(|line| {
            if line.is_empty() {
                read_ord = false;
                0
            } else if read_ord {
                if let Some((x, y)) = line.split_once('|') {
                    let x = x.parse::<usize>().expect("Page should be number");
                    let y= y.parse::<usize>().expect("Page should be number");
                    following_page[x].push(y);
                }
                0
            } else {
                let mut seq: Vec<usize> = line
                    .split(',')
                    .map(|x| x.parse::<usize>().expect("Page should be number"))
                    .collect();

                let mut incorrect = false;

                for (i, &x) in seq.iter().enumerate() {
                    for &y in seq.iter().skip(i + 1) {
                        if following_page[y].contains(&x) {
                            incorrect = true;
                            break;
                        }
                    }
                }

                if incorrect {
                    while incorrect {
                        incorrect = false;
                        for i in 0..seq.len() {
                            for j in (i + 1)..seq.len() {
                                let x = seq[i];
                                let y = seq[j];

                                if following_page[y].contains(&x) {
                                    seq[i] = y;
                                    seq[j] = x;

                                    incorrect = true
                                }
                            }
                        }
                    }

                    seq[seq.len() / 2]
                } else {
                    0
                }
            }
        })
        .sum();

    println!("{sol}");
}
