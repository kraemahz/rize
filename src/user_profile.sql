-- This SQL script contains the schema definition and queries for the user profile system

-- Schema definition for user profile
CREATE TABLE user_profiles (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    first_name VARCHAR(255),
    last_name VARCHAR(255),
    avatar_url TEXT
);

-- Placeholder queries for CRUD operations
-- Insert a new user profile
INSERT INTO user_profiles (username, email, first_name, last_name, avatar_url) VALUES ($1, $2, $3, $4, $5);

-- Retrieve a user profile by its ID
SELECT * FROM user_profiles WHERE id = $1;

-- Update an existing user profile
UPDATE user_profiles SET username = $1, email = $2, first_name = $3, last_name = $4, avatar_url = $5 WHERE id = $6;

-- Delete a user profile by its ID
DELETE FROM user_profiles WHERE id = $1;
