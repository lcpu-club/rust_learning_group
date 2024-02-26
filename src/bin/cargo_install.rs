//! Install cargo toolchain.

/// Try run this program on your computer and expect something fancy printed!
/// 1. run `cargo build --bin cargo_install` to only build this binary.
/// 2. run `cargo run --bin cargo_install` to only run this binary.
fn main() {
    println!("{}", HELLO_MESSAGE);
}

const HELLO_MESSAGE: &'static str = r#"
    +---+    +----------+ +-------------------+
    |   |   /           | |                   |
    |   |  |    +-------+ |    +--------+     +
    |   |  |    |         |    |        |     |
    |   |  |    |         |    +--------+     /
    |   |  |    |         |                  / 
    |   |  |    |         |    +------------+
    |   |  |    +-------+ |    | +----+  +----+
    |   |   \           | |    | |    |  |    |
    |   |    +----------+ |    | |    |  |    |
    |   ----------------+ |    | |    +--+    |
    |                   | |    | |            |
    +-------------------+ +----+ +------------+
    ########### RUST LEARNING GROUP ###########
    HELLO THERE!
    Welcome to LCPU Rust Learning Group.
    This learning group is created and managed
    by LCPU (Linux Club of Peking University),
    which is a non-official learning group for
    all those who want to learn Rust programming
    language. You will be learning mainly only
    your own with "The book", attending group
    talking weekly and sharing what you have dis-
    covered.

    Contributors:
    Bohai Li      (李博海): AP of LCPU.
    Yuanhang Sun  (孙远航): AP and previous leader of LCPU.
    Haonan Xue    (薛昊男): AP of LCPU.
    Zisu Zhang    (张子苏): AP of LCPU.
    QianKang Zhou (周乾康): AP of LCPU.
    ...To be added.

    Contact with us: lcpu@pku.edu.cn
    
    Looking forward if you want to join us and make
    The learning group a better place for sharing
    knowledge!
"#;