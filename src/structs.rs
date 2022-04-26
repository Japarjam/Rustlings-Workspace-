//Structs are used to create customm data types use caps for Struct names. 
// traditional struct
// struct Color {
  //  red: u8, 
   // green: u8, 
    //blue: u8
 //}

// Tuple Struct- disregard commented out items --indexed 
//truct Color(u8, u8, u8); 

struct Person {
  first_name: String,
  last_name: String
  }

impl Person {
  // construct a person
  fn new(first: &str, last: &str) -> Person {
    Person {
   // because it is str, we need to use .to_str()
      first_name: first.to_string(), 
      last_name: last.to_string()
    }
  }

  // Get Full Name self means this- replace Person - format is macro like println- no ; bc we are returning the format self
  fn full_name(&self) -> String {
    format!("{} {}", self.first_name, self.last_name)
  }

  // Set Last Name
  fn set_last_name(&mut self, last: &str)  {
    self.last_name = last.to_string(); 
  }

  // Set Name to tuple (another method)
fn to_tuple(self) ->(String, String) {
  (self.first_name, self.last_name)
}
   
}

pub fn run() {
 //let mut c = Color {
   //red: 255, 
   //green: 0, 
   //blue: 0
// };

  //c.red = 200; 
   // use dot notation to pull them
  //println!("Color: {} {} {}", c.red, c.green, c.blue); 

 // let mut c = Color(255,0,0); 

//  c.1 = 255; 

 // println!("Color: {} {} {}", c.0, c.1, c.2);


  // need to use ::new since we are adding a new person. 
  let mut p = Person::new("Mary", "Doe"); 
    println!("Person {}", p.full_name());
    p.set_last_name("Williams"); 
 // println!("Person {} {}", p.first_name, p.last_name);
  println!("Person {}", p.full_name());
  println!("Person Tuple {:?}", p.to_tuple());
  // since it is a tuple, don'g forget debug {:?}
}