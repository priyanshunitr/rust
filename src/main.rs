fn main() {
    println!("Hello, world!");

    let _x = 5;
    let _y = 10;

    println!("{} {}", _x, _y);

    let greetings = String::from("Hello, Rust!");
    println!(" {}", greetings);


    let char1 = greetings.chars().nth(1000);
    // char1 is an Option<char> because the index may be out of bounds
    // If the index is out of bounds, char1 will be None
    // If the index is within bounds, char1 will be Some(char)


    // Checks whether char1 is Some or None and prints the result accordingly
    println!("Char at index 1000: {:?}", char1);
    match char1 {
        Some(c) => println!("Char at index 1000: {}", c),
        None => println!("No character found at index 1000"),
    }
}

fn get_first_word(sentence : String) -> String{

    let mut ans = String::from("");
    for char in sentence.chars(){
        ans.push_str(char.to_string().as_str());

        if char == ' '{
            break;
        }
    }

    return ans;
}
