fn main() {
    let datastream = include_str!("input.txt");
    let start_of_packet = find_marker(datastream, 4);
    println!("{start_of_packet}");
    let start_of_message = find_marker(datastream, 14);
    println!("{start_of_message}");
}

fn find_marker(datastream: &str, size: usize) -> usize {
    let mut bytes_head = datastream.bytes().map(|b| b.to_index());
    let bytes_tail = bytes_head.clone();
    let mut counter = bytes_head.by_ref().take(size).fold([0; 26], |mut acc, b| {
        acc[b] += 1;
        acc
    });
    bytes_tail
        .zip(bytes_head)
        .position(|(tail, head)| {
            counter[head] += 1;
            counter[tail] -= 1;
            counter.iter().all(|&v| v <= 1)
        })
        .map(|i| i + size + 1)
        .expect("marker not found")
}

trait ToIndex {
    fn to_index(&self) -> usize;
}

impl ToIndex for u8 {
    fn to_index(&self) -> usize {
        (*self - b'a') as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_test_start_of_packet() {
        assert_eq!(find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
        assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }

    #[test]
    fn find_test_start_of_message() {
        assert_eq!(find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
        assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }
}
