fn sink<T>(_: T) {}
fn sink_ref<T>(_: &T) {}
fn sink_ref_mut<T>(_: &mut T) {}
fn main() {
    let mut mystring = String::from("HEllo");
    // No compile time error. The owner of some value can "create" other identities and "lend" them
    // out to other scopes.
    sink_ref(&mystring);
    sink_ref(&mut mystring);
    sink(mystring); // The identity `mystring` is moved into the scope for `sink.`
                    // BOOM!. The `println` is a Compile time error because `mystring` has been
                    // moved.
    println!("{}", mystring);
}
