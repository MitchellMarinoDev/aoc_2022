// https://adventofcode.com/2022/day/1

pub fn solve(input: String) -> (String, String) {
    let inventory = input
        .split("\n\n")
        .map(|e| {
            e.lines()
                .map(|n| n.parse::<i32>().expect(&*format!("{} is not a number", n)))
                .sum::<i32>()
        })
        .collect::<Vec<_>>();

    let p1 = inventory
        .iter()
        .max()
        .expect("input of len > 0")
        .to_string();

    let p2 = maxn(inventory.iter(), 3).iter().sum::<i32>().to_string();

    (p1, p2)
}

fn maxn<'a>(iter: impl Iterator<Item = &'a i32>, n: usize) -> Vec<i32> {
    let mut max = vec![];

    for &element in iter {
        // for the first n elements, prime the max list.
        if max.len() < n {
            max.push(element);
            if max.len() == n {
                max.sort();
            }
            continue;
        }

        // if the element should replace an element in the max list.
        if let Some(idx) = replaces(element, &*max) {
            // remove the elf with the least calories
            max.pop();
            // insert the new elf at the right spot to keep it ordered.
            max.insert(idx, element);
        }
    }

    max
}

/// Checks if a number `n` is greater than any element the ordered (descending) list `list`.
///
/// Returns the index (if any) of the element that it is greater than.
fn replaces(n: i32, list: &[i32]) -> Option<usize> {
    let len = list.len();

    for (i, element) in list.iter().enumerate().rev() {
        if n < *element {
            return if i == len - 1 { None } else { Some(i + 1) };
        }
    }

    Some(0)
}
