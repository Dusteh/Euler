
fn main() {
    println!("Hello, world!");
    let three = 3;
    let five = 5;

    let range = 1000;
    let mut sum = 0;
    for x in  1..(range) {
        if x % three == 0 || x % five == 0 {
                println!("Number is: {}",x);
                sum += x;
        }
    }
    println!("Sum: {}",sum );

}
