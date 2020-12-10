const INPUT: &str = include_str!("../input.txt");

const STEP: usize = 25;

use std::collections::VecDeque;

fn main() {
    let numbers: Vec<usize> = INPUT
        .split('\n')
        .map(|line| line.parse::<usize>().unwrap())
        .collect();

    // calculate all possible sums
    let sums: VecDeque<Vec<usize>> = numbers[0..STEP]
        .iter()
        .enumerate()
        .map(|(i, a)| {
            numbers[0..STEP]
                .iter()
                .enumerate()
                .filter(move |(j, _)| *j < STEP && i < *j)
                .map(move |(_, b)| a + b)
                .collect()
        })
        .collect();

    let res: (usize, usize) = numbers
        .windows(STEP + 1)
        .enumerate()
        .scan(sums, |state, (index, w)| {
            match state.iter().flatten().find(|x| *x == &w[STEP]) {
                Some(_) => {
                    // update the possible sums
                    // the sums at the front of the queue are the ones involving the first element, let's remove them
                    let first = state.pop_front().unwrap();
                    // insert a new vector for sums involving the new element
                    state.push_back(vec![]);
                    // update the sums based on removing the first element and adding the last one
                    for (s, n) in state.iter_mut().zip(first.into_iter()) {
                        // we reuse the elements we just removed, subtracting the first element and adding the last
                        s.push(n - w[0] + w[STEP]);
                    }
                    Some(None)
                }
                None => Some(Some((index + STEP, w[STEP]))),
            }
        })
        .find(|r| r.is_some())
        .unwrap()
        .unwrap();

    println!("{}", res.1);

    let (index, target) = res;

    fn find_segment(
        target: usize,
        data: &[usize],
        from: usize,
        to: usize,
        sum: usize,
        max: usize,
    ) -> Option<(usize, usize)> {
        // note: `to` is excluded
        if sum == target {
            Some((from, to))
        } else if sum < target {
            if to + 1 == max {
                // we reached the end of the list
                None
            } else {
                // extend the section on the right
                find_segment(target, data, from, to + 1, sum + data[to], max)
            }
        } else {
            // shrink the section on the left
            find_segment(target, data, from + 1, to, sum - data[from], max)
        }
    }

    let (from, to) = find_segment(target, &numbers[0..index], 0, 0, 0, index).unwrap();

    println!(
        "{:?}",
        numbers[from..to].iter().max().unwrap() + numbers[from..to].iter().min().unwrap()
    );
}
