

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
}

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
