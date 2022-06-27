
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
