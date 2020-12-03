const INPUT: &str = include_str!("../input.txt");

fn count_trees(right: usize, down: usize) -> usize {
    let lines = INPUT.split("\n");

    lines
        .step_by(down)
        .fold((0, 0), |mut acc, line| {
            if line.chars().nth(acc.0 % line.len()).unwrap() == '#' {
                acc.1 += 1;
            }
            acc.0 += right;
            acc
        })
        .1
}

fn main() {
    let lines = INPUT.split("\n");

    let res = lines
        .fold((0, 0), |mut acc, line| {
            if line.chars().nth(acc.0 % line.len()).unwrap() == '#' {
                acc.1 += 1;
            }
            acc.0 += 3;
            acc
        })
        .1;

    println!("{}", res);

    let res: usize = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|(right, down)| count_trees(right, down))
        .product();
    println!("{}", res);
}
