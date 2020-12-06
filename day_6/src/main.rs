const INPUT: &str = include_str!("../input.txt");

use std::collections::HashSet;

fn main() {
    let res: usize = INPUT
        .split("\n\n")
        .map(|group| {
            group
                .split('\n')
                .flat_map(|person| person.chars())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum();

    println!("{}", res);

    let res: usize = INPUT
        .split("\n\n")
        .map(|group| {
            group
                .split('\n')
                .map(|person| person.chars().collect::<HashSet<_>>())
                .fold(None, |res, person| match res {
                    None => Some(person),
                    Some(res) => Some(res.intersection(&person).cloned().collect()),
                })
                .unwrap()
                .len()
        })
        .sum();

    println!("{}", res);
}
