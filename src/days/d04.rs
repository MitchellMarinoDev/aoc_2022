// https://adventofcode.com/2022/day/4

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct ElfPair {
    /// Elf 1 lower bound.
    e1_lower: i32,
    /// Elf 1 upper bound.
    e1_upper: i32,
    /// Elf 2 lower bound.
    e2_lower: i32,
    /// Elf 2 upper bound.
    e2_upper: i32,
}

pub fn solve(input: String) -> (String, String) {
    let mut pairs = vec![];

    for line in input.lines() {
        let mut split = line.split(&['-', ','][..]);

        let e1_lower = split
            .next()
            .expect("not enough elements in line")
            .parse()
            .expect("failed to parse elf 1 lower bound");
        let e1_upper = split
            .next()
            .expect("not enough elements in line")
            .parse()
            .expect("failed to parse elf 1 upper bound");
        let e2_lower = split
            .next()
            .expect("not enough elements in line")
            .parse()
            .expect("failed to parse elf 2 lower bound");
        let e2_upper = split
            .next()
            .expect("not enough elements in line")
            .parse()
            .expect("failed to parse elf 2 upper bound");
        assert!(split.next().is_none(), "too many elements in line");

        pairs.push(ElfPair {
            e1_lower,
            e1_upper,
            e2_lower,
            e2_upper,
        })
    }

    let p1 = pairs
        .iter()
        .filter(|pair| {
            (pair.e1_lower <= pair.e2_lower && pair.e1_upper >= pair.e2_upper)
                || (pair.e2_lower <= pair.e1_lower && pair.e2_upper >= pair.e1_upper)
        })
        .count();

    let p2 = pairs
        .iter()
        .filter(|pair| {
            (pair.e1_lower >= pair.e2_lower && pair.e1_lower <= pair.e2_upper)
                || (pair.e2_lower >= pair.e1_lower && pair.e2_lower <= pair.e1_upper)
        })
        .count();

    (p1.to_string(), p2.to_string())
}
