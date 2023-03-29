# cudd-rs

Cudd Rust Library

### Example
```rust
fn test() {
    let mut cudd = Cudd::new();
    let var0 = cudd.new_var();
    let var1 = cudd.new_var();
    let _and = &var0 & &var1;
    let _or = &var0 | &var1;
    let _xor = var0 ^ var1;
}
```
