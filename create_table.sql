CREATE TYPE user_power AS ENUM ('admin', 'user');

CREATE TABLE user (
    id SERIAL PRIMARY KEY,
    power user_power NOT NULL,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255)
    password VARCHAR(255) NOT NULL,
    description VARCHAR(255),
    programe_link TEXT,
);
