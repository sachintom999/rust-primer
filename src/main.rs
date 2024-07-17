fn main() {
    let x: i32 = 5;
    let y: f32 = 10.001;
    let a = false;
    let mut b = true;
    println!("b = {}", b);
    b = false;

    let is_even = false;

    let greet = String::from("hello");
    let greeting = "good morning";

    println!("x = {}", x);
    println!("y = {}", y);
    println!("a = {}", a);
    println!("b = {}", b);

    println!("{}", greet);
    println!("{}", greeting);
    let char = greet.chars().nth(0);
    println!("{}", char.unwrap());

    if is_even {
        println!("number is even")
    } else {
        println!("num is not even")
    }

    for i in 0..11 {
        print!("{} ", i)
    }

    let sentence = String::from("john doe is the name..");
    let first_word = get_first_word(sentence);
    println!("");
    println!("First word ::{}", first_word);
    let sum = add(10, 15);
    println!("sum = {}", sum);

    update_string()
}

fn get_first_word(sentence: String) -> String {
    let mut first_word = String::from("");
    for char in sentence.chars() {
        if char == ' ' {
            break;
        }
        first_word.push_str(char.to_string().as_str());
    }
    return first_word;
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn update_string() {
    let mut s = String::from("initial string");
    for _ in 1..100 {
        s.push_str("this is additional data");
        println!(
            "length:{} capacity:{} pointer:{:p}",
            s.len(),
            s.capacity(),
            s.as_ptr()
        )
    }
}



/*


Ownership :: scenario-1

fn main() {
    let greet = String::from("hello");
    println!("{}", greet);
    take_ownership(greet);
    
}

fn take_ownership(temp: String) -> String {
    println!("in take_ownership () {}", temp);
    return temp;
}



Ownership :: scenario-2

fn main() {
    let mut greet = String::from("hello");
    println!("{}", greet);
    greet = take_ownership(greet);
    println!("{}", greet);
}

fn take_ownership(temp: String) -> String {
    println!("in take_ownership () {}", temp);
    return temp;
}

*/
