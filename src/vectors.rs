// VECTORS are resizable Arrays

use std::mem;

pub fn run() {
  // use Vec<data type> and vec! bwfore DATA 
  let mut numbers: Vec<i32> = vec![1,2,3,4,5]; 

  // can reassign a value- 
  numbers[2] = 77;
  // add to the Vector with push
  numbers.push(66);
  numbers.push(55);

  // can also POP off last number
  numbers.pop();
  
  // Use the debug for printing VECTORS :?
  println!("{:?}", numbers);

  // get a single value
  println!("Single Value is: {}", numbers[1]);

  // get VECTOR length
  println!("Vector Length is: {}", numbers.len());

// VECTORS are STACK allocated memory can add STD up top and remove here
  println!("Vector occupies {} bytes", mem::size_of_val(&numbers)); 

  // get slices need & for function type signature and the type receiving it- also need :? debug because it's an vector being printed- need to add RANGE [0..2]
  let slice: &[i32] = &numbers[0..2];
  println!("Slice {:?}", slice); 

  // Can Loop Through the Vector values with dot iter (iteration)
  for x in numbers.iter() {
    println!("Number: {}", x); 
  }
  // Can also mutate values here we multiply by 2
  for x in numbers.iter_mut() {
    *x *= 2;
  }

    println!("Numbers: {:?}", numbers);
    
  }
  
  
