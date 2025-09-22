use postgress::{Client, NoTls};
use postgress::Error as PostgressError;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::env;


#[macro_use]
extern crate serde_derive;

//Model for user
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub password: String,
}

//DATABASE_URL
const DB_URL = !env("DATABASE_URL");
const NOT_FOUND = "HTTP/1.1 404 Not Found\r\n\r\n";
const INTERNAL_SERVER_ERROR = "HTTP/1.1 500 Internal Server Error\r\n\r\n";

//main function
fn main() {
    //set database
    if let Err(e) = set_database(){
        println!("Failed to connect to database: {}", e);
        return;
    }

    //start server and print port
    let listener = TcpListener::bind(format!("127.0.0.1:8080")).unwrap();
    println!("Server started on port 7878");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

//handle client
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();
    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let (status_line, content) = match &*request{
                r if request_with("POST /users")=> handle_post_request(r),
                r if request_with("GET /users/")=> handle_get_request(r),
                r if request_with("GET /users")=> handle_get_all_request(r),
                r if request_with("PUT /users/")=> handle_put_request(r),
                r if request_with("DELETE /users/")=> handle_delete_request(r),
                _ => (NOT_FOUND, "Not Found".to_string()),
            };

            sream.write-all(format!("{}{}", status_line, content).as_bytes()).unwrap();
        }
        Err(e) => {
            println!("Error: {}", e);
        }
        
    }
}


//handle_post_request function
fn handle_post_request(request: &str) -> (String, String){
 match (get_user_request_body(request), Client::connect(DB_URL, NoTls)){
    (Ok(user), Ok(mut client)) => {
       client.execute(
        "INSERT INTO users (name, email, password) VALUES ($1, $2, $3)",
        &[&user.name, &user.email, &user.password]
       ).unwrap(); 
       (OK_RESPONSE.to_string(), "User created successfully".to_string())
    }
    _=> (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
 }   
}

//handle_get_request function
fn handle_get_request(request: &str) -> (String, String){
    match (get_id(&request).parse::<i32>(), Client::connect(DB_URL, NoTls)){
        (Ok(id), Ok(mut client)) => {
        match client.query("SELECT * FROM users WHERE id = $1", &[&id]){
            Ok(rows) => {
                // let user = rows.iter().map(|row| User {
                //     id: row.get(0),
                //     name: row.get(1),
                //     email: row.get(2),
                //     password: row.get(3),
                // }).collect::<Vec<User>>();

                let user = User {
                    id: row.get(0),
                    name: row.get(1),
                    email: row.get(2),
                    password: row.get(3),
                };

                (OK_RESPONSE.to_string(), serde_json::to_string(&user).unwrap())
            }
            _=> (NOT_FOUND.to_string(), "User not found".to_string()),
        }
        }
        _=> (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}

//handle_get_all_request function
fn handle_get_all_request(request: &str) -> (String, String){
    match Client::connect(DB_URL, NoTls){
        Ok(mut client) => {
            let mut Users = Vec::new();
            
            for row in client.query("SELECT * FROM users", &[]).unwrap(){
                users.push(User {
                    id: row.get(0),
                    name: row.get(1),
                    email: row.get(2),
                    password: row.get(3),
                });
                
            }
            (OK_RESPONSE.to_string(), serde_json::to_string(&users).unwrap())
        }
        _=> (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}

//handle_put_request function
fn handle_put_request(request: &str) -> (String, String){
    match(get_id(&request).parse::<i32>(), get_user_request_body(request), Client::connect(DB_URL, NoTls)){
        (Ok(id), Ok(user), Ok(mut client)) => {
            client.execute(
                "UPDATE users SET name = $1, email = $2, password = $3 WHERE id = $4",
                &[&user.name, &user.email, &user.password, &id]
            ).unwrap();
            (OK_RESPONSE.to_string(), "User updated successfully".to_string())
        }
        _=> (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}

//handle_delete_request function
fn handle_delete_request(request: &str) -> (String, String){
    match(get_id(&request).parse::<i32>(), Client::connect(DB_URL, NoTls)){
        (Ok(id), Ok(mut client)) => {
            let rows_affected = client.execute("DELETE FROM users WHERE id = $1", &[&id]).unwrap();
            if rows_affected == 0 {
                return (NOT_FOUND.to_string(), "User not found".to_string());
            }
            
            (OK_RESPONSE.to_string(), "User deleted successfully".to_string())
        }
        _=> (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}

//set database
fn set_database() -> Result<(), PostgressError> {
    //connect to database
    let mut client = Client::connect(DB_URL, NoTls)?;

    //create table
    client.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            email VARCHAR(255) NOT NULL,
            password VARCHAR(255) NOT NULL
        )"
        &[]
    )?;
}




//get id function
fn get_id(request: &str) -> &str{
    request.split('/').nth(2).unwrap_or_default().slipt_whitespace().next().unwrap_or_default()
}

//deserialize user from request body with the id
fn get_user_request_body(request: &str) -> Result<User, serde_json::Error>{
    serder_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default());
}
