//fn main() {
//   println!("Hello, world!");
//
//    another_function(5);
//}
//
//fn another_function(x: i32) {
//    println!("the value of x is: {}", x)
//}


//fn main() {
//    print_labeled_measurement(5, 'h');
//}
//
//fn print_labeled_measurement(value: i32, unit_lable: char){
//    println!("the measurement is: {}{}", value, unit_lable);
//}

//fn main() {
//    let y = {
//        let x = 3;
//        x + 1
//    };
//    println!("the value of y is: {}", y);
//}

fn five () -> i32 {
    5
}

fn main() {
    let x = five();

    println!("the value of x is: {}", x); 
}