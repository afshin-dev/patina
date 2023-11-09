fn main() {
    // closure can use environment or
    // enclosing variable scope
    // we define a variable called x in main scope
    let x: i64 = 11;

    // then we define a closure that use x in body of itself

    let compare_with_x = |n: i64| -> bool { n == x };

    // then use tht `compare_with_x` function
    println!("{}", compare_with_x(3));

    // but if we use function in this example
    // function no compile

    // this section wont works
    // uncomment this to try
    /*
    fn compare_with_x_(n: i64) -> bool {
        n == x // can't capture dynamic environment in a fn item
    }
    println!("{}", compare_with_x_(3));
    */
}
