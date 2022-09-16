//use std::io;
//use std::io::prelude::*;
//use std::char;
use std::str::Split;
fn main() {
    
    //let input = String::from("5 1 3 2 5 4"); 
    //let mut chars: Vec<char> = input.chars().collect();
    let mut given_numbers: Vec<u32> = Vec::new(); 
    
    //let input: &str = "5 1 3 2 5 4";
    let mut input1 = String::new();
    
    std::io::stdin().read_line(&mut input1).expect("Failed to read line"); //read_line(&mut line).unwrap();
    //println!("Hello , {}", input);

    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).expect("Failed to read line");
    let mod_input = &input2[0..].trim();
    let given_numbers_as_strings: Split<&str> = mod_input.split(" ");
    

    for current_number_as_string in given_numbers_as_strings {
        let current_number: u32 = current_number_as_string.trim().parse().expect("Please type a number!");
        given_numbers.push(current_number);
        //println!("Number {}", current_number);
    }

    //io::stdin().read_line(&mut guess).expect("Failed to read line");
    /*
    
    for someth in 0..chars[0].to_string().trim().parse().expect("Please type a number!") {
        let mut index = start + (2 * someth);
        //println!("{}", index);
        let mut current_char_as_string = chars[index as usize].to_string();
        let current_number: u32 = current_char_as_string.trim().parse().expect("Please type a number!");
        given_numbers.push(current_number);
        println!("{}", current_number);
    }
    */
    given_numbers.sort();
    if (given_numbers.len() % 2) == 0 {
        let start = given_numbers.len() / 2;
        let mut sum = 0;
        for i in start..(given_numbers.len()) {
            sum = sum + given_numbers[i as usize];
        }
        println!("{}", sum);
    } else {
        let start = (given_numbers.len()-1) / 2;
        //println!("{}", start);
        let mut sum = 0;
        for i in start..(given_numbers.len()) {
            sum = sum + given_numbers[i as usize];
        }
        println!("{}", sum);
    }
    //println!("{}", string);

    
    /*
    let mut input = String::new();
    
    
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("{}", input);
        },
        Err(e) => println!("Oops something went wrong: {}", e)
    }
    
    //get starting position i of first number, 
    let start = 1;

    //loop through every number with i = i + 2, convert char to int, append to a vector
    let mut given_numbers: Vec<i32> = Vec::new();
    let a = String::from("55 3 2 1 1");
    let chars: Vec<_> = a.chars().collect();
    //chars.retain(|&x| x != '');
    /*
    for ch in a.chars() {
        println!("{}", ch);
    }
    println!("{}", chars.len());
    */
    
    let nchar = chars[0];
    let nint = nchar.to_digit(10).unwrap();
    println!("{}", nint);
    
    //.t::<i32>().unwrap();
    
    
    for i in 0..nint {
        let charschar = chars[(start+2*i) as usize];
        let charsint:<i32> = charschar.to_digit(10).unwrap();
        given_numbers.push(charsint);
    }

    //convert vector to array
    //sort array
    
    given_numbers.sort();
    println!("{:?}", given_numbers);

    //if statement depending on odd or even, ergo, get the last whatever numbers and add them
    //print sum
    
    */
}
