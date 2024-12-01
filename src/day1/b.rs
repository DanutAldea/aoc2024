use std::collections::HashMap;

fn main() {
    let input = include_str!("input");
    let mut list1 = vec![];
    let mut list2 = HashMap::new();

    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_ascii_whitespace()
            .take(2)
            .map(|num| num.parse().expect("Input should be numbers"))
            .collect();

        list1.push(nums[0]);

        let count = list2.get(&nums[1]).unwrap_or(&0);
        list2.insert(nums[1], count + 1);
    }

    list1.sort();

    let sol = list1
        .into_iter()
        .map(|elem| elem * list2.get(&elem).unwrap_or(&0))
        .reduce(|acc, e| acc + e)
        .expect("The list is not empty");

    println!("{sol}");
}
