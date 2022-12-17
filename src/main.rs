#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use rand::Rng;
#[allow(unused_imports)]
use std::cmp::Ordering;
use std::fs::File;
use std::io::{Write,BufReader,BufRead,ErrorKind};


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

    // for index in 0..arr_1.len(){
    //     println!("index : {} and value is {}",index,arr_1[index]);
    // }


    // ? how to use tuples in rust 

    // let my_tuple: (u32,String,f32) = (34,"akshay".to_string(),3434.45);

    // println!("name is {}",my_tuple.1);
    // #[allow(unused_variables)]
    // let (age,name,package) = my_tuple;
    // println!("my name is {} and i am {}",name,age);


    // ! difference between str and String
    // let mut str1 = String::new();
    // str1.push('A');
    // str1.push_str(" I am Akshay");
    // for word in str1.split(' ') {
    //     println!("{}",word);
    // }
    // let str2 = str1.replace("A", "another");
    // println!("{}",str2);

    // string manipulations 
    // let str3 = String::from("a b c d e f g h i j k l m n o p q r s t u v w x y z");
    // let mut v1: Vec<char> = str3.chars().collect();
    // v1.dedup();
    // v1.sort();
    // for char in v1 {
    //     println!("{}",char);
    // }

    // let str4 = "this is a string";
    // let mut str5 = str4.to_string();
    // println!("{}",str5);

    // let byte_array = str5.as_bytes();
    // how the slicing is done in rust programming langauge 
    // let str6 = &str5[0..5];
    // println!("{}",str6);
    // println!("the length of the string is {}",str6.len());
    // clearing the string 
    // str5.clear();

    // * combining two strings together
    // let str7 = String::from("just some ");
    // let str8 = String::from("words");
    // let str9 = str7 + &str8;
    // // now the str7 should not be called by reference as its need to change but the str8 refrence is needed to add 
    // // in the str9 
    // // println!("{}",str9);
    // for char in str8.bytes(){
    //     println!("{}",char);
    // }

    // * casting in rust programming language 

    // let int_u8:u8 = 45;
    // let int2_u8:u8 = 3;
    // let int_u32:u32 = (int_u8 as u32) + (int2_u8 as u32);

    
    // ? hwo to use enums

    // enum Days {
    //     monday,
    //     tuesday,
    //     wednesday,
    //     thrusday,
    //     friday,
    //     saturday,
    //     sunday,
    // }

    // impl Days {
    //     fn is_weekday(&self) -> bool {
    //         match self {
    //             Days::saturday | Days::sunday => true,
    //             _ => false
    //         }
    //     }
    // }

    // let today:Days = Days::saturday;
    // match today {
    //     Days::monday => println!("boring"),
    //     _ => println!("meh")
    // }
    // match today.is_weekday() {
    //     true => println!("yay weekend"),
    //     _ => println!("ahh fuck no"),
    // }


    // * how to use vectors in rust 

    let mut v1:Vec<i32> = vec![3,4,5,6,7,78,8,];
    // if you want to push a value 
    v1.push(34);
    println!("v1: {:?}", v1);
    let v2 = &v1[1];
    println!("v2: {}", v2);

    match v1.get(2){
        Some(v2) => println!("{}",v2),
        None => println!("no second value"),

    }
    // for loop for vector is 
    for i in 0..v1.len() {
        println!("{}",v1[i]*2);
    }

    for i in &mut v1{
        println!("{}",i);
    }



    

    
}
