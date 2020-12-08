fn some_str() -> &mut str{
    let result = String::from("Hello there");
    &result

}
fn main(){

    println!("{}", some_str());

}
