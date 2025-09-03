fn main() {
    // let _unsigned: u8 = 10;

    // let _signed: i8 = -10;

    // let _float: f32 = -1.5;

    // println!("unsign: {} sign: {} float: {}", _unsigned, _signed, _float);

    // let is_true: bool = true;
    // println!("is_true: {}", is_true);

    let arr: [u8; 3] = [1, 2, 3];
    let other_arr: [u8; 5] = [100, 5, 3, 6, 92];

    println!("index: {}, length: {}", arr[0], other_arr.len());

    println!("{:?}", other_arr);
}
