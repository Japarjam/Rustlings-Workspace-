// primitive "str" are immutable fixed=len somewheere in mem
// "String" = growable, heap allocated data structure Use when string data needs to be modified. Like a Vector

pub fn run() {
  let hello = "hello "; //str 
  let mut hola = String::from("hola ");

    
  println!("{} is {}", hello, hola); 
  
  hola.push('J'); //for String only one CHAR
  hola.push_str("aparJam"); // for str need _ for a string
  // these are METHODS with the dot notation and ()
  println!("Capacity: {}", hola.capacity());
// Is empty
  println!("Is Empty{}", hola.is_empty()); 
// contains a word  
  println!("Contains {}", hola.contains("Jap")); 
// Replace 
  println!("we can replace words like {}", hola.replace("JaparJam", "Snap"));

//Loop through strings by whitespace- this splits it at the whitespace 
  for word in hola.split_whitespace() {
    println!("{}", word);
  }
//DOT NOTATION WITH () ARE METHODS WITH ARGUMENTS
// Create a string- as in an address or a hash? 
  let mut s = String::with_capacity(10);
  s.push('a');
  s.push('b');

  println!("{}", s); 

// Assertion Testing "assert equal" 
  assert_eq!(2, s.len()); // won't print anything if it passes/ will fail- panick if not 
  assert_eq!(10, s.capacity()); //tests the capacity of the S
  
  
  // get lwngth (REMEMBER () is needed) 
  println!("{}", hola.len());
  println!("{}", hola);
}