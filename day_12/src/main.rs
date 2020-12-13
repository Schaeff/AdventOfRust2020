const INPUT: &str = include_str!("../input.txt");

use num_complex::Complex;

fn main() {
    let (_, position) = INPUT.split('\n').fold(
        (Complex::new(1isize, 0isize), Complex::from(0)),
        |(angle, position), instruction| {
            let length = instruction[1..].parse::<usize>().unwrap();

            let (angle, direction) = match &instruction.chars().nth(0).unwrap() {
                'N' => (angle, Complex::new(0, 1)),
                'S' => (angle, Complex::new(0, -1)),
                'E' => (angle, Complex::new(1, 0)),
                'W' => (angle, Complex::new(-1, 0)),
                'F' => (angle, angle),
                'L' => (
                    angle * Complex::i().powu(length as u32 / 90),
                    Complex::from(0),
                ),
                'R' => (
                    angle / Complex::i().powu(length as u32 / 90),
                    Complex::from(0),
                ),
                _ => unreachable!(),
            };

            let movement = direction * (length as isize);

            let position = position + movement;

            (angle, position)
        },
    );

    let res = position.re.abs() + position.im.abs();

    println!("{}", res);

    let (_, position) = INPUT.split('\n').fold(
        (Complex::new(10, 1), Complex::new(0, 0)),
        |(waypoint, position), instruction| {
            let length = instruction[1..].parse::<isize>().unwrap();

            match &instruction.chars().nth(0).unwrap() {
                'N' => (waypoint + Complex::new(0, 1) * length, position),
                'S' => (waypoint + Complex::new(0, -1) * length, position),
                'E' => (waypoint + Complex::new(1, 0) * length, position),
                'W' => (waypoint + Complex::new(-1, 0) * length, position),
                'F' => (
                    waypoint + (waypoint - position) * length,
                    position + (waypoint - position) * length,
                ),
                'L' => (
                    position + (waypoint - position) * Complex::i().powu(length as u32 / 90),
                    position,
                ),
                'R' => (
                    position + (waypoint - position) / Complex::i().powu(length as u32 / 90),
                    position,
                ),
                _ => unreachable!(),
            }
        },
    );

    let res = position.re.abs() + position.im.abs();

    println!("{}", res);
}
