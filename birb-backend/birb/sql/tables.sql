-- schemas
CREATE SCHEMA blog;
CREATE SCHEMA auth;

-- table of posts in the db
CREATE TABLE IF NOT EXISTS blog.posts (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    title TEXT NOT NULL,
    author TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- table of refresh tokens
CREATE TABLE IF NOT EXISTS auth.refresh_tokens (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    hashed_token TEXT NOT NULL UNIQUE,
    exp_at TIMESTAMP NOT NULL,
    issued_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    revoked BOOLEAN DEFAULT FALSE
);

-- table of admins registered
CREATE TABLE IF NOT EXISTS auth.admins (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    hashed_pass BYTEA NOT NULL,
    salt BYTEA NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    issued_by INET -- may be null if issued by a "unknown" entity 😨
);
