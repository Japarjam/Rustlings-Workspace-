//Reference pointers point to a resource in memory

pub fn run() {
  //primitive Array
  let arr1 = [1,2,3];
  let arr2 = arr1; 



// with non-primitives (vectors) when assigning var to a piece of data, first var will no longer hold the value, need to use a reference & to point the resource. IMPL with Borrowing, Ownership, Memory

  // vector - don't forget the vec! before brackets- then we move vec1 sowe need to use & as a reference pointer. 
  let vec1 = vec![1,2,3];
  let vec2 =  &vec1; 

  println!("Values: {:?}",(&vec1, vec2));
  
}