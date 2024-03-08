//! Basic data types in Rust: numbers and booleans

/// ### Numbers and Booleans in Rust
///
/// Understanding numeric types is typically the initial step in learning any
/// programming language.
///
/// In C/C++, numeric types include `int`, `long`, `float`, `double`, and
/// others. Here are some examples:
///
/// ```c
/// int x = 5;
/// float y = 5.0;
/// unsigned long z = 5;
/// ```
///
/// Python, on the other hand, primarily uses `int` and `float` types:
///
/// ```python
/// x = 5
/// y = 5.0
/// ```
///
/// Rust's numeric types bear a resemblance to those in C/C++, albeit with a
/// few distinctive characteristics. They incorporate fixed sizes and
/// signedness directly into their names:
///
/// ```rust
/// let x: i32 = 5;
/// let y: f64 = 5.0;
/// let z: u64 = 5;
/// ```
///
/// In Rust, the `i` in `i32` denotes "integer", the `f` in `f64` represents
/// "float", and the `u` in `u64` signifies "unsigned integer". The number
/// following the letter specifies the number of bits that the type occupies
/// in memory. For instance, the commonly used `int` in C/C++ corresponds to
/// `i32` in Rust, which is a 32-bit signed integer.
///
/// Additionally, Rust includes `isize` and `usize`, which are pointer-sized
/// integers. These types correspond to `ssize_t` and `size_t` in C/C++, and
/// are commonly employed for indexing collections.
///
/// Rust also incorporates a `bool` type for Boolean values, which can either
/// be `true` or `false`. Conditional expressions such as `if` and `while`
/// necessitate a `bool` value. 1 and 0 are not considered `true` and `false`.
///
/// ```rust
/// let t: bool = true;
/// let f: bool = false;
/// if t {
///    println!("It's true!");
/// }
/// ```
///
/// It's important to note that Rust does not support implicit conversion
/// between different numeric types or arithmetic operations between them.
///
/// ```rust
/// let arr: Vec<i32> = vec![1, 2, 3];
/// let i: i32 = 2;
/// let x = arr[i]; // Error: mismatched types
///
/// let x: i32 = 1;
/// let y = x + 1.0; // Error: mismatched types
/// ```
///
/// The above code fails to compile due to type mismatches: `i` is of type
/// `i32`, whereas vector indices must be of type `usize`. Similarly, the
/// second example cannot compile because an `i32` cannot be added to an `f64`.
///
/// To rectify these errors, explicit type conversion is required. In Rust,
/// this can be achieved using the `as` keyword.
///
/// ```rust
/// let arr: Vec<i32> = vec![1, 2, 3];
/// let i: i32 = 2;
/// let x = arr[i as usize]; // OK
///
/// let x: i32 = 1;
/// let y = x as f64 + 1.0; // OK
/// ```
///
/// Moreover, the `as` keyword can be used to convert between integers, floats,
/// and booleans.
///
/// ```rust
/// let x: i32 = 1;
/// let y: f64 = x as f64;
/// assert_eq!(y, 1.0);
///
/// let b: bool = true;
/// let i: i32 = b as i32;
/// assert_eq!(i, 1);
/// ```
///
/// The prohibition of implicit conversion between different types is a design
/// decision in Rust aimed at preventing bugs. This requirement encourages
/// careful consideration of potential truncation, overflow, or loss of
/// precision when casting between types. 
///
/// When casting between integer types using `as`, Rust performs truncations
/// and/or sign extensions. For instance, converting from `u8` to `i32` pads
/// the most significant bits with zeros, while converting from `u64` to `i32`
/// simply truncates.
///
/// When casting between float and integer types, Rust performs rounding
/// towards zero. The complete rules can be found in [the Rust Reference].
/// 
/// Cast is only allowed between certain primitive types. For more complex type 
/// conversions, we'll cover them later.
///
/// ### Quiz
///
/// Read an integer `x` and a float `y`, then print the following:
///
/// 1. The lowest 8 bits of `x` as an unsigned integer.
/// 2. The integer part of `y`.
/// 3. The sum of the above two numbers.
///
/// The input will be two lines, the first line contains an integer `x`, and the
/// second line contains a float `y`. The output should be three lines, each
/// contains a single integer.
///
/// `x` will be in the range of [-2^31, 2^31 - 1], and `y` will be in the range of
/// [-10^9, 10^9].
///
/// Complete the function `quiz` to solve the problem.
///
/// ```
/// fn quiz() {
///     let x = read_i32();
///     let y = read_f64();
///     
///     // Your code here
/// }
/// ```
///
/// [the Rust Reference]: https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast
fn quiz() {
    let x = read_i32();
    let y = read_f64();

    // Your code here
}

fn read_i32() -> i32 {
    read()
}

fn read_f64() -> f64 {
    read()
}

fn read<T>() -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse::<T>().unwrap()
}

fn main() {
    quiz()
}
