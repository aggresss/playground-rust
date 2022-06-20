
Closures succinctly capture variables from enclosing scopes. Does this have any consequences? It surely does. Observe how using a closure as a function parameter requires generics, which is necessary because of how they are defined:


```rust
// `F` must be generic.
fn apply<F>(f: F) where
    F: FnOnce() {
    f();
}
```

**When a closure is defined, the compiler implicitly creates a new anonymous structure to store the captured variables inside, meanwhile implementing the functionality via one of the traits: `Fn`, `FnMut`, or `FnOnce` for this unknown type. This type is assigned to the variable which is stored until calling.**

Since this new type is of unknown type, any usage in a function will require generics. However, an unbounded type parameter `<T>` would still be ambiguous and not be allowed. Thus, bounding by one of the traits: `Fn`, `FnMut`, or `FnOnce` (which it implements) is sufficient to specify its type.

- borrow
- move

- Fn // 可以捕获 引用
- FnMut // 可以捕获 可变引用
- FnOnce // 可以捕获 移动

Rust 中 borrow 只存在两种情况

- 多个 &
- 一个 &mut

使用 move 会强制闭包取得被捕获变量的所有权
关键字 move 的作用是将所引用的变量所有权转移至闭包内，通常用于使闭包的声明周期大于所捕获的变量的原生命周期，例如将闭包返回或移至其他线程。
发散函数不会返回，使用 ! 标记，表示一个空类型。

