use std::fs;
use std::error::Error;

fn main() {
  let vec = read_file_per_line("src/input.txt").unwrap();

  let mut vec_numbers = vec
    .iter()
   .map(|x| x.chars().collect::<Vec<char>>())
   .map(|y| y.iter().map(|num| num.to_digit(10).unwrap()).collect())
   .collect::<Vec<Vec<u32>>>();

  // vec_numbers = vec![
  //   [0,0,1,0,0].to_vec(),
  //   [1,1,1,1,0].to_vec(),
  //   [1,0,1,1,0].to_vec(),
  //   [1,0,1,1,1].to_vec(),
  //   [1,0,1,0,1].to_vec(),
  //   [0,1,1,1,1].to_vec(),
  //   [0,0,1,1,1].to_vec(),
  //   [1,1,1,0,0].to_vec(),
  //   [1,0,0,0,0].to_vec(),
  //   [1,1,0,0,0].to_vec(),
  //   [0,0,0,1,0].to_vec(),
  //   [0,1,0,1,0].to_vec()
  // ];

  let mut result_vec = vec![0; vec_numbers[0].len()];

  for vec in &vec_numbers {
    for i in 0..vec_numbers[0].len() {
      result_vec[i] += vec[i];
    }
  }

  println!("{:?}", result_vec);

  let mut gamma_rate_vec = Vec::new();

  for num in &result_vec {
    if num > &(vec_numbers.len() as u32/ 2_u32){
      gamma_rate_vec.push(1)
    } else {
      gamma_rate_vec.push(0)
    }
  }
  let epsilon_rate_vec = gamma_rate_vec.iter().map(|x| (x - 1_i32).abs()).collect::<Vec<i32>>();

  println!("{:?}", &gamma_rate_vec);
  println!("{:?}", &epsilon_rate_vec);

  println!("Gamma Rate: {:?}", 3808);
  println!("Epsilon Rate: {:?}", 287);
  println!("Result: {:?}", 3808* 287);
}

fn read_file_per_line(file_name: &str) -> Result<Vec<String>, Box<dyn Error>> {
  let input: String = fs::read_to_string(file_name)?.parse()?;
  let vec: Vec<String> = input.split("\n").map(|x| x.to_string()).collect();
  return Ok(vec)
}