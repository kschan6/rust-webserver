-- Describe the table "posts"
CREATE TABLE posts (
id SERIAL PRIMARY KEY,
body TEXT NOT NULL,
published TIMESTAMP NOT NULL DEFAULT NOW()
)
