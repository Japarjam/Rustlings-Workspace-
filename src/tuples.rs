// Tuples group together calues of different tupes
// max 12 elements in a tuple

pub fn run() {
  let person:(&str, &str, i8) = ("Japar", "PA", 51);
  //meed types &str is a string literal
  println!("{} is from {} and is {} years old", person.0, person.1, person.2); 
}