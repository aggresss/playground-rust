
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
