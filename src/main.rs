fn main() {

    let x:i32 = 5;
    let y:f32 = 10.001;
    let a = false;
    let mut b = true;
    b = false;

    let greet = String::from("hello");
    let greeting = "good morning";
    
    println!("x = {}",x);
    println!("y = {}",y);
    println!("a = {}",a);
    println!("b = {}",b);
    
    println!("b = {}",b);
    println!("{}",greet);
    println!("{}",greeting);
    let char = greet.chars().nth(0);
    println!("{}",char.unwrap());

}
