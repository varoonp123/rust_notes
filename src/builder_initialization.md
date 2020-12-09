# Initialization and the Builder Pattern

Many facts about complex initialization with the builder pattern follow from the
proposition that initialization is typically a function invocation.  For
example, if one replaces curly braces with parens, stuct initialization looks
like a function call using keyword arguments. This analogy is rather tight ln
CPython at
[multiple](https://docs.python.org/3/reference/datamodel.html#object.__init__)
levels of
[abstraction](https://docs.python.org/3/c-api/typeobj.html#c.PyTypeObject.tp_init). 

```rust
struct Container{
    field1: Vec<String>,
    field2: usize,
}

// 2 moves. One copy.
let container = Container{
    field1: vec![String::from("field1")],
    field2: 2,
};
```
Classmethods and the builder pattern allow one to set different kinds of
sensible defaults. The builder pattern is also useful for separating an internal
representation that is useful for computations and a dictionary into that
internal representation that is more user frieldly. This pattern
[shows](https://guava.dev/releases/19.0/api/docs/com/google/common/collect/MapMaker.html)
[up](https://docs.oracle.com/javase/8/docs/api/java/util/Calendar.Builder.html)
[frequently](https://docs.oracle.com/javase/7/docs/api/java/util/Locale.Builder.html)
in the Java world.

While there are few functions in the standard library that rely on the builder
pattern it does show up in multiple places. In practice, the builder pattern
might look like this.

```rust
let file = std::fs::OpenOptions::new().write(true).create_new(true).open()?;
```

In the spirit of RAII, ownership is also determined at instantiation. To the
best knowledge of the author, at instantiation, every struct has unique
ownership. This is important because it has implications for destruction. The
builder pattern, when used properly, will respect this unique ownership when the
`.build()` method is eventually called. See the discussion on the relationship
between the builder pattern and move semantics for more details on how this
works. Along with macros, the builder pattern is likely the most flexible
initialization regime available in Rust as of December 2020. 

Of course we can compose the builder pattern with the
[`std::convert::From`](https://doc.rust-lang.org/std/convert/trait.From.html),
use move semantics with both, and get a result that is functionally similar to
polymorphically `.build()`ing. We also achieve polymorphism via iterators and
the [`std::iter::FromIterator`]() traits. While is it less frequent to see the
builder pattern on containers, one gets many of the same benefits of the builder
pattern using iterators. 

The fact that we do not have to worry about inheritance simplifies the
initialization situation greatly and the efficacy of the builder pattern is a
consequence. 
