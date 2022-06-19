

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
