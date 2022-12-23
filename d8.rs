// https://adventofcode.com/2022/day/8

use std::cmp::max;

const BASE_TEN: u32 = 10;

// TODO:
//  - refactor to use new function.
//  - make a separate buffer for visibility.

pub fn solve(input: String) -> (String, String) {
//     let input = "30373
// 25512
// 65332
// 33549
// 35390";

    let mut tree_map = vec![];
    let height = input.chars().take_while(|c| *c != '\n').count();
    for line in input.lines() {
        let mut tree_row = vec![];

        for c in line.chars() {
            tree_row.push(c.to_digit(BASE_TEN).expect(&*format!("number 0-9 (got {})", c)));
        }

        tree_map.push(tree_row);
    }
    let width = tree_map.len();
    let mut vis_map = (0..width).map(|_| vec![false; height]).collect::<Vec<_>>();

    // set all the outer trees to visible
    for i in 0..width {
        vis_map[i][0] = true;
        vis_map[i][height - 1] = true;
    }
    for i in 0..height {
        vis_map[0][i] = true;
        vis_map[width - 1][i] = true;
    }

    // check vertically

    for w in 1..width - 1 {
        set_vis_from(&mut tree_map, &mut vis_map, (w, 0), (0, 1));
        set_vis_from(&mut tree_map, &mut vis_map, (w, height-1), (0, -1));
    }

    // check horizontally
    for h in 1..height - 1 {
        set_vis_from(&mut tree_map, &mut vis_map, (0, h), (1, 0));
        set_vis_from(&mut tree_map, &mut vis_map, (width-1, h), (-1, 0));
    }

    let p1 = vis_map
        .iter()
        .flatten()
        .filter(|tree| **tree)
        .count();

    // let s = vis_map.iter().map(|row| row.iter().map(|b| if *b { 'X' } else { '.' }).collect::<String>() + "\n").collect::<String>();
    // print!("{}", s);

    let mut max_scenic_score = 0;
    for w in 0..width {
        for h in 0..height {
            let starting = (w, h);
            let scenic_score =
            set_vis_from(&mut tree_map, &mut vis_map, starting, (0, 1)) *
            set_vis_from(&mut tree_map, &mut vis_map, starting, (0, -1)) *
            set_vis_from(&mut tree_map,&mut vis_map,  starting, (1, 0)) *
            set_vis_from(&mut tree_map, &mut vis_map, starting, (-1, 0));
            // println!("{},{}: {}", w, h, scenic_score);

            max_scenic_score = max(max_scenic_score, scenic_score);
        }
    }

    (p1.to_string(), max_scenic_score.to_string())
}

/// Sets the visibility for the trees in the direction given by `dir`.
///
/// This does not set the cell `starting`.
///
/// Returns the number of cells marked.
fn set_vis_from(trees: &mut Vec<Vec<u32>>, vis: &mut Vec<Vec<bool>>, starting: (usize, usize), dir: (i32, i32)) -> u32 {
    fn move_pos(v: (usize, usize), inc: (i32, i32)) -> (usize, usize) {
        ((v.0 as i32 + inc.0) as usize, (v.1 as i32 + inc.1) as usize)
    }
    assert_ne!(dir, (0, 0), "direction must not be (0, 0)");

    let mut current_height = 0;
    let mut tree_count = 0;

    let mut last_pos = starting;
    let mut next_pos = move_pos(starting, dir);
    loop {
        let next = trees.get(next_pos.0).map(|row| row.get(next_pos.1)).flatten();
        if next.is_none() {
            break;
        }
        let next = *next.unwrap();
        let last = trees[last_pos.0][last_pos.1];

        let next_vis = &mut vis[next_pos.0][next_pos.1];
        current_height = max(current_height, last);
        if current_height == 9 {
            // max height
            break;
        }

        if next > current_height {
            current_height = next;
            tree_count += 1;
            *next_vis = true;
        }

        // move to next tree
        last_pos = next_pos;
        next_pos = move_pos(next_pos, dir);
    }
    tree_count
}

/// Sets the visibility for the trees in the direction given by `dir`.
///
/// This does not set the cell `starting`.
///
/// Returns the number of cells marked.
fn calc_visible_left(trees: &Vec<Vec<u32>>, vis: &mut Vec<Vec<bool>>, starting: (usize, usize)) -> u32 {
    let mut current_height = 0;
    let mut tree_count = 0;

    let start_x = starting.0;
    let start_y = starting.1;

    for x in 0.. {
        match check_tree(trees, start_x + x, start_y, &mut current_height) {
            // out of bounds; stop counting
            None => break,
            // tree is visible;
            Some(true) => {
                vis[start_x + x][start_y] = true;
                tree_count += 1;
            }
            Some(false) => continue,
        }
        // max height short circuit
        if current_height == 9 {
            break;
        }
    }
    tree_count
}

fn check_tree(trees: &Vec<Vec<u32>>, x: usize, y: usize, current_height: &mut u32) -> Option<bool> {
    let tree = *trees.get(x)?.get(y)?;
    if tree > *current_height {
        *current_height = tree;
        Some(true)
    } else {
        Some(false)
    }
}

fn index_twice_2d<T>(
    slc: &mut [Vec<T>],
    a: (usize, usize),
    b: (usize, usize),
) -> Option<(&mut T, &mut T)> {
    if a == b
        || a.0 >= slc.len()
        || a.1 >= slc[a.0].len()
        || b.0 >= slc.len()
        || b.1 >= slc[b.0].len()
    {
        return None;
    }

    // safe because a, b are in bounds and distinct
    unsafe {
        if a.0 == b.0 {
            let com = slc.get_unchecked_mut(a.0) as *mut Vec<T>;

            let ar = (&mut *com).get_unchecked_mut(a.1);
            let br = (&mut *com).get_unchecked_mut(b.1);

            Some((ar, br))
        } else {
            let ar = (&mut *(slc.get_unchecked_mut(a.0) as *mut Vec<T>)).get_unchecked_mut(a.1);
            let br = (&mut *(slc.get_unchecked_mut(b.0) as *mut Vec<T>)).get_unchecked_mut(b.1);

            Some((ar, br))
        }
    }
}
