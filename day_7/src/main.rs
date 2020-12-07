mod part_1_quadratic;

const INPUT: &str = include_str!("../input.txt");

use std::collections::{HashMap, HashSet};

type Color<'a> = &'a str;

type ReversedColors<'a> = HashMap<Color<'a>, Vec<Color<'a>>>;

// a cache to store all containers
type Reached<'a> = HashSet<Color<'a>>;

// find all colors reachable from a given color in the reversed graph
fn color_is_contained_in_other_color<'a>(
    color: &Color<'a>,
    colors: &ReversedColors<'a>,
    reached: Reached<'a>,
) -> Reached<'a> {
    match reached.contains(color) {
        // this color was already reached, we return the cache as is
        true => reached,
        // this color was not already reached, add it and its children to the cache
        false => match colors.get(color) {
            Some(containers) => containers.iter().fold(reached, |reached, c| {
                // if we first add `c`, then its children will not be visited due to caching
                let mut reached = color_is_contained_in_other_color(c, colors, reached);
                reached.insert(c);
                reached
            }),
            None => reached,
        },
    }
}

type CountedColors<'a> = HashMap<Color<'a>, Vec<(usize, Color<'a>)>>;

type CountedCache<'a> = HashMap<Color<'a>, usize>;

// get the number of bags in a given bag *including itself* and return it in a cache
// an empty bag yields 1
// a non empty bag returns the sum of its children plus 1
fn count<'a>(
    color: &Color<'a>,
    colors: &CountedColors<'a>,
    cache: CountedCache<'a>,
) -> CountedCache<'a> {
    match cache.get(color) {
        Some(_) => cache,
        None => {
            let (mut cache, m) =
                colors
                    .get(color)
                    .unwrap()
                    .iter()
                    .fold((cache, 0), |(cache, m), (n, c)| {
                        if c == color {
                            panic!("INFINITY");
                        } else {
                            let cache = count(c, colors, cache);

                            let count = n * cache.get(c).unwrap();

                            (cache, m + count)
                        }
                    });

            cache.insert(color, m + 1);

            cache
        }
    }
}

fn main() {
    // This was my first idea.
    // This is not ideal as it find reachability for any pair of colors, so the cache size is
    // quadratic in the number of colors. It does not take into account the fact that we're
    // looking at the reachability of a given color.
    // A better idea would be to build the inversed graph: (contained -> container[]), start from the searched value,
    // and find all reachable nodes
    part_1_quadratic::main();

    // build the reversed graph, from each color to the colors that contain it
    let colors: ReversedColors = INPUT
        .split("\n")
        .map(|line| {
            let mut line = line.split(" bags contain ");
            let container: Color = line.next().unwrap();

            let contained: Vec<Color> = line
                .next()
                .unwrap()
                .split(", ")
                .map(|contained| contained.split(" bag").next().unwrap())
                .map(|numbered_color_or_no_other| {
                    if numbered_color_or_no_other == "no other" {
                        Err(())
                    } else {
                        Ok(numbered_color_or_no_other)
                    }
                })
                .collect::<Result<_, _>>()
                .unwrap_or(vec![])
                .into_iter()
                .map(|numbered_color| &numbered_color[2..])
                .collect();

            (container, contained)
        })
        .fold(
            ReversedColors::default(),
            |mut is_contained_in, (container, contained)| {
                contained.into_iter().for_each(|contained| {
                    is_contained_in
                        .entry(contained)
                        .or_default()
                        .push(container.clone())
                });
                is_contained_in
            },
        );

    let searched = "shiny gold";

    let reached = color_is_contained_in_other_color(&searched, &colors, Reached::default());

    let res = reached.len();

    println!("{}", res);

    let counted_colors: CountedColors = INPUT
        .split("\n")
        .map(|line| {
            let mut line = line.split(" bags contain ");
            let container: Color = line.next().unwrap();

            let contained: Vec<(usize, Color)> = line
                .next()
                .unwrap()
                .split(", ")
                .map(|contained| contained.split(" bag").next().unwrap())
                .map(|numbered_color_or_no_other| {
                    if numbered_color_or_no_other == "no other" {
                        Err(())
                    } else {
                        Ok(numbered_color_or_no_other)
                    }
                })
                .collect::<Result<_, _>>()
                .unwrap_or(vec![])
                .into_iter()
                .map(|numbered_color| {
                    (
                        *&numbered_color[0..1].parse::<usize>().unwrap(),
                        &numbered_color[2..],
                    )
                })
                .collect();

            (container, contained)
        })
        .collect();

    let cache = count(&searched, &counted_colors, CountedCache::default());

    // subtract 1 because we want the number of bags inside the shiny gold, excluding shiny gold
    let res = cache.get(&searched).unwrap() - 1;

    println!("{}", res);
}
