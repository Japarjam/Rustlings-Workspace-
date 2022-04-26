/* 
Primitive types
integers (u and i) number of bits
Floats
boolean
characters
Tuples - lists
arrays - set lengths list  (-Vectos are mutable arrays)
statically typed- needs to know the type of variable at compiler time- but can usually infer
*/

pub fn run () {
    // find the max of the i32 
  println!("Max i32: {}", std::i32::MAX);
  println!("Max i64: {}", std::i64::MAX);

  let x: i32 = 4534;
  let y: i64 = 100000000000;
  let z: f64 = 3434.111;

  let is_awesome = true;
  // get  boolean from an expression 
  let is_greater: bool = 10 == 5; 

  // single quotes for a char
  let a1 = 'a'; 
  let face = '\u{1F600}'; // \u means unicode 1F600 is emoji
  
  println!("{:?}", (x,y,z,is_awesome, is_greater, a1, face)); 
}