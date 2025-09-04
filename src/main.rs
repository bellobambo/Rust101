fn main() {
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

    for i in 0..6 {
        print!("{}", i);
    }
}

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
