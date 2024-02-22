use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub preferences: UserPreferences,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPreferences {
    // Define the fields related to user preferences here
}

// Enums and any additional structs would be defined here
