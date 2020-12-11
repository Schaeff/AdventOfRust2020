const INPUT: &str = include_str!("../input.txt");

fn generate_new_grid(current: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    current
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .map(|(j, c)| match c {
                    '#' => {
                        if (i - 1..i + 2)
                            .flat_map(|i| (j - 1..j + 2).map(move |j| current[i][j]))
                            .filter(|c| *c == '#')
                            .count()
                            > 4
                        // strictly because the current cell is counted
                        {
                            'L'
                        } else {
                            '#'
                        }
                    }
                    'L' => (i - 1..i + 2)
                        .flat_map(|i| (j - 1..j + 2).map(move |j| current[i][j]))
                        .find(|c| *c == '#')
                        .map(|_| 'L')
                        .unwrap_or('#'),
                    c => *c,
                })
                .collect()
        })
        .collect()
}

fn visible(data: &Vec<Vec<char>>, position: (usize, usize)) -> impl Iterator<Item = char> + '_ {
    [
        (-1isize, -1isize),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .iter()
    .map(move |direction| {
        // iterate in this direction, until we reach the fence
        std::iter::repeat(direction)
            .scan(position, |(i, j), (di, dj)| {
                *i = (*i as isize + *di) as usize;
                *j = (*j as isize + *dj) as usize;

                data.get(*i).map(|l| l.get(*j).cloned()).flatten()
            })
            .filter(|c| *c != '.')
            .next()
            .unwrap()
    })
}

fn generate_new_grid_part_2(current: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    current
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .map(|(j, c)| match c {
                    '#' => {
                        if visible(current, (i, j)).filter(|c| *c == '#').count() >= 5
                        // not strictly because the current cell is not counted
                        {
                            'L'
                        } else {
                            '#'
                        }
                    }
                    'L' => visible(current, (i, j))
                        .find(|c| *c == '#')
                        .map(|_| 'L')
                        .unwrap_or('#'),
                    c => *c,
                })
                .collect()
        })
        .collect()
}

fn run(update: fn(&Vec<Vec<char>>) -> Vec<Vec<char>>) {
    let line_len = INPUT.split('\n').next().unwrap().len();

    // we add X all around so that we can always look at the 8 spots around any seat
    let mut grid: Vec<Vec<char>> = std::iter::once(vec!['X'; line_len + 2])
        .chain(INPUT.split('\n').map(|line| {
            std::iter::once('X')
                .chain(line.chars())
                .chain(std::iter::once('X'))
                .collect()
        }))
        .chain(std::iter::once(vec!['X'; line_len + 2]))
        .collect();

    let mut new_grid = None;

    let final_grid = loop {
        match new_grid {
            None => {
                new_grid = Some(update(&grid));
            }
            Some(g) => {
                if g == grid {
                    break grid;
                } else {
                    grid = g;
                    new_grid = Some(update(&grid))
                }
            }
        }
    };

    let res = final_grid
        .iter()
        .flat_map(|line| line.iter())
        .filter(|c| **c == '#')
        .count();

    println!("{}", res);
}

fn main() {
    run(generate_new_grid);

    run(generate_new_grid_part_2);
}
