# Constvec
A library that offers a verry simple Vec-like API useable in `const fn`.

```Rust
const fn example() {
    let mut v = ConstVec::<_, 10>::new();

    v.push(10);
    v.push(20);
}