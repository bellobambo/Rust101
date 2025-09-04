use std::collections::HashMap;

#[derive(Debug)]
enum MyError {
    Error1,
}

fn divide(dividend: i32, divisor: i32) -> Result<i32, MyError> {
    if divisor == 0 {
        Err(MyError::Error1)
    } else {
        Ok(dividend / divisor)
    }
}

fn main() {
    // let divide = divide(4, 2);

    // match divide {
    //     Ok(v) => println!("value: {}", v),
    //     Err(e) => println!("error: {:?}", e),
    // }

    let divide = divide(4, 3);
    let res = divide.expect("We crashed");

    println!("value: {}", res);

    // let divide1: Option<i32> = divide(4, 2);
    // let divide2: Option<i32> = divide(2, 3);

    // println!("divide1: {:?}", divide1, divide1.unwrap());
    // println!("divide2: {:?}", divide2, divide2.unwrap());

    // let _unsigned: u8 = 10;

    // let _signed: i8 = -10;

    // let _float: f32 = -1.5;

    // println!("unsign: {} sign: {} float: {}", _unsigned, _signed, _float);

    // let is_true: bool = true;
    // println!("is_true: {}", is_true);

    // let arr: [u8; 3] = [1, 2, 3];
    // let other_arr: [u8; 5] = [100, 5, 3, 6, 92];

    // println!("index: {}, length: {}", arr[0], other_arr.len());

    // println!("{:?}", other_arr);

    // let tuple: (u8, bool, f32) = (10, true, -1.5);
    // let tuple2: (i32, i32) = (3, 5);

    // println!("first{}, second{}, third{}", tuple.0, tuple.1, tuple.2);
    // println!("tuple2: {:?}", tuple2);

    // println!("{}", is_even(2));

    // let mut num = 5;
    // num = 3;
    // println!("{}", num);

    // let arr = [0, 1, 3, 5];
    // let slice = &arr[1..3];
    // borrowing_slice(arr, slice);

    // let str: &str = "hello World";
    // let mut string: String = String::from("hello World");

    // let slice = &string[..6];
    // slice.len();

    // string.push_str("two");

    // string.push_str("Working");
    // string = string.replace("World", "Rust");
    // println!("{}", string);

    // let n = 3;
    // if n > 0 {
    //     println!("greater than 0");
    // } else if n < 0 {
    //     println!("less than 0");
    // } else {
    //     println!("n is zero");
    // }

    // for i in 0..6 {
    //     print!("{}", i);
    // }

    // let i = 9;
    // match i {
    //     0 => println!("zero"),
    //     1 | 2 => println!("two"),
    //     3..=5 => println!("3,4,5"),
    //     _ => println!("default"),
    // }

    // let name = String::from("Eagle");
    // let bird = Bird {
    //     name: name,
    //     attack: 10,
    // };
    // bird.print_name();
    // println!("{} {}", bird.can_fly(), bird.is_animal());

    // let a: MyEnum = MyEnum::A;
    // let b: MyEnum = MyEnum::B(5);
    // let c: MyEnum = MyEnum::C { x: 10, y: 20 };

    // println!("{:?}", a);
    // println!("{:?}", b);
    // println!("{:?}", c);

    // let mut vec: Vec<i64> = vec![1, 2, 3, 4, 5];
    // vec.len();
    // vec[0];
    // vec.push(6);
    // vec.remove(0);
    // println!("{:?}", vec);
}

// #[derive(Debug)]
// enum MyEnum {
//     A,
//     B(i32),
//     C { x: i32, y: i32 },
// }

// struct Bird {
//     name: String,
//     attack: u8,
// }

// impl Bird {
//     fn print_name(&self) {
//         println!("Bird name: {}", self.name);
//     }
// }

// impl Animal for Bird {
//     fn can_fly(&self) -> bool {
//         true
//     }
//     fn is_animal(&self) -> bool {
//         false
//     }
// }

// trait Animal {
//     fn can_fly(&self) -> bool;
//     fn is_animal(&self) -> bool {
//         true
//     }
// }

// fn borrowing_slice(arr: [u8; 4], slice: &[u8]) {
//     println!("{:?}", arr);
//     println!("{:?}", slice);
//     println!("length: {}", slice.len());
//     println!("{} {}", slice[0], slice[1]);
// }

// pub fn is_even(num: u8) -> bool {
//     let digit: u8 = num % 2;
//     digit == 0
// }
