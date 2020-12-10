const INPUT: &str = include_str!("../input.txt");

use std::collections::HashMap;

fn main() {
    let mut input: Vec<usize> = INPUT
        .split('\n')
        .map(|line| line.parse::<usize>().unwrap())
        .collect();

    input.push(0);

    input.sort();

    let (ones, threes) = input
        .windows(2)
        .fold((0, 0), |(ones, threes), w| match w[1] - w[0] {
            1 => (ones + 1, threes),
            3 => (ones, threes + 1),
            _ => (ones, threes),
        });

    println!("{}", ones * (threes + 1));

    // we cache how many arrangements are possible starting with `value` and picking adapters from index `start_at`
    fn count_arrangements(
        value: usize,
        start_at: usize,
        data: &[usize],
        cache: HashMap<(usize, usize), usize>,
    ) -> (usize, HashMap<(usize, usize), usize>) {
        match cache.get(&(value, start_at)) {
            Some(v) => {
                return (*v, cache);
            }
            None => {}
        };

        match data[start_at..].len() {
            0 => unreachable!(),
            // there's only one way to pick one adapter
            1 => (1, cache),
            _ => data[start_at..]
                .iter()
                .enumerate()
                .fold((0, cache), |(s, cache), (i, v)| match v - value {
                    1 | 2 | 3 => {
                        let (res, mut c) = count_arrangements(*v, start_at + i, data, cache);
                        // add to cache
                        c.insert((*v, start_at + i), res);
                        // sum the results and return the cache
                        (s + res, c)
                    }
                    _ => (s, cache),
                }),
        }
    }

    let res = count_arrangements(0, 0, &input, HashMap::default());

    println!("{:?}", res.0);
}
