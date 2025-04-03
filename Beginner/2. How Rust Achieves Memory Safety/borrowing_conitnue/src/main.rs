fn main() {
    
  let mut s1 = String::from("hello");
  let r2 = &mut s1;
  add_to_string(r2);
  println!("s1 is: {s1}");
}

fn add_to_string(s: &mut String)  {
    s.push_str(", world");
    
}

fn get_string(s: &String)-> &String {
    String::from("Ferrari")
    
} // can't return a reference to a local variable