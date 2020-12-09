# Chapter 1

The quintessential special case of ownsership semantics in Rust involves usique
ownership over some resource. At one level of abstraction, move semantics allow
one to "atomically" change the identity and scope that is responsible/able to
delete some value without invalidating other, possibly nested, references. At
another level of abstraction, move semantics allow one to reuse temporary
variables by marking when the contents of a value can be "stolen." One common
implementation of the builder pattern uses this to initialize complex structs.
This chapter discusses examples of move semantics within (builder pattern) and
between (atomic ref counting) threads. 

## Notes from CPP

In CPP thes[{Rule of
5](https://isocpp.github.io/CppCoreGuidelines/CppCoreGuidelines\#S-ctor)(or
0,3,6,7, etc. depending on whom and when one
asks) is the lifeblood of RAII and how CPP relates objects and memory.
The CPP Core guidelines dedicate many subsequent clarifications. For
example, one should prefer the compiler-generated defaults unless there
is a good reason to do otherwise (C.20), IN particular, the destructor
(C.37) and move operations (C.66) should be `noexcept`. Every class X
must have the following:

```cpp
    class X{
        X(const X&)=default;
        operator=(const X&)=default;
        X(X&&)=default;
        X& operator=(X&&)=default;
        ~X()=default; 
    }
```

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
calling std::swap on nullptr and a pointer to the root node of a tree
(plus some minor clean up to preserve set invariants). 
