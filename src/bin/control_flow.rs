//! Learning basic control flow: if else

use std::io;

/// ### Control Flow if...else if...else...
///
/// Just like what you would write in C/C++:
///
/// ```cpp
/// int x;
/// scanf("%d", &x); // C
/// std::cin >> x; // C++
/// if (x < 5) {
///     printf("The number is smaller than 5\n"); // C
///     std::cout << "The number is smaller than 5" << std::endl; // C++
/// } else if (x > 5) {
///    printf("The number is bigger than 5\n"); // C
///    std::cout << "The number is bigger than 5" << std::endl; // C++
/// } else {
///     printf("The number is just 5!\n"); // C
///     std::cout << "The number is just 5!" << std::endl; // C++
/// }
/// ```
///
/// In Rust, just do it the same but remember to eliminate "()"!
///
/// ```no_run
/// let x: i32;
/// if x < 5 {
///     println!("The number is smaller than 5");
/// }
/// ```
///
/// Implement function cf_if which:
///
/// 1. take a number x (i32) from stdin.
/// 2. Compare it with 5, print "Less", "Equal", "Greater" (remember newline)
/// according to the result of comparing x with 5.
fn cf_if() {
    todo!() // implement here!
}

fn main() {
    cf_if();
}
