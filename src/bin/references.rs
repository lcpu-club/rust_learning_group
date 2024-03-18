//! References and Borrowing in Rust
//! See also: [References and Borrowing in the Rust Book](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

/// ### References
/// 
/// If you came from C++, you might have heard of references. However, the
/// concept of references in Rust is *very different* from C++. In C++, a 
/// reference is somewhat *an alias* to the variable. Notice that in the 
/// following C++ code, when you pass `a` to a reference `b`, `b` will become
///  an alias to it. You can use `b` as if it's of type `T` , without having
///  to write `b->push_back`.
///
/// ```c++
/// std::vector<int> a{1, 2, 3};
/// std::vector<int> &b = a; // A C++ reference to a variable
/// b.push_back(4); // You can use C++ reference type `T&` just like `T`
/// ```
///
/// Under the hood, C++ references are pointers, but they are used like normal
/// variables, that's why they are treated as aliases.
///
/// References in Rust are different. You may call them *safe pointers*. 
/// Use **borrow operator** `&` (the address-of operator in C) to get a reference.
///
/// ```
/// let x: usize = 1;
/// let y: &usize = &x; // You cannot write `x` here!
/// ```
///
/// References to `T` are of their own type `&T`, just like C++ pointers.
///
/// ```
/// fn ref_is_a_type(x: &usize) { /* ... */ }
/// 
/// let x = 114514;
/// ref_is_a_type(&a); // OK
/// ref_is_a_type(a);  // ! Compiler Error: Mismatched type!
/// ```
/// 
/// The `&mut` operator will give you a mutable reference, which allows
/// you to modify the address it points to through it. The mutable
/// reference `&mut T` is like the normal C pointer, while the immutable
/// reference `&T` (also called shared reference) is like the pointer
/// version of C++ `const T&`.
///
/// Deref operator `*` is `&`'s counterpart.
///
/// ### Borrowing
///
/// Instead of 'getting address of', Rust uses 'borrowing' to describe
/// the `&`. A reference doesn't take away the original value's ownership,
/// but uses it temporarily, and guarantees a 'return' after use. 
///
/// ```
/// let owner = String::from("Hello, world!");
/// let reference = &owner;
/// drop(*reference); // ! Compiler Error: Cannot move out of reference
/// ```
/// 
/// Rust compiler also enforces a reader-writer lock rule on reference
/// (with no runtime cost). Within any scope, you can either
///
/// 1. Have *any number of* immutable references.
/// 2. Have *only one* mutable reference.
///
/// You may wonder why we need to prevent data races if we are not
/// concerned about concurrent programming. Consider the following
/// situation. If you push an element to the back of a `Vec` through
/// a mutable reference `a`, but there's an immutable reference `b`
/// pointing to the first element at the same time. What would you get
/// when you read from `b`, if the `Vec` grows and moved to a new place?
/// 
/// As a *safe* pointer, references are guaranteed to be valid, if you
/// are not playing with `unsafe` stuff. You won't access illegal memory
/// when using it. The tradeoff is that you have to follow some rules.
/// Besides the borrowing rules, Rust also have lifetime rules to ensure
/// this. We'll (definitely) look into lifetimes later.
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
#[allow(unused)]
fn main() {
    let vec: Vec<_> = vec!["Rust", "is", "the", "Genshin", "Impact", "of", "programming", "languages", "but", "if", "you", "learn", "it", "well", "it", "feels", "like", "Saizeriya"].into_iter().map(String::from).collect();

    fn this_is_t(t: String) -> String { t }
    fn this_is_ref(r: &String) -> &String { r }
    fn this_is_mut_ref(r: &mut String) -> &mut String { r }

    /*
        Guide: Fix the code below to make it compile.

        DO NOT REORDER any line in the following code.

        Fix errors in the following order:
        1. Check the function signatures and fix function and macro calls. Add `mut`, `&`, `&mut` where necessary. Do not change the `in vec` part. 
        2. Solve compiler error E0382. The compiler will suggest something for you. Consider ownership rules and reason why the solution works.
        3. Solve compiler error E0502 by commenting out one line of the `assert_eq!`. Consider borrowing rules and reason why commenting out that line works.
     */

    for word // FIX ME           
    in vec {
        let t = {
            this_is_t(word) // FIX ME
        };
        let r = {
            this_is_ref(word) // FIX ME
        };
        let r_mut = {
            this_is_mut_ref(word) // FIX ME
        };

        assert_eq!(t, r); // FIX ME
        assert_eq!(t, r_mut); // FIX ME
    }
}
