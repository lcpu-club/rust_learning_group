//! Basic data types in Rust: characters and strings

/// ### Characters and Strings in Rust
///
/// Rust's character and string types distinctively deviate from other
/// languages, drawing from the strengths and avoiding the pitfalls of its
/// predecessors.
///
/// In Rust, the `char` type denotes a Unicode character, which is not merely a
/// single byte but a full Unicode scalar value. With a size of 32 bits, a Rust
/// `char` can represent any Unicode character, including those beyond the ASCII
/// range. Character literals in Rust are expressed using single quotes. For
/// instance:
///
/// ```
/// let ch = 'A'; // ASCII character
/// let ch = '龘'; // Chinese character
/// let ch = 'ℤ'; // Mathematical character
/// let ch = '🦀'; // Emoji
/// ```
///
/// Rust also guarantees that a `char` is a valid Unicode scalar value at compiler
/// level.
/// 
/// Contrarily, Rust strings are not simply arrays of characters. They are
/// represented as UTF-8 encoded bytes, enhancing memory efficiency and
/// compatibility with byte-oriented systems. UTF-8 is a variable-width
/// encoding format that translates a Unicode code point into one to four bytes,
/// depending on the code point, thereby ensuring full compatibility with ASCII.
///
/// Due to performance considerations, Rust conducts string operations at the
/// byte level rather than the character level, significantly affecting string
/// length and indexing. For example:
///
/// ```
/// let s = "Hello";
/// assert_eq!(s.len(), 5); // For ASCII, the number of bytes equals the number
///                         // of characters
/// let s = "你好";
/// assert_eq!(s.len(), 6); // Chinese characters are mostly encoded into three
///                         // bytes, hence the length is 6
/// assert_eq!(&s[0..3], "你"); // Indexing is also at the byte level
/// ```
///
/// For character-level operations, Rust provides the `chars` method to iterate
/// over the characters in a string, returning an `Iterator`. We will delve
/// into the concept of `Iterator` in more depth in future discussions. Here is
/// an example:
///
/// ```
/// let s = "你好";
/// assert_eq!(s.chars().count(), 2); // The number of characters is 2
/// assert_eq!(s.chars().nth(0), Some('你')); // Indexing at the character level
/// ```
///
/// Rust provides two types for handling strings: `&str` and `String`.
/// The `&str` type corresponds to an immutable reference to a string slice,
/// while `String` denotes a growable, heap-allocated string. For those
/// familiar with C++, `&str` is similar to `const char*`, and `String` is
/// similar to `std::string`.
///
/// String literals are written using double quotes, resulting in a value of
/// type `&str`. A `&str` can be converted into a `String` by invoking the
/// `to_string` method. This would involve heap allocation and copying.
///  For example:
///
/// ```
/// let s: &str = "Hello"; // A string literal
/// let s: String = "Hello".to_string(); // A string literal converted to a String
/// ```
/// 
/// Rust also has multiline string literals, you may check them out in the 
/// [Raw Literals in Rust Reference].
/// 
/// To obtain a `&str` from a `String`, you can either slice the `String` or
/// use the `as_str` method to fetch a reference to the entire string, as shown
/// below:
///
/// ```
/// let s: String = "Hello".to_string();
/// let s: &str = &s[1..]; // Slicing
/// assert_eq!(s, "ello");
/// let s: String = "Hello".to_string();
/// let s: &str = s.as_str(); // Using the as_str method
/// assert_eq!(s, "Hello");
/// ```
///
/// In the above examples, `1..` signifies the range from the first byte to the
/// string's end. Since slicing operates at the byte level, the slicing
/// position must align with a character boundary; otherwise, it will result in
/// a panic. If you don't want to panic, you can use the `str::get` method to
/// slice the string, getting a `None` if there's any boundary error.
/// 
/// ```
/// let s: String = "Hello".to_string();
/// let s: Option<&str> = s.get(1..); // Using the get method
/// assert_eq!(s, Some("ello"));
/// ```
///
/// The `&` operator is utilized to fetch a reference to a value. We will delve
/// into this operator in more depth in subsequent discussions. 
/// 
/// You may wonder what is a `str` compared to `&str`. The `str` is a type that
/// you cannot build a corresponding value by hand, but can be used to construct
/// other types. We'll ignore it for now.
///
/// You may also wonder why we can call the `get` method on a `String`, which is
/// not a `str`. There are indeed some sneaky implicit type conversions going on!
/// We'll discuss this in more detail in the future, but for now, you can safely call
/// any method that is defined for `str` on a `String`.
/// 
/// ### Quiz: "NOT yes!"
///
/// The "yes" command is a fascinating command-line utility that perpetually
/// outputs the character "y" to the standard output. Your task is to implement
/// an inverted version of the "yes" command that prints strings devoid of the
/// letter "y".
///
/// The program should accept a string `s` and an integer `x`, then print the
/// string `x` times, with any occurrence of the letter "y" omitted. For
/// instance, if the input string is "yes" and the integer is 3, the output
/// should be:
///
/// ```text
/// eseses
/// ```
///
/// The input will be provided in two lines: the first line contains the string
/// `s`, and the second line contains the integer `x`. Any leading or trailing
/// whitespace in `s` should be disregarded. The output should consist of a
/// single line containing the repeated strings.
///
/// You might find the following functions useful:
///
/// 1. `trim`: `s.trim()` returns a string slice with leading and trailing
///     whitespace removed. The return type is `&str`. What can you do if you
///     need a `String`?
/// 2. `retain`: `s.retain(|c| c != 'y')` eliminates all characters equal to 'y'
///     from the string `s`. This in-place operation requires `s` to be
///     declared as mutable. This method is applicable for `String` but not
///     for `&str`.
/// 3. `push_str`: `s.push_str(t)` appends the string `t` to the string `s`.
///     Note that `s` must be a mutable `String`, while `t` must be a `&str`.
///     This is also an in-place operation.
/// 4. `read_line` and `parse_i32`: These functions are provided for you to
///     read a line from standard input and parse a string into an integer,
///     respectively.
///
/// Complete the `quiz` function to solve this problem.
///
/// ```
/// fn quiz() {
///     let mut result = String::new();
///
///     // Your code here
///
///     println!("{}", result);
/// }
/// ```
/// 
/// [Raw Literals in Rust Reference]: https://doc.rust-lang.org/reference/tokens.html#raw-string-literals
///
fn quiz() {
    let mut result = String::new();

    // Your code here
    
    println!("{}", result);
}

fn read_line() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer
}

fn parse_i32(s: &str) -> i32 {
    s.trim().parse::<i32>().unwrap()
}

fn main() {
    quiz()
}
