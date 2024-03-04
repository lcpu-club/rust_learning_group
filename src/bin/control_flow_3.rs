//! Learning basic control flow: loops

/// ### Loops
///
/// In Rust, there are three primary loop types: `while`, `for`, and `loop`.
///
/// The `while` loop in Rust is akin to its counterparts in other programming
/// languages. Consider the following C code:
///
/// ```c
/// int x = 0;
/// while (x < 5) {
///   printf("x is %d\n", x);
///   x++;
/// }
/// ```
///
/// This can be equivalently expressed in Rust as:
///
/// ```rust
/// let mut x = 0;
/// while x < 5 {
///    println!("x is {}", x);
///    x += 1;
/// }
/// ```
///
/// Next, `for` loops in Rust are typically used to iterate over a range, an
/// array, or any type that implements the `Iterator` trait. Their behavior is
/// similar to Python's `for` loops. Here are some examples:
///
/// ```rust
/// // Iterating over a range
/// for x in 0..5 {
///     println!("x is {}", x);
/// }
///
/// // Iterating over an array
/// let arr = vec![1, 2, 3, 4, 5];
/// for x in arr {
///     println!("x is {}", x);
/// }
///
/// // Iterating over characters in a string
/// let s = "Hello";
/// for c in s.chars() {
///     println!("c is {}", c);
/// }
/// ```
///
/// The `loop` construct in Rust creates an infinite loop, which is analogous
/// to a `while` loop with a perpetually true condition.
///
/// ```rust
/// let mut x = 0;
/// loop {
///     println!("x is {}", x);
///     x += 1;
/// }
/// ```
///
/// Control flow statements like `break` and `continue` are used to terminate
/// the loop or skip the remainder of the current iteration, respectively, and
/// initiate a new one. This behavior is consistent with their usage in other
/// languages.
///
/// ```rust
/// let mut x = 0;
/// loop {
///     if x >= 5 {
///         break;
///     }
///     x += 1;
/// }
/// assert_eq!(x, 5);
/// ```
///
/// In the context of nested loops, `break` and `continue` only influence the
/// innermost loop by default. To affect the outer loop, labels can be used to
/// specify which loop should be broken or continued.
///
/// ```rust
/// 'outer: for x in 0..5 {
///     'inner: for y in 0..5 {
///         if y == 3 {
///             break 'outer;
///         }
///     }
/// }
/// ```
///
/// Rust also allows the `break` statement in a `loop` to return a value. This
/// feature is particularly useful for extracting a value from the loop.
///
/// ```rust
/// let x = loop {
///     let mut buffer = String::new();
///     std::io::stdin().read_line(&mut buffer).unwrap();
///     let x = buffer.trim().parse::<i32>();
///     if x.is_ok() {
///         break x.unwrap();
///     }
///     println!("Invalid input, please try again");
/// };
/// ```
///
/// ### Quiz: UNO!
///
/// Let's play a game of UNO! You are given a deck of cards, each numbered from
/// 0 to 12, plus two special cards: "+2" and "+4" (not in the deck). The game
/// proceeds as follows:
///
/// 1. The judge presents you with a card.
/// 2. If you possess a card bearing the same number, you play it. In the
///    absence of such a card, you are required to continually draw from the
///    deck until you obtain a playable card, which you then play.
/// 3. If the judge presents a special card, "+2" or "+4", you are obligated to
///    draw two or four cards from the deck, respectively.
/// 4. After you have played a card (or drawn cards as a result of a special
///    card), you need to verify if you are left with only one card. If this is
///    the case, you must announce "UNO!", signifying the end of the game.
/// 5. If the game does not end, return to the first step.
///
/// To ensure the game is reproducible, we utilize a magic deck that generates
/// cards in a mathematically predictable manner. The deck is initialized with
/// a zero seed value, and after each draw, the seed is updated according to
/// the following formula: `(seed * 71 + 3) % 100`. The card number is the new
/// seed modulo 10. Thus, the first card you draw is 3, and the subsequent
/// card's number is 6. If this calculation seems complex, fret not; we have
/// implemented the magic deck for you - simply invoke the `draw_card()`
/// function to acquire the next card.
///
/// The input for the game is provided in the following format: Each line
/// contains a card presented by the judge, either a number from 0 to 12, "+2",
/// or "+4". When the game ends, output "UNO!" and the total count of
/// cards you have drawn on the next line.
///
/// Complete the function `quiz` to solve the problem.
///
/// #### Example
///
/// ##### Input
///
/// ```text
/// +2
/// 2
/// 3
/// 6
/// ```
///
/// ##### Output
///
/// ```text
/// UNO!
/// 4
/// ```
fn quiz() {
    let mut total = 0;
    let mut cards = [0; 13];
    
    // Your code here

    println!("{}", total);
}

fn read_line() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

/// A magic deck for you :D
fn draw_card() -> usize {
    use std::cell::Cell;
    thread_local! {
        static SEED: Cell<i32> = const { Cell::new(0) };
    }
    SEED.with(|seed| {
        let val = seed.get();
        let next = (val * 71 + 3) % 100;
        seed.set(next);
        (next % 10) as usize
    })
}

fn main() {
    quiz()
}
