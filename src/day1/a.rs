fn main() {
    let input = include_str!("input");
    let mut list1 = vec![];
    let mut list2 = vec![];

    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_ascii_whitespace()
            .take(2)
            .map(|num| num.parse().expect("Input should be numbers"))
            .collect();

        list1.push(nums[0]);
        list2.push(nums[1]);
    }

    list1.sort();
    list2.sort();

    let sol = list1
        .into_iter()
        .zip(list2)
        .map(|(a, b)| a.abs_diff(b))
        .reduce(|acc, e| acc + e)
        .expect("The list is not empty");

    println!("{sol}");
}
