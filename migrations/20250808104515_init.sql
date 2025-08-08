-- Define ENUM type for roles
CREATE TYPE Roles AS ENUM (
    'admin',
    'seller',
    'buyer'
);

-- Create User table
CREATE TABLE "User" (
    id SERIAL PRIMARY KEY,
    email VARCHAR(64) NOT NULL UNIQUE,
    username VARCHAR(10) NOT NULL UNIQUE,
    password VARCHAR(60) NOT NULL,
    role Roles NOT NULL
);

-- Create Product table
CREATE TABLE Product (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(40) NOT NULL,
    description TEXT NOT NULL,
    price INTEGER DEFAULT 0 CHECK (price >= 0),
    rating SMALLINT DEFAULT 0 CHECK (rating >= 0 AND rating <= 5),
    owner_id INTEGER NOT NULL,
    FOREIGN KEY (owner_id) REFERENCES "User"(id) ON DELETE CASCADE
);

CREATE INDEX user_username_index ON "User"(username);
CREATE INDEX product_id_index ON Product(id);
