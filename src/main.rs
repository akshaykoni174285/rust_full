#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use rand::Rng;
#[allow(unused_imports)]
use std::cmp::Ordering;


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
    // let mut age = String::new();
    // #[allow(unused_macros)]
    // io::stdin().read_line(&mut age)
    //     .expect("error");
    // let age:i32 = age.trim().parse()
    //     .expect("error");
    // if age < 18{
    //     println!("you can not vote yet");
    // }
    // else {
    //     println!("you can vote");
    // }


    // * how to use match in rust 

    // let age2 = 7;
    // match age2 {
    //     1..=18 => println!("imp bday"),
    //     21 | 50 => println!("imp bday"),
    //     60..=i32::MAX => println!("imp bday"),
    //     _ => println!("not a imp bday"),
    // };

    // * using match for two variable and comparing the value 

    // let my_age = 18;
    // let age_to_vote = 18;
    // match my_age.cmp(&age_to_vote) {
    //     Ordering::Less => println!("cant vote"),
    //     Ordering::Greater => println!("you can vote"),
    //     Ordering::Equal => println!("you gained the right to vote now"),

    // };


    // ? now lets talk about the arrays 
    // in rust when using arrays you have to keep the same datatype throughout the array 
    // also they have fixed size 
    
    // * initialize

    let arr_1 = [1,2,3,4,4,5];
    // println!("first item :{}",arr_1[0]);
    // println!("length is : {}",arr_1.len());

    // ? how to print the contents of the array 
    // ! using a debug method
    // println!("array is {:?}",arr_1);

    // * 1) using loop 
    // let array_indx = 0;
    // loop {
    //     if arr_1[array_indx]%2==0 {
    //         array_indx+=1;
    //         continue;

    //     }
    //     if arr_1[array_indx] == 5 {
    //         break;
    //     }
    //     println!("{}",arr_1[array_indx]);
    //     array_indx+=1;
    // }
    // * 2) using while loop 

    // while arr_1[array_indx] != 5 {
    //     if arr_1[array_indx] %2==0{
    //         array_indx+=1;
    //         continue;
    //     }
    //     println!("{}",arr_1[array_indx]);
    //     array_indx+=1;
    // }

    // * 3) for loop
    // for val in arr_1.iter(){
    //     println!("{}",val);
    // }

    for index in 0..arr_1.len(){
        println!("index : {} and value is {}",index,arr_1[index]);
    }


    

    
}
