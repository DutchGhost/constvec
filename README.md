# Constvec
A library that offers a very simple Vec-like API useable in `const fn`.

```Rust
const fn example() -> i32 {
    let mut v = ConstVec::<_, 10>::new();

    v.push(10);
    v.push(20);

    match v.pop() {
        Some(elem) => elem,
        None => panic!("Expected at least one element")
    }
}
```