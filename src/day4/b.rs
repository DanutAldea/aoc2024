fn main() {
    let input = include_str!("input");
    let text: Vec<_> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let sol = (1..(text.len() - 1))
        .map(|x| {
            (1..(text[0].len() - 1))
                .map(|y| {
                    if text[x][y] != 'A' {
                        0
                    } else {
                        let cross = [
                            text[x - 1][y - 1],
                            text[x - 1][y + 1],
                            text[x + 1][y + 1],
                            text[x + 1][y - 1],
                        ];

                        let m_count = cross.iter().filter(|m| **m == 'M').count();
                        let s_count = cross.iter().filter(|s| **s == 'S').count();

                        (cross[0] != cross[2]
                            && cross[1] != cross[3]
                            && m_count == 2
                            && s_count == 2) as usize
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("{sol}");
}
