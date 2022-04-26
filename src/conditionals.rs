// Conditionals check the conditions and act on the result of the chck

pub fn run () {
  let age: u8 = 22;
  let check_id: bool = true;
  let knows_person_of_age = true;

  if age >= 21 && check_id || knows_person_of_age  {
    println!("enjoy the cocktails");
      } else if age < 21 && check_id {
    println!("not quite yet, kiddo"); 
      } else {
    println!("Can I see your ID?"); 
      }

  //Shorthand Version 
  let is_of_age = if age >= 21 {true} else {false};
  println!("Is of Age: {}", is_of_age); 
  
}