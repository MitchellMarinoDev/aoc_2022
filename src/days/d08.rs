// https://adventofcode.com/2022/day/8

use std::cmp::max;
use std::isize;

const BASE_TEN: u32 = 10;

// TODO:
//  - refactor to use new function.
//  - make a separate buffer for visibility.

pub fn solve(input: String) -> (String, String) {
    let mut tree_map = vec![];
    let height = input.chars().take_while(|c| *c != '\n').count();
    for line in input.lines() {
        let mut tree_row = vec![];

        for c in line.chars() {
            tree_row.push(
                c.to_digit(BASE_TEN)
                    .expect(&*format!("number 0-9 (got {})", c)),
            );
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
        calc_visible(&tree_map, &mut vis_map, (w, 0), (0, 1));
        calc_visible(&tree_map, &mut vis_map, (w, height - 1), (0, -1));
    }

    // check horizontally
    for h in 1..height - 1 {
        calc_visible(&tree_map, &mut vis_map, (0, h), (1, 0));
        calc_visible(&tree_map, &mut vis_map, (width - 1, h), (-1, 0));
    }

    let p1 = vis_map.iter().flatten().filter(|tree| **tree).count();

    let mut max_scenic_score = 0;
    for x in 0..width {
        for y in 0..height {
            let starting = (x, y);
            let scenic_score = view_dist(&mut tree_map, starting, (0, 1))
                * view_dist(&mut tree_map, starting, (0, -1))
                * view_dist(&mut tree_map, starting, (1, 0))
                * view_dist(&mut tree_map, starting, (-1, 0));

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
fn calc_visible(
    trees: &Vec<Vec<u32>>,
    vis: &mut Vec<Vec<bool>>,
    starting: (usize, usize),
    dir: (isize, isize),
) {
    let mut current_height = 0;

    let start_x = starting.0 as isize;
    let start_y = starting.1 as isize;

    for dist in 0.. {
        let x = dist * dir.0 + start_x;
        let y = dist * dir.1 + start_y;
        match check_tree_height(trees, x, y, current_height) {
            TreeHeightComp::OutOfBounds => break,
            TreeHeightComp::Taller => {
                vis[x as usize][y as usize] = true;
                current_height = trees[x as usize][y as usize];
            }
            _ => continue,
        }
        // max height short circuit
        if current_height == 9 {
            break;
        }
    }
}

fn view_dist(trees: &Vec<Vec<u32>>, starting: (usize, usize), dir: (isize, isize)) -> u32 {
    use TreeHeightComp::*;

    let start_x = starting.0 as isize;
    let start_y = starting.1 as isize;
    let height = trees[starting.0][starting.1];

    for dist in 1.. {
        let x = dist * dir.0 + start_x;
        let y = dist * dir.1 + start_y;
        match check_tree_height(trees, x, y, height) {
            Shorter => continue,
            SameHeight | Taller => return dist as u32,
            OutOfBounds => return (dist - 1) as u32,
        }
    }
    panic!("reached {} without hitting the edge of the map", isize::MAX);
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum TreeHeightComp {
    OutOfBounds,
    Taller,
    Shorter,
    SameHeight,
}

/// Checks a certain tree to see if it is:
/// 1. In bounds
/// 2. Taller than the `height`
///
/// ### Returns
/// - `None` if `x` and `y` are out of bounds.
/// - `Some(true)` if the tree at `x` and `y` is taller than `height`.
/// - `Some(false)` if the tree at `x` and `y` is shorter or the same height as `height`.
fn check_tree_height(trees: &Vec<Vec<u32>>, x: isize, y: isize, height: u32) -> TreeHeightComp {
    if x.is_negative()
        || y.is_negative()
        || x >= trees.len() as isize
        || y >= trees[x as usize].len() as isize
    {
        return TreeHeightComp::OutOfBounds;
    }

    let tree = trees[x as usize][y as usize];
    if tree == height {
        TreeHeightComp::SameHeight
    } else if tree > height {
        TreeHeightComp::Taller
    } else {
        TreeHeightComp::Shorter
    }
}
