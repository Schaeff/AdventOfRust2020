const INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut lines = INPUT.split('\n');
    let start = lines.next().unwrap().parse::<usize>().unwrap();
    let busses: Vec<_> = lines
        .next()
        .unwrap()
        .split(',')
        .filter(|b| *b != "x")
        .map(|b| b.parse::<usize>().unwrap())
        .collect();

    let (bus, time) = (start..)
        .map(|time| (busses.iter().find(|bus| time % **bus == 0), time))
        .find(|(bus, _)| bus.is_some())
        .unwrap();

    let res = bus.unwrap() * (time - start);

    println!("{}", res);

    let mut busses: Vec<(usize, usize)> = INPUT
        .split('\n')
        .skip(1)
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(index, b)| b.parse::<usize>().map(|b| (index, b)).ok())
        .collect();

    // sort busses in decreasing frequency
    busses.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // normalize so that we're solving `t == remainder_i mod bus_i` with `remainder_i < bus_i`
    let busses: Vec<_> = busses
        .into_iter()
        .map(|(index, bus_id)| ((bus_id - index % bus_id) % bus_id, bus_id))
        .collect();

    // apply chinese remainder theorem, see [here](https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Search_by_sieving)
    let res = busses.into_iter().fold(None, |acc, (remainder, bus_id)| {
        Some(
            acc.map(|(value, step)| {
                (0..)
                    .map(|k| k * step + value)
                    .find(|t| t % bus_id == remainder)
                    .map(|t| (t, bus_id * step))
                    .unwrap()
            })
            .unwrap_or((remainder, bus_id)),
        )
    });

    println!("{}", res.unwrap().0);
}
