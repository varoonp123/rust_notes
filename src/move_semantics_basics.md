# Move Semantics: What They Are and Why You Should Care

Our code frequently chains operations on data structures especially with unary
and binary functions, so we can focus on those. E.x.  Pandas dataframes,
matrices/ndarrays, sets, maps. While copying an integer may be ok, needlessly
copying entire matrices or dataframes can kill performance. CPP and Rust in
particular have relatively thorough and high level semantics for explicitly
controlling when to copy and when to "steal" an data structure's contents.
Rust's ownership system helps to avoid common bugs and adhere to many of the
CPP Core Guidelines.   

Consider the problem of merging two sets of strings. One way to accomplish this
to copy all of the node contents and have three independent sets. Another way
is to merge one set into another. This option involves mutating the target set.
Once again, one could copy all of the nodes. But one can also swap some
pointers so that nodes in the target set point to nodes in the source set. To
preserve unique ownership, it is useful to invalidate the pointers in the
source set, although the CPP standard makes no requirements about the state of
the object whose contents are "stolen." 

Indeed [std::set.merge](https://en.cppreference.com/w/cpp/container/set/merge)
has overloads to copy and move. One can also copy or move into the back of a
vector with
[std::vector.push_back](https://en.cppreference.com/w/cpp/container/vector/push_back)Move
semantics have been an essential piece of CPP since their standardization in
CPP11. 


In Rust, function parameters move by default, mimicking passing by r value
reference from CPP. While it was a CPP convention to never access a variable
after it has been moved out of (or atleast clear the container first), the
borrow checker turns using an identity after it has been moved a compile time
error. This permits safe, frequent moves.

The following example does not compile because the `mystring` identity has been
invalidated and `std::String` does not impl `Copy`. We could fix it by calling
`sink(mysink.clone())` and explicitly cloning the string.

```rust
{{#include move_sink_compile_error.rs}}
```

### A Miserable Meal

Consider the following solution to the [Dining Philosophers Problem](https://en.wikipedia.org/wiki/Dining_philosophers_problem)

```rust
{{#include dining_philosophers.rs}}

```

While we are on the topic of concurrency and unique ownership, the following CPP
Core Guidelines are still valid. It is still useful to assume that our code will
be run in a multithreaded program
([CP.1](https://isocpp.github.io/CppCoreGuidelines/CppCoreGuidelines#Rconc-multi)).
And many practices that are useful in a multithreaded environment are also
useful in single-threaded environments
([P.10](https://isocpp.github.io/CppCoreGuidelines/CppCoreGuidelines#Rp-mutable),
[R.6](https://isocpp.github.io/CppCoreGuidelines/CppCoreGuidelines#Rr-global),
[F.8](https://isocpp.github.io/CppCoreGuidelines/CppCoreGuidelines#Rf-pure))
create more situations when move semantics can be helpful.

