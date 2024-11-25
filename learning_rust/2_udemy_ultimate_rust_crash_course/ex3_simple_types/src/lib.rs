pub fn print_difference(x: f32, y: f32) {
    println!("Difference between {} and {} is {}", x, y, (x - y).abs());
}

pub fn print_array(arr: [f32; 2]) {
    println!("The corrdinates are {:?}", arr);
}

pub fn ding(x: i32) {
    if x == 12 {
        println!("Ding, you found 13!");
    }
}

pub fn on_off(val: bool) {
    if val {
        println!("Lights are on!");
    }
}