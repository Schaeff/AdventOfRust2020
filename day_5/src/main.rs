const INPUT: &str = include_str!("../input.txt");

fn main() {
    let seats = INPUT.split('\n').map(|line| {
        line.chars()
            .rev()
            .enumerate()
            .fold(0, |acc, (index, e)| match e {
                'B' | 'R' => acc + 2u32.pow(index as u32),
                _ => acc,
            })
    });

    println!("{}", seats.clone().max().unwrap());

    let mut sorted = seats.collect::<Vec<_>>();
    sorted.sort();

    let missing = sorted.windows(2).find(|w| w[1] != w[0] + 1).unwrap()[0] + 1;

    println!("{}", missing);
}
