use std::fs;
use std::env;
use regex::Regex;
use itertools::Itertools;
//John Bedette, Dominant letter counter in Rust using Regex and Itertools. 


//simple tuple for storing a letter and a number
#[derive(Debug)]
struct DomLet{
    ltr: char,
    num: u32,
}


fn main() {

    //get file name from args, check if correct args
    let file_name: Vec<String> = env::args().collect();
    match file_name.len(){
        1 => println!("\n\nNo file name entered\n"),
        2 =>{ 
            //read to string from file
            let phrase = fs::read_to_string(file_name[1].clone());

            //on Ok, destructure and do program, on fail catch err and display a message
            match phrase{
                Ok(phrase)=> {

                    //set regex and dom letter counter
                    let reg = Regex::new(r"(^[a-z]+[a-z]*$)").unwrap();
                    let mut count: u32 = 0; 


                    //take text from file, reduce to lowercase, 
                    //replace newlines with spaces, split on whitespace
                    for word in phrase.clone().to_lowercase().replace('\n', " ").split(' '){

                        //regex to eliminate non a-z words
                        if reg.is_match(word){

                            //tuples for tracking 
                            let mut most = DomLet {ltr : '!', num : 0};
                            let mut curr = DomLet {ltr : '!', num : 0};

                            //Sort word by ascii val of chars, iterate over the chars
                            for c in word.chars().sorted(){

                                //if current != to c in word, set current to track new char freq
                                if curr.ltr != c {
                                    curr.ltr = c;
                                    curr.num = 1;
                                }
                                //increment if the same char
                                else {
                                    curr.num += 1;
                                }

                                //if curr letter/num tuple is has higher num than most, update most to curr
                                if curr.num > most.num {
                                    most.ltr = curr.ltr;
                                    most.num = curr.num;
                                }
                            }
                            //increm count by most num, curr and most should be dropped
                            count += most.num;
                        }
                    }

                    //output and return
                    println!("\nFile: {}\n\nText:\n {}\n\nDominant Letter Count: {}\n",
                        file_name[1].clone(), phrase.clone(), count)
                },
                Err(_)=>println!("\n\nFile not found, please try again\n\n")
            }
        },
        _ => println!("\n\nOnly one file please\n\n")
    }
}
