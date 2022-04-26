// Variables hold primitive data or reference to data (Ownership)
//Variables are immutable by default (use mut)
// Rust is a block scoped lang

pub fn run () {
  let name = "Japar";
  let mut age = 51;

  println!(" My name is {}, and I am {}", name, age); 

  // define constants
  const ID: i32 = 515151; 
    println!("ID: {}", ID);

  // Multiple Variables at once
  let (my_name, my_age) = ("Japar", 51);
  println! ("My name is {}, and my age is {}", my_name, my_age); 
}