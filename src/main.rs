use std::io;
#[allow(unused_imports)]
use rand::Rng;

fn main() {
    // let wish = "hello world";
    // println!("{}", wish);

    // *lets see how would you work if the user input 
    // let mut name = String::new();
    // let greeting = "hello nice to meet you";
    // println!("enter your name: ");
    // io::stdout().flush();
    // io::stdin().read_line(&mut name)
    //     .expect("didnt recieved the proper input");
        
    // println!("{} {}", greeting,name.trim_end());

    // ? now lets see how the different data types are done 
    // const ONE_MIL:u32 = 1000000;
    // let mut age = 32;
    // let age = "32";
    // let mut age: u32 = age.trim().parse()
    //     .expect("age wast assigned a number");
    // age = age +1;
    // println!("i am {} and i want {}",age,ONE_MIL);


    // ! how can you implement a random number generator 

    // let random_num = rand::thread_rng().gen_range(1..101);
    // println!("Random : {}", random_num);

    // * how to use if statements in rust 
    let mut age = String::new();
    #[allow(unused_macros)]
    io::stdin().read_line(&mut age)
        .expect("error");
    let age:i32 = age.trim().parse()
        .expect("error");
    if age < 18{
        println!("you can not vote yet");
    }
    else {
        println!("you can vote");
    }

    
}
