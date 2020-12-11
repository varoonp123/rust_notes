# What Problems Does Rust Attmept to Address

Many of the CPP Core guidelines are easier to follow with different language semantics. Relevant
areas that come to mind include immutability, resource deallocation, pattern matching, more
powerful iterator abstrations/adapters, and many more, We will explore some of these areas,
especially with respect to concurrency, and return to this later. Many of features in Rust exist to
help answer questions of the form "iAt which program states are given identity able/allowed to
inspect, mutate, and deallocate some resource or value?" To what degree can this delicate dance be
abstractly represented and enforced?


Many library features of Rust have similar counterparts in CPP. Compare, for example,
     [`std::Result`]() vs [`std::expected`](), [`std::Option`]() vs [`std::optional`](),
     [`std::Box`]() vs [`std::unique ptr`](), [`std::Rc`]() and [`std::Arc`]() vs
     [`std::shared`](), in Rust and CPP, respectively. The ownership rules apply rather naturally
     to Rust's pattern matching syntax, which is a welcome feature inspeired by the functional
     programming community.  See match, the ? operator, if let, slice/tuple destructuring, etc.


Compared to CPP, Rust has a useful ecosystem for "getting things done." It has good answers for
dependency management, package registry management, build specification, testing (an acceptably
    useful system for now), benchmarking, and documentation. In practice, these are extremely
important to reliable, reproducible, and usable software. 

The following are representative of many of the problems that Rust is capable of
solving. Who owns the nodes in a binary tree and controls when they (and their
descendents) are deleted? Who is responsible for cleaning up the forks in the
Dining Philosophers' problem? If a collection owns pointers to other objects,
does the destruction of the collection imply a deep destruction of its contents?
If a collection must be reallocated (e.x. a hash map or vector) and contents are
moved around in memory, what happens to references to the previous location of
the contents? Can parallel reads be non blocking? Many more.

While there may or may not be performance improvements from Rust's
implementation of these ideas in single threaded programs, the real prize is
that these abstractions at the language, library, and ecosystem levels are
powerful enough to enable safe multithreaded programs that may be been more
difficult to practically impossible in C or CPP. The task is daunting and the
burden of proof for determining whether or not Rust's ideas are useful is on the
collective experience of the Rust community over the next couple of decades.
When the time comes, I will discuss the lower half of this diagram and will
probably amend it because there are parts that I conceptualize differently. But
roughly speaking, [this is the
idea](https://github.com/usagi/rust-memory-container-cs). 

![Rust Indirection
Diagram](/introduction/rust_indirection_diagram.png)


It is helpful to make a loose distinction between a "value," a  and "identity,"
some address or pointer to a value. Of course this is complicated by the fact
that value in one context can be an identity in another, but we will come back
to this. In many functional languages, all values are immutable. Consequently
users can give many names to the same value and there is never any issue,
especially when garbage collection will remove the burden of deallocation from
the identities themselves. Haskell and Clojure, for example, take advantage of
this extensively to arrive at some elegent software. To paraphrase Rich Hickey,
using immutable values and pure functions does 80-90% of the heavy lifting
when creating a cornucopia of safe high level concurrency primitives. When one
adds algebraic specifications, this opens up the world of, for example,
distributed and high performance map reduce, in which there is some interest. At
some level of abstraction, however, everything is mutable. And Rust needs to be
able to speak this language to enter the ring for large classes of lower level
problems. The world of the mutable is where these indirection options are
especially important.
