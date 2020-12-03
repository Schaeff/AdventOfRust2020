const INPUT: &str = include_str!("../input.txt");

fn is_valid_1(line: &&str) -> bool {
    let mut blocks = line.split(" ");
    let from_to = blocks.next().unwrap();
    let character = blocks.next().unwrap();
    let password = blocks.next().unwrap();

    let mut from_to = from_to.split("-");
    let from = from_to.next().unwrap().parse::<usize>().unwrap();
    let to = from_to.next().unwrap().parse::<usize>().unwrap();

    let character = character.chars().next().unwrap();

    let occurences = password.chars().filter(|c| *c == character).count();

    occurences >= from && occurences <= to
}

fn is_valid_2(line: &&str) -> bool {
    let mut blocks = line.split(" ");
    let from_to = blocks.next().unwrap();
    let character = blocks.next().unwrap();
    let password = blocks.next().unwrap();

    let mut from_to = from_to.split("-");
    let from = from_to.next().unwrap().parse::<usize>().unwrap();
    let to = from_to.next().unwrap().parse::<usize>().unwrap();

    let character = character.chars().next().unwrap();

    password
        .chars()
        .enumerate()
        .map(|(index, c)| (index + 1, c))
        .filter(|(index, c)| (*index == from || *index == to) && *c == character)
        .count()
        == 1
}

fn main() {
    let res = INPUT.split('\n').filter(is_valid_1).count();

    println!("{}", res);

    let res = INPUT.split('\n').filter(is_valid_2).count();

    println!("{}", res);
}
