// https://adventofcode.com/2022/day/3

use std::collections::HashSet;

struct RucksackSets {
    left: HashSet<char>,
    right: HashSet<char>,
    all: HashSet<char>,
}

pub fn solve(input: String) -> (String, String) {
    let mut rucksacks = vec![];

    for line in input.lines() {
        let mut all = HashSet::new();
        let mut left = HashSet::new();
        let mut right = HashSet::new();

        let halfway = line.len() / 2;
        for (i, c) in line.chars().enumerate() {
            let set = if i < halfway { &mut left } else { &mut right };
            set.insert(c);
            all.insert(c);
        }
        rucksacks.push(RucksackSets { left, right, all });
    }

    let p1 = rucksacks
        .iter()
        .map(|rs| {
            let mut intersection = rs.left.intersection(&rs.right);
            let char = intersection.next().expect("line did not have an error");
            assert!(intersection.next().is_none(), "line had more than 1 error.");
            priority(*char)
        })
        .sum::<i32>();

    let mut p2 = 0;
    let mut rucksack_iter = rucksacks.iter();

    while let Some(elf1) = rucksack_iter.next() {
        let elf2 = rucksack_iter
            .next()
            .expect("ruckstack count is NOT a multiple of 3");
        let elf3 = rucksack_iter
            .next()
            .expect("ruckstack count is NOT a multiple of 3");

        let mut badge_options = elf1
            .all
            .intersection(&elf2.all)
            .filter(|&c| elf3.all.contains(c));
        let badge = *badge_options.next().expect("No badge found");
        assert!(badge_options.next().is_none(), "Multiple badges found");
        p2 += priority(badge);
    }

    (p1.to_string(), p2.to_string())
}

fn priority(c: char) -> i32 {
    match c {
        'a'..='z' => c as i32 - 'a' as i32 + 1,
        'A'..='Z' => c as i32 - 'A' as i32 + 27,
        _ => panic!("char {} does not have a priority", c),
    }
}

#[test]
#[ignore]
fn test_priority() {
    assert_eq!(priority('a'), 1);
    assert_eq!(priority('b'), 2);
    assert_eq!(priority('z'), 26);
    assert_eq!(priority('A'), 27);
    assert_eq!(priority('B'), 28);
    assert_eq!(priority('Z'), 52);

    assert_eq!(priority('p'), 16);
    assert_eq!(priority('L'), 38);
    assert_eq!(priority('P'), 42);
    assert_eq!(priority('v'), 22);
    assert_eq!(priority('t'), 20);
    assert_eq!(priority('s'), 19);
}
