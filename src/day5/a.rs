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
                let seq: Vec<usize> = line
                    .split(',')
                    .map(|x| x.parse::<usize>().expect("Page should be number"))
                    .collect();
                let mut viewed = vec![];

                let mut mid = seq[seq.len() / 2];

                for elem in seq {
                    if viewed
                        .iter()
                        .any(|page| following_page[elem].contains(page))
                    {
                        mid = 0;
                        break;
                    }

                    viewed.push(elem);
                }

                mid
            }
        })
        .sum();

    println!("{sol}");
}
