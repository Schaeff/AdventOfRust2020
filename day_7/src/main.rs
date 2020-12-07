const INPUT: &str = include_str!("../input.txt");

use std::collections::{BTreeMap, BTreeSet};

type Color<'a> = &'a str;

type Colors<'a> = BTreeMap<Color<'a>, Vec<Color<'a>>>;

// a cache to store whether a color (transitively) contains another
type Cache<'a> = BTreeSet<(Color<'a>, Color<'a>)>;

// return the new cache after checking recursively if `other_color` can be found in `color`
fn color_can_contain_other_color<'a>(
    color: &Color<'a>,
    other_color: &Color<'a>,
    colors: &Colors<'a>,
    cache: Cache<'a>,
) -> Cache<'a> {
    match cache.contains(&(*color, *other_color)) {
        // if we already know the answer, we return the cache as is
        true => cache,
        // otherwise we check if the colors contained in `color` contain `other_color`
        false => colors
            .get(color)
            .unwrap()
            .iter()
            .fold(cache, |mut cache, c| {
                // `color` transitively contains all colors which it directly contains
                cache.insert((color, c));
                if c == color {
                    // if a color contains itself, we learn nothing from exploring it again
                    cache
                } else {
                    let mut cache = color_can_contain_other_color(c, other_color, colors, cache);

                    match cache.get(&(*c, *other_color)).cloned() {
                        // if a color contained directly by `color` contains transitively `other_color`,
                        // then `color` contains transitively `other_color`
                        Some(_) => {
                            cache.insert((color, other_color));
                            cache
                        }
                        // other wise we don't learn anything new
                        None => cache,
                    }
                }
            }),
    }
}

type CountedColors<'a> = BTreeMap<Color<'a>, Vec<(usize, Color<'a>)>>;

type CountedCache<'a> = BTreeMap<Color<'a>, usize>;

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
    let colors: Colors = INPUT
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
        .collect();

    let searched = "shiny gold";

    let cache = colors.keys().fold(Cache::default(), |cache, color| {
        color_can_contain_other_color(color, &searched, &colors, cache)
    });

    let res = cache.iter().filter(|(_, other)| *other == searched).count();

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
