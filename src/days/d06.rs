// https://adventofcode.com/2022/day/6

use itertools::Itertools;

pub fn solve(input: String) -> (String, String) {
    let chars = input.chars().collect::<Vec<_>>();

    let packet_marker = find_marker(&chars, 4).expect("No start of packet marker");
    let message_marker = find_marker(&chars, 14).expect("No start of message marker");

    (packet_marker.to_string(), message_marker.to_string())
}

/// Finds and returns a marker of length `length` in the given chars
fn find_marker(chars: &[char], length: usize) -> Option<usize> {
    let marker_idx = chars
        .windows(length)
        .enumerate()
        .filter(|(_i, chars)| chars.iter().unique().count() == length)
        .next()?
        // add `length` to get the index of the end
        .0
        + length;

    Some(marker_idx)
}
