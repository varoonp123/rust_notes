# Pattern Matching, Result, and Option

Pattern matching allows one to elegantly unpack/destructure structs, enums,
arrays, and tuples. More generally, they allow one to dispatch based on some
conditions in a way that can provide compile-time guarantees that all
possibilities/patterns have been exhausted. While many C family languages permit
one to, for example, match tuple patterns, few take the idea as far as some of
the functional languages, namely Haskell. The creators of Rust believe that pattern matching is so powerful and elegant
that they made it central to two standard library types that together are
atleast as important for control flow as the `if` statement. 

In this chapter, we discuss pattern matching in some generality and explore many
useful patterns for flexibly dealing with many core types. We will also explore
[`std::option::Option`]() and [`std::result::Result`]() to highlight the degree
to which one can use pattern matching with zero cost abstractions to get
expressive yet performant code.
