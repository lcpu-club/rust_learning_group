//! Learn how to print something to screen!

use std::io::{self, Write};

/// ### Output is just same logic as input!
/// You may have learnt basic input. Now just fill up the code and expect
/// `hey, stdout!` to be printed.
///
/// ```no_run
/// fn stdout_lock() -> io::Result<()> {
///    // TODO: get stdout
///    /* FILL HERE */
///    // TODO: lock up stdout and get the handle
///    /* FILL HERE */
///    handle.write_all(b"hey, stdout!")?;
///    Ok(())
/// }
/// ```
///
/// ### However, we have shortcuts: macro!
///
/// This is shorter but do the same thing!
///
/// ```no_run
/// print!("hey, stdout!\n");
/// ```
///
/// `println!` adds a '\n' automatically for you.
///
/// ```no_run
/// println!("hey, stdout!");
/// ```
fn stdout_lock() -> io::Result<()> {
    todo!() // Paste code and complete it!
}

fn main() {
    stdout_lock().unwrap();
}
