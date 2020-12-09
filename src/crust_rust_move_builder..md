# Move Semantics, Initialization, and the Builder Pattern

The quintessential special case of ownership in Rust involves unique
ownership over some resource. At one level of abstraction, move semantics allow
one to "atomically" change the identity and scope that is responsible/able to
delete some value without invalidating other, possibly nested, references. At
another level of abstraction, move semantics allow one to reuse temporary
variables by marking when the contents of a value can be "stolen." One common
implementation of the builder pattern uses this to initialize complex structs.
This chapter discusses examples of move semantics within (builder pattern) and
between (atomic ref counting) threads. 
