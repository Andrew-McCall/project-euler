/* p18: Maximum Path Sum I */
pub fn solution(triangle: Vec<u64>) -> Result {
    let mut path = vec![triangle[0]];
    let mut current_x = 0;

    for l in 1..count_layers(triangle.len()) {
        let layer = get_layer(&triangle, l);

        if layer[current_x] > layer[current_x + 1] {
            path.push(layer[current_x]);
        } else {
            path.push(layer[current_x + 1]);
            current_x += 1;
        }
    }

    Result {
        sum: path.iter().sum(),
        path,
    }
}

pub const INPUT: &str = "75 95 64 17 47 82 18 35 87 10 20 04 82 47 65 19 01 23 75 03 34 88 02 77 73 07 63 67 99 65 04 28 06 16 70 92 41 41 26 56 83 40 80 70 33 41 48 72 33 47 32 37 16 94 29 53 71 44 65 25 43 91 52 97 51 14 70 11 33 28 77 73 17 78 39 68 17 57 91 71 52 38 17 14 91 43 58 50 27 29 48 63 66 04 68 89 53 67 30 73 16 69 87 40 31 04 62 98 27 23 09 70 98 73 93 38 53 60 04 23";

#[test]
fn test() {
    assert_eq!(
        solution(string_to_triangle(EXAMPLE)),
        Result {
            sum: 23,
            path: vec![3, 7, 4, 9]
        }
    );

    assert_eq!(solution(string_to_triangle(INPUT)).sum, 1064);
}

#[derive(Debug, Eq, PartialEq)]
pub struct Result {
    pub sum: u64,
    pub path: Vec<u64>,
}

impl std::fmt::Display for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} <= {}",
            self.sum,
            self.path
                .iter()
                .rev()
                .map(u64::to_string)
                .collect::<Vec<_>>()
                .join(" <- ")
        )
    }
}

fn count_layers(c: usize) -> usize {
    let n = ((1.0 + 8.0 * c as f64).sqrt() - 1.0) / 2.0;
    n.floor() as usize
}

fn get_layer(triangle: &[u64], layer: usize) -> &[u64] {
    let start = layer * (layer + 1) / 2;
    &triangle[start..start + layer + 1]
}

pub const EXAMPLE: &str = "3 7 4 2 4 6 8 5 9 3";

#[test]
fn test_get_layer() {
    let triangle = string_to_triangle(EXAMPLE);

    assert_eq!(get_layer(&triangle, 0), &[3]);
    assert_eq!(get_layer(&triangle, 1), &[7, 4]);
    assert_eq!(get_layer(&triangle, 2), &[2, 4, 6]);
    assert_eq!(get_layer(&triangle, 3), &[8, 5, 9, 3]);
}

pub fn string_to_triangle(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}
