### Error Handling

The last example in [this article](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html) shows how to return the
`std::err::Error` trait from a function that calls several functions each returning a different error type ( std::fmt::Error, std::io::Error, and
so forth ).

### Supertraits

Though Rust does not support inheritance on types, it allows programmers to define an hierarchy of traits. Here are two articles that explain the
concept of supertraits:

1. [Supertraits from Rust by Example](https://doc.rust-lang.org/rust-by-example/trait/supertraits.html)
2. [Using Supertraits](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-supertraits-to-require-one-traits-functionality-within-another-trait)


### References

[Week 12](https://this-week-in-rust.org/blog/2013/08/25/this-week-in-rust-12/) of This Week in Rust.
