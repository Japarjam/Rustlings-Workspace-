pub fn run()  {
  greeting("Sup", "Japar");

// binding functions to variables- made new Var for results of add fn below. goes w/ no semicolon comment
  let get_sum = add(5,5); 
  println!("Sum: {}", get_sum); 

  // CLOSURES- allows to use OUTSIDE Variables ie the n1 and n2 are NOt IN ThIS SCOPE. Can also have in-scope in the fn- n3
  let n3: i32 = 10;
  let add_nums = |n1: i32, n2: i32| n1 + n2 + n3; 
  println!("C Sum:{}", add_nums(3,3)); 
  
}
// params need &type to reference the correct type
fn greeting(greet: &str, name: &str) {
  println!("{} {}", greet, name);
  
}
// want to return an i32, so we use the arrow
fn add(n1: i32, n2: i32) -> i32 {
  n1 + n2
  // no semicolon makes this an expression- returnable- can bind to variable above, too. 
  }