# Initialization in Rust: A Quick Guide

The following assumes that one is reasonably familar with Every initialization of a struct accomplishes much beyond merely allocating some
data and managing pointers. While I make a distinction between "value" and
"identities", one should recall that identities can be values in another
context. For example, many smart pointers behave this way.
[`istd::sync::Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html) or an
atomic type can be (owned) struct members.  

1. Mutability of the value is set with the `mut` keyword, with many of the
   standard associated suggestions still applying,
   ([Con.1](https://isocpp.github.io/CppCoreGuidelines/CppCoreGuidelines#Rconst-immutable),
   [Con.5](https://isocpp.github.io/CppCoreGuidelines/CppCoreGuidelines#Rconst-constexpr)),
   where `constexpr` in Cpp is roughly `const` `in Rust.
2. The lifetime of the (unique) identity and the underlying value are set to
   that of the scope of the (unique) identity.  

All of this can be manipulated, of course, with smart pointers, macros, and
unsafe blocks. In particular, manipulating raw pointers, which are indeed a
(nostd) [`primitive
type`](https://doc.rust-lang.org/std/primitive.pointer.html), can bypass all of
these ownership guarantees. To the best knowledge of the author, any use of raw
pointers must be wrapped in an unsafe block. 

## Common Techniques

There are [two or three ways to initialize an object in
CPP](https://i.imgur.com/3wlxtI0.gifv). Those techniques become immensely more
complicated by inheritance. We will come back to this. In Rust, the situation is
simpler. 


![Relevant](./gump_cpp_initialization.gif)

### Struct initialization

This is simple and does exactly what it needs to for small record types. If the struct
has internal state that is not user frieldly, then another method is needed.

```rust

struct QueryString{
    query: String,
    executed: bool,
}

let qstring = QueryString{
    query: String::from("Does elves exist?"),
    executed: false,
    };

// If the variable name and the struct field name are the same, then the field
// names in the initializer are optional.
let query = String::from("SELECT DISTINCT filenames FROM obstore");
let executed = true;

let filename_query = QueryString{
    query,
    executed,
    };
```


### [`std::iter::FromIterator`](https://doc.rust-lang.org/std/iter/trait.FromIterator.html)

This is especially useful for building collections.
```rust
// Note how the `collect` function collects an iterable of results into a 
// result of an iterable.
let positive_nums = -5..10
    .iter()
    .map(usize::try_from)
    .collect::<Result<Vec<usize>>>()?;
```

Being able to pull the `Result` outside of the iterator is surprisingly useful.
We will discuss initialization via conversion, and the analogs in Cpp, in a bit.

### Macros

Because Rust does not have variadic functions, let alone generic variadic
functions, we frequently use macros to initialize structs. At some level of
abstraction, macros provide a (compile time) domain specific language for text
replacement. This permits different user facing invocation styles. The most used
example is probably [`vec!`](https://doc.rust-lang.org/std/macro.vec.html)

```rust
let primes = vec![2, 3, 5, 7, 11];
```

These are a story for another day. Of course, because macros "write" more Rust
code, any initialiation done with this method will use one of the others. 

### Factory Methods

Many structs provide helper associated functions that return instances of the
struct. These are frequently called `::new()`. 

