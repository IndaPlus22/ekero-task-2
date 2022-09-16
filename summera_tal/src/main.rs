use std::str::Split;
fn main() {
    let mut given_numbers: Vec<u32> = Vec::new(); 
    

    let mut input1 = String::new();
    
    std::io::stdin().read_line(&mut input1).expect("Failed to read line"); //read_line(&mut line).unwrap();
   

    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).expect("Failed to read line");
    let mod_input = &input2[0..].trim();
    let given_numbers_as_strings: Split<&str> = mod_input.split(" ");
    

    for current_number_as_string in given_numbers_as_strings {
        let current_number: u32 = current_number_as_string.trim().parse().expect("Please type a number!");
        given_numbers.push(current_number);
    }
    
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
    
}
