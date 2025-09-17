/* p18: Maximum Path Sum */
pub fn solution(triangle: Vec<u64>) -> u64 {
    assert!(!triangle.is_empty());

    let layer_count = count_layers(triangle.len());

    if layer_count == 1 {
        return *triangle.first().unwrap();
    }

    let last_layer = get_layer(&triangle, layer_count - 1);
    let mut paths: Vec<(u64, u64, Vec<u64>)> = last_layer
        .iter()
        .enumerate()
        .map(|p| (p.0 as u64, *p.1, Vec::new()))
        .collect();

    for l in (0..layer_count - 1).rev() {
        let current_layer = get_layer(&triangle, l);
        println!("Layer: {:?}", current_layer);

        for p in &mut paths {
            println!("{:?}", p);

            let left = if p.0 == 0 {
                0
            } else {
                current_layer[(p.0 - 1) as usize]
            };

            let right = if p.0 == current_layer.len() as u64 {
                0
            } else {
                current_layer[p.0 as usize]
            };

            if right > left {
                p.1 += right;
                p.2.push(right);
            } else {
                p.0 -= 1;
                p.1 += left;
                p.2.push(left);
            }
        }
    }

    println!("Final");
    for p in &mut paths {
        println!("{:?}", p);
    }

    paths
        .iter()
        .reduce(|a, b| if a.0 > b.0 { a } else { b })
        .map(|p| p.1)
        .unwrap()
}

pub const INPUT: &str = "75 95 64 17 47 82 18 35 87 10 20 04 82 47 65 19 01 23 75 03 34 88 02 77 73 07 63 67 99 65 04 28 06 16 70 92 41 41 26 56 83 40 80 70 33 41 48 72 33 47 32 37 16 94 29 53 71 44 65 25 43 91 52 97 51 14 70 11 33 28 77 73 17 78 39 68 17 57 91 71 52 38 17 14 91 43 58 50 27 29 48 63 66 04 68 89 53 67 30 73 16 69 87 40 31 04 62 98 27 23 09 70 98 73 93 38 53 60 04 23";

#[test]
fn test() {
    assert_eq!(solution(string_to_triangle(EXAMPLE)), 23);

    assert_eq!(solution(string_to_triangle(INPUT)), 1064);
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
