#![allow(warnings)]

use std::vec;

fn main() {
    // let mut input  = String::new();
    // std::io::stdin().read_line(&mut input).expect("Failed to read line");
    // println!("{}", input);
    // let x = 127_000 as i64;
    // let y  = 10_i32;

    // let z = x / (y as i64);
    // println!("{}", z);
    // let mut input  = String::new();
    // std::io::stdin().read_line(&mut input).expect("Expected to read line");

    // let int_input: i64 = input.trim().parse().unwrap();

    // println!("{}", int_input + 2)
    // let food  = "bread";

    // if food == "cookie" {
    //     println!("i like {} too", food)
    // } else if food == "bread" {
    //     println!("mummy's boy")
    // }else if food == "akpu" {
    //     println!("cool guy")
    // }else {
    //      println!("oh that's too bad")
    // }
    // test();
    // 
    // loop
    // let mut counter = 0;
    // let result = 
    // loop {
    //     counter+=1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    // println!("the result is {}", result)
    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }
    //     count += 1;
    // }
    // println!("end of count = {count}")
    // while loop
    // let mut number = 3;

    // while number != 0 {
    //     println!("{number}");
    //     number -= 1;
    // }
    // println!("LIFTOFF!!!");

    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;

    // while index < a.len() {
    //     println!("the value is : {}", a[index]);

    //     index += 1;
    // }
    // for number in (1..4).rev() {
    //     println!("{number}!")
    // }
    // println!("LIFTOFF!!!")

    // fibonacci(10);
    // chrismac_carol();
    // let s1 = String::from("hello");
    
    // let len = calculate_length(&s1);
    // println!("the length of '{}' is '{}'", s1, len);
    // let mut s1 = String::from("Hello");
    // let s2 = s1;
    // s1 = String::from("value");
    // println!("{}", s1);
    // let mut _x = 5;
    // let  _r = &mut _x;
    // *_r += 1;
    
    // println!("{}", _r);
    // let mut account  = BankAccount{
    //     owner: "George-Goldman".to_string(),
    //     balance: 1_000_000_000.0
    // };
    // account.withdraw(20.090);
    // account.check_balance();
    // let mut a = 5;
    // println!("the value of a is {}", a);
    // a = 6;
    // const Y:i32 = 10;
    // let x = 5;
    // let x = x +1;
    // let x = x +1;
    // println!("{}", x)
    // let spaces = "      ";
    // let spaces = spaces.len();
    // println!("{}", spaces);
    // struct Book {
    //     title: String,
    //     author: String,
    //     pages: u32,
    //     availability: bool,
    // }

    // struct User {
    //     active: bool,
    //     username: String,
    //     email: String,
    //     sign_in_count: u64
    // }

    // let mut user1 = User {
    //     active: true,
    //     username: "george-goldman".to_string(),
    //     email: "george-goldman@email.com".to_string(),
    //     sign_in_count: 100
    // };

    // user1.email = String::from("anotheremail@email.com");
    // // println!("{}", user1.email)
    // fn build_user(email: String, username: String) -> User {
    //      User { active: true, username, email, sign_in_count: 1}
    // }

    // let user2 = User {
    //     email: String::from("another@m.com"),
    //     ..user1
    // };
    // // println!("{}", user2.active);

    // // tuple struct 
    // struct Color(i32, i32, i32);
    // struct Point(i32, i32, i32);

    // // unit struct 
    // struct AlwaysEqual;
    // let subject = AlwaysEqual;

    // enum IpAddrKind {
    //     v4(u8, u8, u8, u8),
    //     v6(String)
    // }
    // let four  = IpAddrKind::v4;
    // let six  = IpAddrKind::v6;

    // // fn route(ip_kind: IpAddrKind){}
    // // route(IpAddrKind::v4(String::from("value")));
    // // route(IpAddrKind::v6(String::from("value")));
    // let home = IpAddrKind::v4(127,0,0,1);
    // let loopback = IpAddrKind::v6(String::from("::1"));

    

// let mut _v: Vec<i32> = vec![-2, -1, 0];

// _v.push(1);
// _v.push(2);
// _v.push(3);
// _v.push(4);
// _v.push(5);
// _v.push(6);
// _v.push(7);
// _v.push(8);
// _v.push(9);
// _v.push(10);

// println!("{:?}", _v);

let _v = vec![1,2,3,4,5,6];

let third = _v.get(2);
match third {
    Some (third) => println!("the third element for get is {third}"),
    None => println!("There is no third element"),
}


}




enum Result<T, E> {
    Ok(T),
    Err(E)
}


fn dividOption(numerator: f64, denominator: f64)-> Option<f64>{
    if denominator == 0.0 {
        None
    }else {
        Some(numerator/denominator)
    }
}

// fn divideResult(numerator: f64, denominator: f64) -> Result<f64, String> {
//     if denominator == 0.0 {
//         Err("Can not divide by 0".to_string());
//     } else {
//          Ok(numerator / denominator)
//     }
// }

struct  BankAccount {
    owner: String,
    balance: f64
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64){
        println!("Withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self){
        println!("Account owned by {} has a balance of {}", self.owner, self.balance)
    }
}

fn temp_convert() -> f32 {
    println!("please 1 for Fahrenheit to Celsius\n 2 for Celsius to Fahrenheit");
    let mut choice = String::new();
    let mut temp = String::new();
    std::io::stdin().read_line(&mut choice).expect("expected a read line");
    println!("{choice}");

    
    if choice.trim() == "1" {
        print!("enter a Fahrenheit value: ");
        std::io::stdin().read_line(&mut temp).expect("expected a read line");
        let temp_to_int: f32 = temp.trim().parse().unwrap();
        (temp_to_int - 32_f32) * (5/9) as f32
    }else if choice.trim() == "2" {
        print!("enter a Celsius: ");
        std::io::stdin().read_line(&mut temp).expect("expected a read line");
        let temp_to_int: f32 = temp.trim().parse().unwrap();
        temp_to_int * (9/5) as f32 + 32_f32
    }else {
        0.0
    }
}

fn fibonacci(mut x: i32) -> i32{
    if x == 0 {
        x
    }else {
        x +=  fibonacci(x -1);
        println!("{x}");
        x
    }
}

fn chrismac_carol(){
    let mut i = 0;
    let lyrics = [
        "Six geese a-laying,",
        "Five golden rings,",
        "Four calling birds,",
        "Three French hens,",
        "Two turtle doves",
        "a partridge in a pear tree.",
    ];
    let mut stanser = String::new();
    loop {
        i+=1;
        // let mut chorus = String::new();
        
        let chorus = "On the ".to_string() +  i.to_string().as_str()+ if i == 1
         {"st"} else if i == 2 
         {"nd"} else if i == 3 
         {"rd"} else 
         {"th"}  +" day of Christmas\nMy true love gave to me";
        stanser = lyrics[lyrics.len() - i].to_owned() + &stanser;
        println!("{chorus}");
        println!("{stanser}\n" );
        
        
        if lyrics.len() == i {
            break;
        }
    }
}


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change( some_string: &mut String) {
    some_string.push_str(", world");
}
