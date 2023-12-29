fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    for arg in args {
        if arg == "sum" {
            sum();
        } else if arg == "double" {
            double();
        } else {
            count();
        }
    }
}

fn sum() {
    let mut sum = 0;

    for num in [7..=23] {
        sum += num;
    }

    println!("The sum is {}", sum);
}

fn double() {
    let mut count = 0;
    let mut x = 1;

}

fn count() {
    
}
