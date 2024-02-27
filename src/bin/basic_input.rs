//! Learn basic input from keyboard!
use std::io::{self, BufRead};

/// ### IO is not as easy as you might have thought!
///
/// You might have written many IO in C, C++, Python or other languages.
///
/// ```c
/// // C
/// #include<stdio.h>
/// int main() {
///     int i; char c;
///     scanf("%d%c", &i, &c); // Don't forget "&" to get the address of `i` and `c`!
///     printf("%d %c", i, c); // No need for "&"
///     return 0; // Good habit not to eliminate return value.
/// }
/// ```
///
/// In C++ and Python, that's much simpler without even considering whether
/// you are putting something "into an address"(id est, you even could never
/// considering pointers with convenient I/O stream. Or, interpreter has done
/// all of these for you!)
///
/// ```cpp
/// // C++
/// #include<iostream>
/// int main() {
///     int i; char c;
///     std::cin >> i >> c; // using istream
///     std::cout << i << ' ' << c << std::endl; // using ostream
///     return 0;
/// }
/// ```
///
/// ```py
/// # Python
/// i = int(input()) # input a `int`
/// c = str(input()) # input a `char` (or maybe a `str` if you want)
/// print(i, c)
/// ```
///
/// ### Rust std input
///
/// But in Rust, it goes a little different.
///
/// Remember that Rust requires you that you must think thoroughly what you
/// are doing?
///
/// So, considering the keyboard and the screen you are using. In fact, many
/// threads in a process might fight for the keyboard and screen, occupy them
/// and then use them for input or output. So there must be a lock to synchronize
/// between all these candidates.
///
/// A thread can only read when it acquired the lock of stdin, and write when it
/// acquired the lock of stdout.
///
/// Implement the code below, run it, type in "foobar" and expect to see the
/// identical content printed out.
///
/// (Remember that if we want to change the content of `buffer`, we need to make
/// it `mut` first. The same goes for input stream because you are `taking`
/// something out of input stream and put it to the buffer!)
/// ```no_run
/// fn stdin_lock() -> io::Result<()> {
///     print!("Feed in here: ");
///     let mut buffer = String::new();
///     // TODO: Get stdin stream
///     /* FILL HERE */
///     // TODO: Lock up the stdin stream with `lock()`
///     /* FILL HERE */
///     // Using the lock handle to read!
///     handle.read_line(&mut buffer)?;
///     println!("{}", buffer);
///     Ok(())
/// }
/// ```
fn stdin_lock() -> io::Result<()> {
    todo!() // Paste code and complete it!
}

fn main() {
    stdin_lock().unwrap();
}
