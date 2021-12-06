fn main() {
  println!("Hello World!")
}

fn read_file_per_line(file_name: &str) -> Result<Vec<String>, Box<dyn Error>> {
  let input: String = fs::read_to_string(file_name)?.parse()?;
  let vec: Vec<String> = input.split("\n").map(|x| x.to_string()).collect();
  return Ok(vec)
}