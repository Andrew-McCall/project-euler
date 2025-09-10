/* p11:Largest Product in a Grid */
pub fn solution(data: Vec<u64>, height: usize, width:usize, adjacent: usize) -> Result {
  assert!(height * width == data.len());

  let mut biggest = 0;
  let mut biggest_row: Vec<(usize, usize)> = Vec::new();

  for x in 0..width-adjacent {
    for y in 0..height-adjacent {
      let mut diagonal_pos = 1;
      let mut diagonal_pos_row = Vec::new();

      let mut diagonal_neg = 1;
      let mut diagonal_neg_row = Vec::new();

      for j in 0..adjacent {
        let mut current = 1;
        let mut current_row = Vec::new();
        for i in 0..adjacent {
          current *= get_point_value(&data, width, x+i, y+j);
          current_row.push((x+i, y));
        }
        if current > biggest {
          biggest = current;
          biggest_row = current_row;
        }

        diagonal_pos *= get_point_value(&data, width, x+j, y+j);
        diagonal_pos_row.push((x+j, y+j));

        diagonal_neg *= get_point_value(&data, width, x+j, y+adjacent-1-j);
        diagonal_neg_row.push((x+j, y+adjacent-1-j));
      }

      if diagonal_neg > biggest {
        biggest = diagonal_neg;
        biggest_row = diagonal_neg_row;
      }

      if diagonal_pos > biggest {
        biggest = diagonal_pos;
        biggest_row = diagonal_pos_row;
      }
    }
  }

  Result { points: biggest_row.iter().map(|c| get_point(&data, width, c.0, c.1)).collect() }
}

#[test]
fn test() {
  let grid_array = spaced_string_to_array("08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08 49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00 81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65 52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91 22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80 24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50 32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70 67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21 24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72 21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95 78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92 16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57 86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58 19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40 04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66 88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69 04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36 20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16 20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54 01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48");
  let res = solution(grid_array,20,20, 4);
  assert_eq!(res.product(), 70600674);
  assert_eq!(res.points.get(0).unwrap().value, 87);
  assert_eq!(res.points.get(1).unwrap().value, 97);
  assert_eq!(res.points.get(2).unwrap().value, 94);
  assert_eq!(res.points.get(3).unwrap().value, 89);
}

#[test]
fn fun() {
  let grid_array = spaced_string_to_array("08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08 49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00 81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65 52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91 22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80 24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50 32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70 67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21 24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72 21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95 78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92 16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57 86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58 19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40 04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66 88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69 04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36 20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16 20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54 01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48");

  println!("{}", solution(grid_array.clone(), 20, 20, 3));
  println!("{}", solution(grid_array.clone(), 20, 20, 4));
  println!("{}", solution(grid_array.clone(), 20, 20, 5));
  println!("{}", solution(grid_array.clone(), 20, 20, 10));
}

pub fn spaced_string_to_array(input: &str) -> Vec<u64> {
  input.split_whitespace().map(|s|s.parse::<u64>().unwrap()).collect()
}

pub fn get_point_value(data: &[u64], width: usize, x: usize, y: usize) -> u64 {
  let index = y * width + x;
  *data.get(index).unwrap()
}

pub fn get_point(data: &[u64], width: usize, x: usize, y: usize) -> Point {
  Point{x_coordinate: x as u64, y_coordinate: y as u64, value: get_point_value(data, width, x, y)}
}

pub struct Result {
  pub points: Vec<Point>,
}

pub struct Point {
  x_coordinate: u64,
  y_coordinate: u64,
  value: u64,
}

impl Result {
  pub fn product(&self) -> u64 {
    self.points.iter().map(|cv| cv.value).product()
  }
}

impl std::fmt::Display for Result {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let coords_str = self.points.iter().map(|p| format!("{} @ ({}, {})",p.value, p.x_coordinate, p.y_coordinate)).collect::<Vec<_>>().join(", ");
    write!(f, "{} = [{}]", self.product(), coords_str)
  }
}