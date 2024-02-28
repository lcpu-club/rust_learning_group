//! Complex "Hello, world!" program!
use std::io::{self, BufRead, Write};

/// We are going to complete a more complex version of "Hello, world"
/// program in this section.
/// Instead of just print out "Hello, world!", we want to make the
/// program receive something you type in from console, and print out
/// it along with other messages.
/// So let's make it clear here: we are going to "read" something with
/// stdin, construct the message, and then "write" the message with
/// stdout.
/// Guide:
/// 1. Prepare a mutable buffer typed as String(std::string::String).
/// 2. Acquire stdin handle as you have done in "basic_input".
/// 3. Call `read_line` to store your input in the buffer.
/// 4. Construct the message s using format! macro. You might have seen
///    similar thing in Python like `print("%r %r" % (a, b))`. Remember
///    to refer to Rust Documentaion if you do not understand!
/// 5. Acquire stdout handle as you have done in "basic_output".
/// 6. Call `write_all` to print you message (convert it to bytes with
///    `as_bytes` first!) to the console. I know you can just use
///    `println!` or `print!` which is more convenient and with which
///    you could just skip step 4 to 6. But in order to practice, we
///     will use stdout instead.
fn main() {
    // Declare a mutable buffer typed as String.
    let mut buffer = String::new();

    let stdin = io::stdin();
    // TODO: acquire stdin lock.
    // FILL HERE!

    // TODO: call `read_line` and read to the buffer.
    // FILL HERE!

    let s = format!("Hello, world! Welcome to LCPU RLG, {}!", buffer);

    let stdout = io::stdout();
    // TODO: acquire stdout lock.
    // FILL HERE!

    // TODO: call `write_all` and write to the console.
    // FILL HERE!
}