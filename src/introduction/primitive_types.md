# Notes on Unique Ownership in Rust


Each value is owned by an identity. There can only be one owner at a time. When
that owner goes out of scope, the value is dropped. 
A [`std::String`] is a uniquely owned heap allocated string. A [`&str`] is an
immutably borrowed view into that a primitive [`str`] type. [`&String`] derefs
to [`&str`]. Only the variable that controls the [`String`]() (not those that
see the views) needs to/is allowed to to free the underlying string memory. The
String identity is not allowed to free the underlying value until the other
(possibly mutable) identities have gone out of scope.
