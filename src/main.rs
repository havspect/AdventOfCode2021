use std::fs;
use std::error::Error;

#[derive(Debug)] 
enum Direction{
  Forward,
  Down,
  Up,
}

#[derive(Debug)] 
struct Command{
  Direction: Direction,
  Steps: i32,
}

fn main() {
  let vec = read_file_per_line("src/input.txt").unwrap();

  let mut command_vec: Vec<Command> = Vec::new();

  for line in &vec {
    let res_dir = match line.split(" ").collect::<Vec<&str>>()[0] {
      "forward" => Direction::Forward,
      "down" => Direction::Down,
      "up" => Direction::Up,
      _ => Direction::Forward,
    };
    let res_num = line
      .split(" ")
      .collect::<Vec<&str>>()[1]
      .parse::<i32>()
      .unwrap();

    command_vec.push(Command{Direction: res_dir, Steps: res_num})
  };

  let mut x = 0;
  let mut y = 0;
  for command in &command_vec {
    match command.Direction {
      Direction::Forward => x += command.Steps,
      Direction::Up => y -= command.Steps,
      Direction::Down => y += command.Steps,
      _ => x = x,
    };
  }

  let mut x_2 = 0;
  let mut y_2 = 0;
  let mut aim = 0;
  for command in &command_vec {
    match command.Direction {
      Direction::Forward => {
        x_2 += command.Steps;
        y_2 += aim * command.Steps;
      },
      Direction::Up => aim -= command.Steps,
      Direction::Down => aim += command.Steps,
      _ => x = x,
    };
  }

  println!("x-Position: {:#?}", x);
  println!("y-Position: {:#?}", y);
  println!("Result: {:#?}", x*y);

  println!("x-Position: {:#?}", x_2);
  println!("y-Position: {:#?}", y_2);
  println!("Result: {:#?}", x_2*y_2);
}



fn read_file_per_line(file_name: &str) -> Result<Vec<String>, Box<dyn Error>> {
  let input: String = fs::read_to_string(file_name)?.parse()?;
  let vec: Vec<String> = input.split("\n").map(|x| x.to_string()).collect();
  return Ok(vec)
}