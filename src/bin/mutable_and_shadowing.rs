//! Declare mutable variables and shadowing.

/// ### Why can't I change the value of my variable?
///
/// In many programming languages, it's a common practice to declare a variable
/// and subsequently modify its value - a feature known as mutability. Consider
/// the following C code:
///
/// ```c
/// int x = 1;
/// x = x + 1;
/// ```
///
/// In this example, the variable `x` is initially assigned a value of 1, which
/// is later incremented by 1.
///
/// Contrastingly, attempting a similar operation in Rust results in a compiler
/// error:
///
/// ```rust
/// let x = 1;
/// x = x + 1; // ERROR
/// ```
///
/// The reason for this error is that, unlike many other languages, variables
/// in Rust are immutable by default. This implies that once a value is
/// assigned to a variable (or, in other words, a value is "bound" to a name),
/// it cannot be altered. This design choice aids in bug prevention and
/// enhances code readability.
///
/// However, Rust provides a feature known as "shadowing" for instances where
/// you may not necessarily want to alter a variable's value, but instead use
/// a different value under the same variable name. This can be achieved by
/// declaring a new variable using the same name as the existing one,
/// effectively "shadowing" the original.
///
/// ```
/// let x = 1;
/// let x = x + 1; // OK
/// ```
///
/// The ability to reuse the same variable name for different types of values
/// is a significant advantage of shadowing in Rust. It eliminates the need to
/// devise unique names for each variable, simplifying the process of data
/// manipulation.
///
/// Consider the following example (note: the example might include some syntax
/// that's unfamiliar to you, but don't worry about it for now. The focus here
/// is on the concept):
///
/// ```rust
/// // Read a line from standard input
/// let mut number = String::new();
/// io::stdin().read_line(&mut number).unwrap();
/// // At this point, `number` is of type `String`
///
/// // Trim the whitespace
/// let number = number.trim();
/// // Now, `number` is of type `&str`
///
/// // Parse the string into an integer
/// let number = number.parse::<i32>();
/// // At this stage, `number` is of type `Result<i32, ParseIntError>`
///
/// // Unwrap the result
/// let number = number.unwrap();
/// // Finally, `number` is of type `i32`
/// ```
///
/// This example demonstrates a common pattern in Rust, often employed to
/// transform values from one type to another. Each step redefines the variable
/// `number` with a new type, using the same name throughout to maintain
/// clarity and consistency in the code.
///
/// Shadowing, while useful, does not cater to all scenarios where variable
/// mutation might be required. In such instances, Rust provides the `mut`
/// keyword for declaring mutable variables.
///
/// Consider the following example:
///
/// ```rust
/// let mut x = 1;
/// x = x + 1; // OK
/// ```
///
/// This approach becomes particularly useful when you need to modify a
/// variable's value within a loop or perform in-place mutation, as
/// demonstrated below:
///
/// ```rust
/// let mut sum = 0;
/// for i in 1..=10 {
///    sum += i;
/// }
/// println!("{}", sum); // Outputs: 55
///
/// let mut x = vec![1, 2, 3];
/// x.push(4);
/// println!("{:?}", x); // Outputs: [1, 2, 3, 4]
/// ```
///
/// Functions that may mutate their arguments necessitate those arguments to be
/// declared as mutable. Fortunately, Rust's compiler will remind you to
/// declare a mutable variable when necessary, preventing potential oversights.
///
/// Mutability is a crucial aspect of Rust's memory model. The discussion here
/// merely scratches the surface of this concept, and there's much more to
/// learn about it in future studies.
///
/// ### Quiz
///
/// Fix the code below to make it compile and pass the tests. Use shadowing and
/// mutability.
///
/// ```no_run
/// fn quiz() {
///     let x = 1; // FIX ME
///     assert_eq!(x, 1);
///     
///     x = 2;
///     assert_eq!(x, 2);
///     
///     x = "hello".to_string(); // FIX ME
///     x.push_str(", world!");
///     assert_eq!(x, "hello, world!");
/// }
/// ```
fn quiz() {
    let x = 1; // FIX ME
    assert_eq!(x, 1);

    x = 2;
    assert_eq!(x, 2);

    x = "hello".to_string(); // FIX ME
    x.push_str(", world!");
    assert_eq!(x, "hello, world!");
}

fn main() {
    quiz()
}
