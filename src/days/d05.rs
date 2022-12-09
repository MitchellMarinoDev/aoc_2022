// https://adventofcode.com/2022/day/5

const N_STACKS: usize = 9;

struct Instruction {
    /// The number of crates to move.
    count: usize,
    /// The 0-index of the stack to move from.
    from: usize,
    /// The 0-index of the stack to move to.
    to: usize,
}

pub fn solve(input: String) -> (String, String) {
    let mut split = input.split("\n\n");
    let crates_input = split.next().expect("No crate map");
    let instructions_input = split.next().expect("No instructions");
    assert!(
        split.next().is_none(),
        "There should only be one \\n\\n in the input"
    );

    let mut stacks_p1 = (0..N_STACKS).map(|_| Vec::new()).collect::<Vec<_>>();
    for line in crates_input.lines().rev() {
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            // skip the line with the numbers
            if c.is_numeric() {
                break;
            }

            if c != ' ' {
                stacks_p1[i].push(c);
            }
        }
    }
    let mut stacks_p2 = stacks_p1.clone();

    let mut instructions = vec![];
    for line in instructions_input.lines() {
        let mut split = line.split(' ').filter_map(|s| s.parse::<usize>().ok());
        let count = split.next().expect("no move count");
        let from = split.next().expect("no from stack") - 1;
        let to = split.next().expect("no to stack") - 1;
        instructions.push(Instruction { count, from, to });
    }

    for instruction in instructions.iter() {
        for _ in 0..instruction.count {
            let move_crate = stacks_p1[instruction.from]
                .pop()
                .expect("tried to move crate that doesnt exist");
            stacks_p1[instruction.to].push(move_crate);
        }
    }
    let p1 = get_answer(&stacks_p1);

    for instruction in instructions.iter() {
        let (from_stack, to_stack) = index_twice(&mut stacks_p2, instruction.from, instruction.to)
            .expect("to/from stack indexes are out of bounds or same index");

        let start_idx = from_stack.len() - instruction.count;
        for move_crate in &from_stack[start_idx..] {
            to_stack.push(*move_crate);
        }
        from_stack.truncate(start_idx);
    }
    let p2 = get_answer(&stacks_p2);

    (p1, p2)
}

fn get_answer(stacks: &Vec<Vec<char>>) -> String {
    stacks
        .iter()
        .filter_map(|stack| stack.last())
        .map(|c| *c)
        .collect()
}

fn index_twice<T>(slc: &mut [T], a: usize, b: usize) -> Option<(&mut T, &mut T)> {
    if a == b || a >= slc.len() || b >= slc.len() {
        return None;
    }

    // safe because a, b are in bounds and distinct
    unsafe {
        let ar = &mut *(slc.get_unchecked_mut(a) as *mut _);
        let br = &mut *(slc.get_unchecked_mut(b) as *mut _);
        Some((ar, br))
    }
}
