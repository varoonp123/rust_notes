# Move Semantics and Initialization: A Detour Into CPP

The [Rule of
5](https://isocpp.github.io/CppCoreGuidelines/CppCoreGuidelines\#S-ctor) (or
0,3,6,7, etc.) depending on whom and when one
asks) is the lifeblood of RAII and how CPP relates objects and memory.
The CPP Core guidelines dedicate many subsequent clarifications. For
example, one should prefer the compiler-generated defaults unless there is a
good reason to do otherwise
([C.20](https://isocpp.github.io/CppCoreGuidelines/CppCoreGuidelines#Rc-zero)).
In particular, the destructor
([C.37](https://isocpp.github.io/CppCoreGuidelines/CppCoreGuidelines#Rc-dtor-noexcept))
and move operations
([C.66](https://isocpp.github.io/CppCoreGuidelines/CppCoreGuidelines#c66-make-move-operations-noexcept))
should be `noexcept`. Every class X must have the following:

```cpp
    class X{
        X(const X&)=default;
        operator=(const X&)=default;
        X(X&&)=default;
        X& operator=(X&&)=default;
        ~X()=default; 
    }
```

To the best knowledge of the author, it is not possible to override the default destructor in Rust. This consists of the following:

> The destructor of a type `T` consists of
>  1. If `T: Drop`, calling `<T as std::ops::Drop>::drop`
>  2. Recursively running the destructor of all of its fields.
>
>        * The fields of a struct are dropped in declaration order.
>        * The fields of the active enum variant are dropped in declaration order.
>        * The fields of a tuple are dropped in order.
>        * The elements of an array or owned slice are dropped from the
>        first element to the last.
>        * The variables that a closure captures by move are dropped in an
>        unspecified order.
>        * Trait objects run the destructor of the underlying type.
>        * Other types don't result in any further drops.

For more, see the [Rust Reference](https://github.com/rust-lang/reference/blob/a8afdca5d0715b2257b6f8b9a032fd4dd7dae855/src/destructors.md).
The Rust analog of the destructor is the Drop trait, which we ignore
for now. The two ampersands denote an r value reference, a language
construct introduced in CPP11 to tag a reference as being temporary in
some sense. The move constructor and move assignment operators leave
the input in a valid but unspecified state, meaning that all class
invariants must be preserved. In particular, move semantics typically
involves moving identities (pointers and references) or scopes, rather
than values (I try to clear my data structures explicitly after moving
out of them). Copy constructing a set, for example, might invoke a copy
for every node. Move constructing a set, by contrast, may only involve
calling std::swap on `nullptr` and a pointer to the root node of a tree
(plus some minor clean up to preserve set invariants). 


Other important uses of move semantics in the Rust standard library include the following.

1. The primary conversion traits
   [`std::convert::From`](https://doc.rust-lang.org/std/convert/trait.From.html#tymethod.from})
   and its sister
   [`std::convert::TryFrom`](https://doc.rust-lang.org/std/convert/trait.TryFrom.html#tymethod.try_from).
   Structs that impl these traits can be safely converted to other types while
   reusing as much as possible.
2. [`std::iter::FromIterator`](https://doc.rust-lang.org/std/iter/trait.FromIterator.html)
which is useful for creating collections. This is typically called with
[`std::iter::Iterator::collect`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect),
sometimes with the target collection as a type parameter.
3. [Monadic](https://doc.rust-lang.org/std/option/enum.Option.html#method.and_then) and
   [functorial](https://doc.rust-lang.org/std/option/enum.Option.html#method.map)
   composition in the `Result` and `Option` types. The fact that these methods
   consume `self` means that the primary overhead to these type constructors,
   besides that of the function being lifted, relates to moving a couple of
   pointers. This means that the types effectively convey intent with little
   overhead. 

## Move Initialization: An Example

Nikolai Josuttis spoke at CppCon 2017 about [The Nightmare of Move Semantics for
Trivial Classes](https://www.youtube.com/watch?v=PNRju6_yn3o&t) and described
the thought that goes in to having as many paths towards initializing a CPP
class use the minimal number of copies. His state after an hour ends up being
rather messy, relying on forwarding references (dispatch to copy vs move
semantics based on the input value category and context), templates (conversions
to and from string types), and type traits (virtual initialization/inheritance).

```rust
#[derive(Debug, Clone, Default)]
struct Cust{
    first: String,
    last: String,
    id: isize
}

impl Cust{
    pub fn from_name(first: String, last: String) -> Cust{
        Cust{
            first,
            last,
            id: 0,
        }
    }
}

// If the fields were public, this would work and the strings would be moved
// into/be owned by the structs.
let c = Cust{first: String::from("First"), last: String::from("abc"), id: 1};
let c1 = Cust::from_name(String::from("First"), String::from("abc"));
// Automatically implemented. `c` is moved into `d`. "Move construction"
let d = Cust::from(c);
// Move out of c1. Move assignment
let d1 = c1;

// All destructors/Drop traits are called at the end of this scope. Heap data
// is freed.
```

If `Cust` had references, then we could make constructors generic over types
whose references can be cast into an `&str` using the
[`std::convert::AsRef`](https://doc.rust-lang.org/std/convert/trait.AsRef.html)
trait. 

In Rust we get to avoid many CPP move semantics and initialization pitfalls.

1. (How) should we move assign/construct to a subclass? To a superclass? No inheritance.
2. Is anyone going to (re)use the struct out of which one moved? This is a
   compile time error. In CPP, this is unspecified behavior and possibly a lint
   error.
3. Copy constructors/assignments. We have `Copy` and `Clone` derive macros and
   traits.
4. Destructors. We have the `Drop` trait.
