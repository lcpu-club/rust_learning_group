//! Function definition and usage.

/// ### Not that functional programming, I think
/// 
/// In Rust, functions are defined using the `fn` keyword, followed by the
/// function name, a list of parameters, an arrow, and the return type. The
/// function body is enclosed in curly braces. Here's an example:
/// 
/// ```
/// fn add(x: i32, y: i32) -> i32 {
///     x + y
/// }
/// ```
/// 
/// The `add` function takes two parameters, `x` and `y`, both of type `i32`, and
/// returns an `i32`. The function body simply adds `x` and `y` together and
/// returns the result.
/// 
/// Types are an important aspect of Rust's function signatures, and cannot be
/// omitted. If a function does not return a value, the return type is `()`, which
/// is the unit type, in which case the arrow and return type can be omitted.
/// 
/// ```
/// fn greet(name: &str) {
///     println!("Hello, {}!", name);
/// }
/// ```
/// 
/// Calling a function is straightforward. Here's how you would call the `add`
/// and `greet` functions:
/// 
/// ```
/// let sum = add(1, 2);
/// greet("world");
/// ```
/// 
/// ### Quiz
/// 
/// Fix the code below to make it compile and pass the tests.
/// 
/// Try `cargo fix` if you're stuck.
/// 
/// ```no_run
/// fn quiz() {
///     fn foo(x, y: i32) { // FIX ME
///         println!("x: {}, y: {}", x, y);
///         "hello"
///     }
/// 
///     assert_eq!(foo(1, 2), "hello");
/// }
/// ```
fn quiz() {
    fn foo(x, y: i32) { // FIX ME
        println!("x: {}, y: {}", x, y);
        "hello"
    }
    assert_eq!(foo(1, 2), "hello");
}

fn main() {
    quiz()
}
