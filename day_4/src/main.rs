const INPUT: &str = include_str!("../input.txt");

use std::collections::HashSet;

fn valid_byr(byr: &str) -> bool {
    byr.parse::<usize>()
        .map(|byr| byr >= 1920 && byr <= 2002)
        .unwrap_or(false)
}

fn valid_iyr(iyr: &str) -> bool {
    iyr.parse::<usize>()
        .map(|iyr| iyr >= 2010 && iyr <= 2020)
        .unwrap_or(false)
}

fn valid_eyr(eyr: &str) -> bool {
    eyr.parse::<usize>()
        .map(|eyr| eyr >= 2020 && eyr <= 2030)
        .unwrap_or(false)
}

fn valid_hgt(byr: &str) -> bool {
    byr.split("cm")
        .next() // returns the whole str if "cm" is not a suffix, which will fail when parsing to usize
        .map(|height_in_cm| height_in_cm.parse::<usize>().map(|h| h >= 150 && h <= 193))
        .unwrap()
        .unwrap_or(
            byr.split("in")
                .next()
                .map(|height_in_inches| {
                    height_in_inches
                        .parse::<usize>()
                        .map(|h| h >= 59 && h <= 76)
                })
                .unwrap()
                .unwrap_or(false),
        )
}

fn valid_hcl(hcl: &str) -> bool {
    hcl.split('#')
        .skip(1)
        .next()
        .map(|hcl| hcl.len() == 6 && usize::from_str_radix(hcl, 16).is_ok())
        .unwrap_or(false)
}
fn valid_ecl(ecl: &str) -> bool {
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl)
}

fn valid_pid(pid: &str) -> bool {
    pid.len() == 9 && pid.parse::<usize>().is_ok()
}

fn main() {
    let target: HashSet<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .into_iter()
        .collect();

    let count = INPUT
        .split("\n\n")
        .filter(|passport| {
            let set = passport
                .split(|c| c == ' ' || c == '\n')
                .map(|field| field.split(':').next().unwrap())
                .filter(|field| *field != "cid")
                .collect::<HashSet<_>>();
            set == target
        })
        .count();

    println!("{}", count);

    let count = INPUT
        .split("\n\n")
        .filter(|passport| {
            // the set of *valid* fields
            let set = passport
                .split(|c| c == ' ' || c == '\n')
                .map(|field| {
                    let mut iter = field.split(':');
                    (iter.next().unwrap(), iter.next().unwrap())
                })
                .filter(|(field, _)| *field != "cid")
                .filter(|(field, value)| match *field {
                    "byr" => valid_byr(value),
                    "iyr" => valid_iyr(value),
                    "eyr" => valid_eyr(value),
                    "hgt" => valid_hgt(value),
                    "hcl" => valid_hcl(value),
                    "ecl" => valid_ecl(value),
                    "pid" => valid_pid(value),
                    _ => false,
                })
                .map(|(field, _)| field)
                .collect::<HashSet<_>>();
            set == target
        })
        .count();

    println!("{}", count);
}
