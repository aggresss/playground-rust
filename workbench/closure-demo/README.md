

- Fn // 可以捕获 引用
- FnMut // 可以捕获 可变引用
- FnOnce // 可以捕获 移动


- move

## Generic bounds

```rust
impl<A: TraitB + TraitC, D: TraitE + TraitF> MyTrait1<A, D> for YourType {}
```

```rust
impl<A, D> MyTrait2<A, D> for YourType
where
    A: TraitB + TraitC,
    D: TraitE + TraitF,
{
}
```
