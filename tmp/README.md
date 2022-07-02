
## Scratch 001

Rust is an `expression-based` language

`Statements` are instructions that perform some action and do not return a value. `Expressions` evaluate to a resulting value.

Calling a function is an expression, Calling a macro is an expression, A new scope block created with curly brackets is an expression.

## Scratch 002

Note that a reference's scope starts from where it is introduced and continues through the last time that reference is used.

The ability of the compiler to tell that a reference is no longer being used at a point before the end of the scope is called Non-Lexical Lifetimes (NLL).

## Scratch 003

- Field Init Shorthand
- Struct Update Syntax

Which expresses that a value can be either something or nothing.

Enums are a feature in many languages, but their capabilities differ in each language. Rust's enums are most similar to algebraic data types in functional languages, such as F#, OCaml, and Haskell.

## Scratch 004

In his 2009 presentation "Null Reference: The Billion Dollar Mistake", Tony Hoare, the inventor of null.

## Scratch 005

The compiler will look for the code inside the module in these places:

- Inline, directly following mod garden, within curly brackets instead of the semicolon
- In the file src/garden.rs
- In the file src/garden/mod.rs

## Scratch 006

We can also use pub to designate structs and enums as public, but there are a few extra details. If we use pub before a struct definition, we make the struct public, but the struct's fields will still be private.
In contrast, if we make an enum public, all of its variants are then public. We only need the pub before the enum keyword.

A path can take two forms:

- An absolute path starts from a crate root by using a crate name(for code from an external crate) or literal `crate` (for code from the current crate)
- A relative path starts from the current module and use `self`, `super`, or an identifier in the current module.

Idiomatic path Bring the function's parent module into scope with use, but specify the full path when bringing in structs, enums, and other item.

Re-exporting Names with pub use

```
use std::io;
use std::io::Write;
```

```
use std::io::{self, Write};
```

```
use std::collections::*
```

This `use` statement bring all public items defined in `std::collections` into the current scope. Be careful when using the glob operator. Glob can make it harder to tell what names in scope and where a name used in your program was defined. The glob operator is often used when testing to bring everything under test into the test module.

## Scratch 007

- String
- string slice
- &str

Rust has only one string type in the core language, which is the string slice `str` that is usually seen in its borrowed form `&str` .

`fn first_word(s: &String) -> &str`

A more experienced Rustacean would write the signature shown because it allows us to use the same function on both `&String` values and `&str` values.

`fn first_word(s: &str) -> &str`

This flexibility takes advatage of deref coercion. Rust uses a `deref coercion`, which here turns `&s` into `&s[..]` .

## Scratch 008

many more goodies are hiding in the functions defined on HashMap<K, V> by the standard library. As always, check the standard library documentation for more information.

The type annotation `HashMap<_, _>` is needed here because it's possible to collect into many different data structures and Rust does't know which you want unless you specify. For the parameters for the key and value types, however, we use underscores, and Rust can infer the types that the hash map contains based on the types of the data in the vectors.

## Scratch 009

Propagating errors. The `?` placed after a Result value is defined to work in almost the same way as the `match` expressions we defined to handle the `Result` value. If the value of the `Result` is an `Ok`, the value inside the `Ok` will get returned from the expression, and the program will continue. If the value is an `Err`, the `Err` will be returned from the whole function as if we had used the `return` keyword so the error value gets propagated to the calling code.

## Scratch 010

Generics allow us to replace specific types with a placeholder that represents multiple types to remove code duplication.

## Scratch 011

`&[T]`slice

```
// A heap-allocated array, coerced to a slice
let boxed_array: Box<[i32]> = Box::new([1, 2, 3]);
```

## Scratch 012

`fn NAME<T>` `struct NAME<T>` `enum NAME<T>`
`impl<T>`

We can use trait bounds to specify that a generic type can be any type that has certain behavior.

Use trait bounds improves performance without having to give up the flexibility of generics.

The only difference is that the user must bring the trait into scope as well as the types.

One restriction to note is that we can implement a trait on a type only if at leaset one of the trait or the type is local to our crate.

## Scratch 013

Instead of a concrete type of the item parameter, we specify the `impl` keyword and the trait name. This parameter accepts any type that implements the specified trait.

The `impl trait` syntax works for straightforward case but is actually syntax sugar for a longer form known as a trait bound.

- Trait Bound Syntax
- Specifying Multiple Trait Bounds with the + Syntax
- Clearer Trait Bounds with where Clauses

```
fn some_function(t: &(impl Display + Clone), u: &(impl Clone + Debug)) -> i32 {}
```

```
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
```

```
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug,
{}
```

- Trait as Parameters
- Returning Types that implement Traits

You can only use `impl Trait` if you returning a single type.

## Scratch 014

Most of the time, lifetimes are implicit and inferred.
We must annotate lifetimes when the lifetimes of references could be related in a few different ways. Rust requires us to annotate the relationships using geeric lifetime parameters to ensure the actual references used at runtime will definitely be valid.

The main aim of lifetimes is to prevent dangling reference.

- Preventing Dangling References with Lifetimes
- The Borrow Checker
- Generic Lifetimes in Functions
- Lifetime Annotiation Syntax
- Lifetime Annotations in Function Signatures
- Lifetime Annotations in Stuct Definitions

Lifetime annotations don't change how long any of the references live. Rather, they describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.

The lifetime annotations become part of the contract of the function.

Every reference has a lifetime and that you need to specify lifetime parameters for functions or structs that use reference.

lifetime elision rule

Because lifetime are a type of generic, the declarations of the lifetime parameter `'a` and the generic type parameter `T` go in the same list inside the angle brackets after the function name.

## Scratch 015

Some command line options go to `cargo test`, and some go to the resulting test binary. To separate these two types of arguments, you list the arguments that go to `cargo test` followed by the separator `--` and then the ones that go to the test binary.

## Scratch 016

Rust 有两种多态方式：

- Generic // compile time, static // impl trait
- Trait Object // run time, dynamic // dyn trait

- polymorphism
- monomorphism

## Scratch 017

Although we rarely need to annotate type in Rust, `collect` is one function you do often need to annotate because Rust isn't able to infer the kind of collection you want.

## Scratch 018

- associated type
- generic parameter

## Scratch 019

Boxes don't have performance overhead.

```
error[E0072]: recursive type `List` has infinite size
 --> tmp/examples/tmp010.rs:1:1
  |
1 | enum List {
  | ^^^^^^^^^ recursive type has infinite size
2 |     Cons(i32, List),
  |               ---- recursive without indirection
  |
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `List` representable
  |
2 |     Cons(i32, Box<List>),
  |               ++++    +
```