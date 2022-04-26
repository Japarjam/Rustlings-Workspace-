// fixed lists of elements of the same data types

use std::mem;

pub fn run() {
  // name it, then put [type; length]
  let mut numbers: [i32; 5] = [1,2,3,4,5]; 

  // can't change the length of the array, but can reassign a value- make array mut above
  numbers[2] = 77;
  

  // Use the debug for printing arrays :?
  println!("{:?}", numbers);

  // get a single value
  println!("Single Value is: {}", numbers[1]);

  // get array length
  println!("Array Length is: {}", numbers.len());

// Arrays are STACK allocated memory can add STD up top and remove here
  println!("Array occupies {} bytes", mem::size_of_val(&numbers)); 

  // get slices need & for function type signature and the type receiving it- also need :? debug because it's an array being printed- need to add RANGE [0..2]
  let slice: &[i32] = &numbers[0..2];
  println!("Slice {:?}", slice); 
  
}