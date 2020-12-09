struct DatabaseConnection{
    client_id: isize,
    queries: Vec<String>,
    url: String,
}

let x = DatabaseConnection{
    client_id: 1,
    queries: vec![String::from("SELECT field1 from table24")],
    url: String::from("localhost:3000")
};
