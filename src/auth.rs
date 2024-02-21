use serde_json::{json, Value};
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use actix_web::{web, HttpResponse, Responder};
pub mod auth {
    use std::collections::HashMap;

    use serde_json::json;

    use serde::{Deserialize, Serialize};

    use lazy_static::lazy_static;

    use actix_web::{web, HttpResponse, Responder};

    use super::*;

    // Placeholder for authentication logic

    lazy_static! {
        static ref USER_CREDENTIALS: HashMap<String, String> = {
            let mut m = HashMap::new();
            m.insert("test_user".to_string(), "test_password".to_string());
            m
        };
    }

    pub fn verify_user_credentials(credentials: &LoginCredentials) -> bool {
        if let Some(stored_password) = USER_CREDENTIALS.get(&credentials.username) {
            return stored_password == &credentials.password;
        }
        false
    }

    #[derive(Deserialize, Serialize)]
    pub struct LoginCredentials {
        pub username: String,
        pub password: String,
    }

    #[derive(Serialize)]
    pub struct LoginResponse {
        pub message: String,
        pub user_data: Option<serde_json::Value>,
    }

    pub async fn authenticate_and_respond(credentials: web::Json<LoginCredentials>) -> impl Responder {
        if verify_user_credentials(&credentials) {
            let user_data = json!({"profile": "Profile data", "content": "Personalized content"});
            HttpResponse::Ok().json(LoginResponse {
                message: "Login successful",
                user_data: Some(user_data),
            })
        } else {
            HttpResponse::Unauthorized().json(LoginResponse {
                message: "Invalid username or password",
                user_data: None,
            })
        }
    }
}

pub struct LoginRequest {
    username: String,
    password: String,
}
pub async fn login(form: web::Json<LoginRequest>) -> impl Responder {
    if verify(&form.username, &form.password) {
        HttpResponse::Ok().json(LoginResponse { message: String::from("Welcome!") })
    } else {
        HttpResponse::Unauthorized().json(LoginResponse { message: String::from("Unauthorized") })
    }
}
lazy_static! {
    static ref USER_CREDENTIALS: HashMap<String, String> = {
        let mut m = HashMap::new();
        // This is a placeholder for user credentials.
        // In a real-world scenario, you should retrieve these from a database and store passwords securely (hashed and salted).
        m.insert("test_user".to_string(), "test_password".to_string()); // Example user
        m
    };
}

    [derive(Deserialize, Serialize)]

    {
    static ref USER_CREDENTIALS: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("test_user".to_string(), "test_password".to_string());
        m
    };
}
pub fn verify_user_credentials(credentials: &LoginCredentials) -> bool {
    if let Some(stored_password) = USER_CREDENTIALS.get(&credentials.username) {
        return stored_password == &credentials.password;
    }
    false
}
#[derive(Deserialize, Serialize)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String,
}
// all other code from the file remains the same
#[derive(Serialize)]


#[derive(Serialize)]
pub struct LoginResponse {
    message: String,
}
pub async fn authenticate_and_respond(credentials: web::Json<LoginCredentials>) -> impl Responder {
    if verify_user_credentials(&credentials) {
        let user_data = json!({ "username": credentials.username });
        HttpResponse::Ok().json(LoginResponse {
            message: "Login successful".to_string(),
            user_data: Some(user_data)
        })
    } else {
        HttpResponse::Unauthorized().json(LoginResponse {
            message: "Invalid username or password".to_string(),
            user_data: None
        })
    }
}
// The rest of the previous code block remains the same
// ... rest of the file remains unchanged
lazy_static! {
    static ref USER_CREDENTIALS: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("test_user".to_string(), "test_password".to_string());
        m
    };
}

    [derive(Deserialize, Serialize)]

    {
    static ref USER_CREDENTIALS: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("test_user".to_string(), "test_password".to_string()); 
        m
    };
}

    {
    static ref USER_CREDENTIALS: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("test_user".to_string(), "test_password".to_string()); // Example user
        m
    };
}

    {
    static ref USER_CREDENTIALS: HashMap<String, String> = {
        let mut credentials = HashMap::new();
        credentials.insert("user1".to_string(), "password1".to_string());
        credentials
    };
}

    {
    static ref USER_CREDENTIALS: HashMap<String, String> = {
        let mut m = HashMap::new();
        // This is a placeholder for user credentials.
        // In a real-world scenario, you should retrieve these from a database and store passwords securely (hashed and salted).
        m.insert("test_user".to_string(), "test_password".to_string()); // Example user
        m
    };
}

    {
    static ref USER_CREDENTIALS: HashMap<String, String> = {
        let mut credentials = HashMap::new();
        credentials.insert("user1".to_string(), "password1".to_string());
        credentials
    };
}

    {
    static ref USER_CREDENTIALS: HashMap<String, String> = {
        let mut m = HashMap::new();
        // This is a placeholder for user credentials.
        // In a real-world scenario, you should retrieve these from a database and store passwords securely (hashed and salted).
        m.insert("test_user".to_string(), "test_password".to_string()); // Example user
        m
    };
}
pub async fn authenticate_user(credentials: web::Json<LoginCredentials>) -> impl Responder {
    if let Some(expected_password) = USER_CREDENTIALS.get(&credentials.username) {
        if &credentials.password == expected_password {
            let user_data = json!({ "personal_info": "Value related to user1" });
            return HttpResponse::Ok().json(LoginResponse {
                message: "Successfully authenticated".to_string(),
                user_data: Some(user_data),
            });
        }
    }

    HttpResponse::Unauthorized().json(LoginResponse {
        message: "Invalid credentials".to_string(),
        user_data: None,
    })
}
#[derive(Serialize)]


lazy_static! {
    static ref USER_CREDENTIALS: HashMap<String, String> = {
        let mut credentials = HashMap::new();
        credentials.insert("user1".to_string(), "password1".to_string());
        credentials
    };
}

    [derive(Deserialize, Serialize)]

    {
    static ref USER_CREDENTIALS: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("test_user".to_string(), "test_password".to_string()); 
        m
    };
}

    {
    static ref USER_CREDENTIALS: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("test_user".to_string(), "test_password".to_string());
        m
    };
}

    {
    static ref USER_CREDENTIALS: HashMap<String, String> = {
        let mut users = HashMap::new();
        users.insert("User1".to_string(), "Password1".to_string());
        users
    };
}

    {
    static ref USER_CREDENTIALS: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("test_user".to_string(), "test_password".to_string());
        m
    };
}
/// Verifies if the provided credentials are correct.
pub fn verify_login_credentials(credentials: &LoginCredentials) -> bool {
    if let Some(stored_password) = USER_CREDENTIALS.get(&credentials.username) {
        stored_password == &credentials.password
    } else {
        false
    }
}
/// Handler for the login API endpoint.
pub async fn login_handler(info: web::Json<LoginRequest>) -> impl Responder {
    if USER_DATABASE.get(&info.username) == Some(&info.password) {
        HttpResponse::Ok().json(LoginResponse { message: "Logged in successfully" })
    } else {
        HttpResponse::BadRequest().json(LoginResponse { message: "Incorrect username or password" })
    }
}
lazy_static! {
    static ref USER_CREDENTIALS: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("test_user".to_string(), "test_password".to_string()); 
        m
    };
}

    {
    static ref USERS: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("test_user".to_string(), "test_pass".to_string());
        m
    };
}

    {
    static ref USER_DATABASE: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("test_user".to_string(), "password123".to_string());
        m
    };
}

    {
    static ref USERS: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("user".to_string(), "password".to_string());
        m
    };
}

    {
    static ref USERS: HashMap<String, String> = {
        let mut users = HashMap::new();
        users.insert("Alice".to_string(), "wonderland".to_string());
        users.insert("Bob".to_string(), "builder".to_string());
        users.insert("Clementine".to_string(), "ohmydarling".to_string());
        users
    };
}

    {
    static ref USER_DATABASE: HashMap<String, String> = {
        let mut m = HashMap::new();
        // In a real application, use hashed passwords
        m.insert("test_user".to_string(), "test_pass".to_string());
        m
    };
}

    {
    static ref USER_CREDENTIALS: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert(String::from("user1"), String::from("password1"));
        m
    };
}

    {
    static ref USER_CREDENTIALS: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("test_user".to_string(), "test_password".to_string());
        m
    };
}

    {
    static ref USER_CREDENTIALS: HashMap<String, String> = {
        let mut users = HashMap::new();
        users.insert("User1".to_string(), "Password1".to_string());
        users
    };
}
fn verify_credentials(username: &str, password: &str) -> bool {
    if let Some(stored_password) = USER_CREDENTIALS.get(username) {
        return stored_password == password;
    }
    false
}
pub fn verify(username: &str, password: &str) -> bool {
    // This should be an actual verification against a user service or database
    username == "admin" && password == "secret"
}