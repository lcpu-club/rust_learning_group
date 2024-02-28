//! Introduce basic concepts to you are get started quickly!

#[allow(unused)]
fn p0_introduce_prints() {
    /*  Prints:

        Like what you would do in C, printing things to console:
        `printf("Basics\n");` .
        Rust do same thing in a similar form. But instead of calling
        a function, we use a macro to do all these things.
        Macros have following form: `<name>!` with `!` is the symbol
        of macro.

        `print!` print out the content exactly while `println!` print
        out the content and append a newline automatically.
    */
    print!("Basics\n");
    println!("Basics");
}

#[allow(unused)]
fn p1_introduce_basic_types() {
    /* Types:

        Type system in Rust is as powerful as the one in C. But instead
        of naming them with "int", "long", "char", Rust names them in a
        more unified form. And instead of praying that the C compiler
        would regard your `int` as a 32 bit interger (yes, the C standard
        didn't regulates how long exactly `int` should be, but just say
        it should be at least 32 bit), Rust integer types have explicit
        information of their length.

        `ix` means signed integer with length of x bits.
            e.g. `i8`, `i16`, `i32`, `i64` and `i128`.
            and `isize` means signed interger that is as long as your
            machine word (e.g on 32bit machine isize is i32, and on 64
            bit machine isize is i64.)

        `ux` means unsigned integer with length of x bits.
            e.g. `u8`, `u16`, `u32`, `u64` and `u128`.
            `usize` is just like `isize` but it's unsigned.

        By the way, instead of type them before their name like what you
        would do in C/C++ (e.g. int a; char c; unsigned long ul;), you'd
        type them after their name.
    */

    let a_dec: i32; // Declare a variable typed as i32 using `let` keyword.
    let a_def: u64 = 42; // Define a variable typed as u64 with initial
                         // value 42.

    /* bool
        Boolean value. Same as in most programming languages.
    */
    let b_t: bool = true;
    let b_f: bool = false;

    /*
        Where is `char`?
        Do not be confused by C/C++ concepts. `char` in C/C++ actually stores
        a 8-bit length value no matter it could be interpreted as a human
        readable "character".

        So feel free to use `u8` in Rust (or `i8` if you like. But usually 
        we use `u8`).

        We will cover more on this later.
    */
    let char_u8: &[u8] = "a".as_bytes(); // Don't worry if you do not understand this!
    // &[u8] means a reference to a array containing `u8` values.   
}

#[allow(unused)]
fn p2_introduce_vec_and_array() {
    /*
        In ancient languages such as C, we have `array` which occupies
        a continous block of memory to store values.
        `int a[10];`

        And in C++, which is a much more modern language than C (though
        not modern enough!), it is equipped with more powerful data types
        std::vector, which could store values and manage them automatically
        through methods like `push_back`, `clear`, `is_empty`.

        In Rust, we have them two as well.
    */

    // In rust, the type of array is annotated as [<item type>; <size>];
    // And could be initialized with [<init-value>; <size>];
    let a_array: [i32; 10] = [0; 10];

    // In rust, the type of vector is annotated as Vec<type>;
    let a_vec: Vec<i32> = Vec::new();
    let a_vec_another_initialization: Vec<i32> = vec![1,2,3];
    // vec! is a convenient macro for initialize a vector in Rust.

    /* The difference between Array and Vec.
        Array in C/C++ is expected to have fixed size upon declaration
        (in C++) or could be defined when the function stack is being
        built (in C, which support flexible array whose size could be defined
        in runtime).

        Usually Array is expected to be on stack.
        C/C++: `int a[100];`
        Rust: `let a: [i32; 100];`

        Or on heap.
        C: `int *a = malloc(100 * sizeof(int));`
        C++: `int *a = new int[100];`
        Rust: **We will talk about how to put value on heap explicitly later.**

        But array is hard to monitor and manipulate.
        So that's why vector exists (probably).
        To illustrate more clearly about the difference between array and vector,
        we need to know first how does vector's memory layout looks like.

          Stack                        Heap
        +-------+
        |  PTR  | ---------------> [0,1,2,3,4,5,42,1000,0,1] (An array)
        +-------+
        |  LEN  |
        +-------+
        |  CAP  |
        +-------+

        Basically, that's what a vector looks like.
        On stack, the vector only occupies 3-word space with one word storing the
        length of Vec, one word storing the capacity of Vec, and one word storing
        the pointer to a position in heap, where lies the "raw data" of Vec.
        
        You can think of Vec as: an array on heap + some metadata on stack.
        And that's the difference between array and vector.

        Vec could easily tell you how long it is by just refer to `LEN` field, and
        tell you it's capacity by just refer to `CAP` field with O(1) complexity.
    */

    /* So what is `&[i32]`?
        
        You might have know clearly what is pointer (in C) and reference (in C++).
        They are very similar but indeed different. Pointer is a concrete 4-Bytes or
        8-Bytes value that could be stored in a variable and cast to a interger.
        While reference is a notation that's much safer and easy to use.

        In basic Rust, there's only reference and you could only use pointer in an
        unsafe environment (you know pointer is of big trouble in C, don't you?) which 
        we would cover later in this semester.

        `&[i32]` means "a reference to a continuous space storing `i32` data".
        You can create a reference to a array store on stack.
        (So the reference actually refer to somewhere on the stack)
        Or you can create a reference to a vector, who stores its raw data on the heap.
        (So the reference actually refer to somewhere on the heap)

        By the way, you should specify "how much" data you want to refer to.
        In the example below, we created an array and a vector, and created two reference
        to them respectively using `&` operator.

        We say that we want to refer to the array from its 3rd item to its 9th item. (which
        is called a "slice" of array or vector)
        Then we should write our definition as `&array_demo[3..9]` or `&vec_demo[3..9]`.
        So `array_refer` and `vec_refer` are called "slice reference".
    */
    let array_demo: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let vec_demo: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let array_refer: &[i32] = &array_demo[3..9];    // [3, 4, 5, 6, 7, 8]
    let vec_refer: &[i32] = &vec_demo[3..9];        // [3, 4, 5, 6, 7, 8]

    /* Wait, what's 3..9?
        `3..9` means a range from 3 to 9, with 3 included and 9 excluded.
        If you want 9 be included, use `3..=9` instead.

        Range could be used to index an array, or put into a for loop like:
        ```
        for i in 0..10 {
            print!("{} ", i);
        }
        ```
        Output: 0 1 2 3 4 5 6 7 8 9 
        Just like in python you will do the same thing like:
        ```
        for i in range(0, 10):
            print(i, end=' ')
        ```
        Output: 0 1 2 3 4 5 6 7 8 9
    */

    /*
        Q: Methods on array and Vec?
        Q: What about other data structs I can use in Rust?

        A: Too much to talk about here! Refer to the rust documentaion
        by yourself! It's important to learn search and learn on your
        own!        
    */

}

#[allow(unused)]
fn p3_introduce_string() {
    /* String and &str.
        Basically, you can think String as Vec<...> and &str as &[...], id.
        est. Vector and slice reference containing some data.
        But it's worth mentioning that Rust String contains UTF-8 encoded
        values instead of ASCII encoded to support Unicode characters.
        It's both convenient and inconvenient. The pros are that we could
        store nearly any character on this planet as we need. The cons are 
        that counting are index the x-th character is much more complex.

        In Rust, generally we use `String`(aka std::string::String, but
        Rust std library has a set of default-imported types with String
        included already so you do not need to import it explicitly.)
        to express a string value. 
        Just like std::string in C++!

        Have you ever writed C? Then you are definitely familiar with
        `char *s = "foobar";`. That what &str is most like. But there's
        difference between them.
        In rust, &str is actually a reference(&) to a piece of exsiting
        continous characters, stored either in String or in static area
        of the process (id est, static value that's hard coded in source
        code and compiled into the target file)

        To illustrate the difference between 

        Ways of construct String:
        (1) `String::new()`: Create a new String with no content in it.
        (2) `String::from("Hello!")`: Create a new String containing "Hello!".
        (3) `"Hello!".to_string()`: The same as (2)
        (4) Convert from other types:
            `let s: String = 42.to_string()`: call .to_string() on a integer
            type and get a string value containing "42".
        (5) ...

        Ways of construct &str:
        (1) `let s_ref: &str = &String::from("asdf")[1..3]`: Refer to a String.
        (2) `let s_sta: &str = "asdf":`Construct manully.
    */
    let s: String = String::new();
    // Rust could infer the type of variable. (C++ auto).
    let s1 = String::new(); // So sometimes leave out type anno-
                                    // tation is allowed as long as Rust
                                    // could infer it.
    let s2 = String::from("Hello,String!");
    println!("s2: {}", s2); // s2: Hello,String!

    // s_str refer to the s2, from index 1 to index 5. [1, 5).
    let s_str: &str = &s2[1..5];
    println!("s_str: {}", s_str); // s_str: ello

    // s_static refer to a piece of hard-coded value.
    let s_static: &'static str =
        "This is a static value hard-coded in source file and shoule be compiled 
        into the target file, where this piece of value you see should be located
        in ro-data section (ready-only section), s_static refer to it.";
}

#[allow(unused)]
fn introduce_mutability() {
    /* What's mutability?
        Variables could be declared, defined and modified. That's common sense.
        
        But you don't want some thing you didn't expected to be accidentally
        modified somewhere in the program, do you?
        So a best practice in C/C++, actually, I think, is to add to every
        variable a "const" keyword and cancel it when you really need it to be
        modified somewhere.
        But as a human being, most of us just don't want to type extra five
        characters. In fact, in a program there's much more variables that
        do not need modification than those needs!

        So Rust makes everything immutable unless you declare it as mutable
        using keyword `mut`.
    */
    let foo: i32 = 42;
    // foo = 43; // Can't compile!
    let mut bar: i32 = 42;
    bar = 43;    // Fine!

    /* Shadowing
        Shadowing is a functionality that Rust provides to allow programmers
        spend less time considering what's the best name for the variable.
        
        For example, we want to transform a variable many times with many
        inter-status with different types. In C/C++, we have to name those
        inter-status each with  a different name because they have different
        types. But in Rust, we could just keep using the same name as long as
        we use `let` expression to "shadow" previous name.
    */
    let msg = 9; // Now msg's type is i32.
    let msg = msg.to_string(); // Now msg's type is String
    let msg = std::sync::Mutex::new(msg); // Now msg's type is Mutex<String>
    let msg = std::sync::Arc::new(msg); // Now msg'type is Arc<Mutex<String>>
    // By `let msg = msg.to_string()`, the fact that "msg refer to a i32 value 9"
    // no longer exists in the scope. In stead, now msg refer to a String value.
    // And the i32 value 9 that msg previously refers to are permanently lost and
    // dropped (destructed).
}

/* enum
    Just like what you would do in C with `enum` keyword.
    The example below would be written in C as:
    ```
    typedef enum {
        Red,
        Green,
        Blue,
    } RGB;
    ```
    However, Rust's enum is much more powerful than you might
    have thought, as least much more powerful than enum in C/C++!

    We would cover this later, but not for now.
*/
#[allow(unused)]
enum RGB {
    Red,
    Green,
    Blue,
}

/* struct
    Just like what you would do in C with `struct` keyword.
    The example below would be written in C as:
    ```
    typedef struct {
        int id;
        char* name;
        uint8_t age;
        char* description;
    } MyStruct;
    ```
*/
#[allow(unused)]
struct Person {
    id: i32,
    name: String,
    age: u8,
    description: Option<String>,
}

/* Add methods to your struct
    By doing so, we could call these methods.
    ```Rust
    let p = Person::new();
    let his-her-name = p.get_name();
    ```
*/
impl Person {
    fn new() -> Person {
        Person {
            id: 42,
            name: "John".to_string(),
            age: 20,
            description: Some("Hello, I am John!".to_string()),
        }
    }
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_desciption(&self) -> String {
        if self.description.is_some() {
            "The person's have description".to_string()
        } else {
            "The person have no description!".to_string()
        }
    }
}

#[allow(unused)]
fn introduce_option() {
    /* What is `Option`?
        `Option`(aka std::option::Option) is used when you want to express
        that "I want something that could contain some value but sometimes
        not."
        
        Option is an enum, in fact.
        ```
        pub enum Option<T> {
            None,
            Some(T),
        }
        ```
        `T` is called "generic parameter". We would not talk about it much
        here.
        But let's just make it clear that `Option<T>` means there could be
        "Some value typed `T`"(Some(T)), or just no value at all (None).

        So, in the definition of struct `Person`, the field `description`
        could presents if that person what to talk about him/herself, but
        absent if that person has some secrets.
        So we use `Option<String>` as the type of `Person::description`.

        You might have noticed that in corresponding C code, the field
        `description` is typed as `char *`. So when there's content, this
        `char *` value could just point to somewhere containing his/her
        desciption. But if not, that `char *` should be set to `NULL`, which
        correspond to `None` case in Rust.
        
        However, using pointer is unsafe (reinforce here!).
        And `NULL` is also a pointer, which will get you into a big trouble
        if you try to dereference it by throwing a `Segmentation fault` or
        something like this.

        So Rust does not provide `NULL` or `nullptr`.
        And C++ have noticed this as well. As result, they introduced
        std::optional in C++17 standard.
    */
    let a: Option<i32> = Some(1);
    let b: Option<i32> = None;
}

#[allow(unused)]
fn introduce_if_else() {
    /* if, else if, else
        It's basic functionality is same as in most programming languages.
    */
    let a: bool = true;
    let b: i32 = 9;
    if a {
        // do something
    } else if b != 9 {
        // do something
    }  else {
        // do something
    }

    /* More powerful than you might have thought!
        `if let` expression!
        Have a taste on "pattern matching" here!
        Which is a powerful functionality that's very common in functional
        programming languages.

        We'll cover this much more with `match` expression.
    */

    let demo: Option<String> = Some("This is a message".to_string());

    // Compare Some(msg) with Some("This is a message".to_string())
    // And we extracted "This is a message".to_string() with `msg`!

    if let Some(msg) = demo {
        // `demo`` contains value, and we get its inner value with pattern matching!
        println!("demo contains: {}", msg);
    } else {
        // `demo` is `None`(has no value).
        println!("demo has no value");
    }
}

/* Functions, parameters and return types.
    In rust, you can declare or define a function using kerwork `fn`.
    
    fn <function-name> (<param-name>: <param-type>, <param-name>: <param-type>, ...) -> <return-type> {
        <function body>
    }

    It's worth mentioning that the return value in Rust is an expression
    without semicolon `;`.
    Or you could just write as you would in C/C++. That's Ok.
    But if you want to return before a function ends, you must use `return`.

    Don't pay much attention to #[allow(unused)] for now.
    That's a macro making compiler happy, because we actually do not call
    the function `this_is_a_function` in the control flow of this program. It's
    just a example.
*/
#[allow(unused)]
fn this_is_a_function(a: String, b: bool) -> i32 {
    if b {
        return 1;
    } else {
        println!("{}", a);
    }
    42
} 

fn main() {}