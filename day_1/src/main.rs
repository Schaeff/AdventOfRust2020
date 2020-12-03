const INPUT: &str = include_str!("../input.txt");

fn main() {
    let input: Vec<usize> = INPUT
        .split('\n')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let (x, y) = input
        .iter()
        .flat_map(|i| input.iter().map(move |j| (i, j)))
        .find(|(x, y)| **x + **y == 2020)
        .unwrap();

    println!("{:?}", x * y);

    let (x, y, z) = input
        .iter()
        .flat_map(|i| input.iter().map(move |j| (i, j)))
        .flat_map(|(i, j)| input.iter().map(move |k| (i, j, k)))
        .find(|(x, y, z)| **x + **y + **z == 2020)
        .unwrap();

    println!("{:?}", x * y * z);
}
