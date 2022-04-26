// Loops are used to iterate until conditions are met

pub fn run ()  {
// Infinite loop sample
  let mut count = 0;

 //loop {
   // count += 1;
    //println!("Number: {}", count);

    //if count == 20 {
      //break;
    //}
 //}
  
  // While Loop (FizzBuzz)
//while count <= 100 {
  //if count % 15 == 0 {
   // println!("FizzBuzz");
 // } else if count % 3 == 0 {
   // println!("Buzz");
  //} else if count % 5 == 0 {
    //println!("Fizz");
  //} else {
   // println!("{}", count);
  //}

  //increment
 // count += 1; 

  // for Range Loop
  for x in 0..100 {
      if x % 15 == 0 {
    println!("FizzBuzz");
  } else if x % 3 == 0 {
    println!("Buzz");
  } else if x % 5 == 0 {
    println!("Fizz");
  } else {
    println!("{}", x);
  }

  }
  
}  
  
