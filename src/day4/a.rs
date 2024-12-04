const SEARCHED: &str = "XMAS";
const N: usize = SEARCHED.len();

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn within_boundaries(text: &[Vec<char>], x: isize, y: isize) -> bool {
    x >= 0 && y >= 0 && (x as usize) < text.len() && (y as usize) < text[0].len()
}

fn search(text: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    DIRECTIONS
        .iter()
        .map(|(dx, dy)| {
            let mut nx = x as isize;
            let mut ny = y as isize;
            let mut acc = vec![];

            for _ in 0..N {
                if !within_boundaries(text, nx, ny) {
                    break;
                }

                acc.push(text[nx as usize][ny as usize]);

                nx += dx;
                ny += dy;
            }

            let acc = acc.into_iter();
            acc.eq(SEARCHED.chars()) as usize
        })
        .sum()
}

fn main() {
    let input = include_str!("input");
    let text: Vec<_> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let sol = (0..text.len())
        .map(|x| {
            (0..text[0].len())
                .map(|y| search(&text, x, y))
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("{sol}");
}
