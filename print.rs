pub fn run() {
  println! ("Hello from TDX");

    println!("{}", 1); // what's inside "" is string literal
    println!("{} is from {}", "Japar", "Philly"); 

  // Positional Arguments use index
  println!("{0} is from {1} and {0} enjoys {2}", "Japar", "PA", "code"); 

  //named arguments
  println!("{name} likes {activity}", name= "Japar", activity = "Coding"); 

  // Placeholder Traits- Binary, Hex, ,Octal 
  println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10); 

// Placeholder for Debug Trait TUPLE 
  println!("{:?}", (12, true, "Hola")); 

// Basic Math
  println!("10 + 10 = {}", 10 + 10);
}