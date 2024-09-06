CREATE TABLE user (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255)
    password VARCHAR(255) NOT NULL,
    description VARCHAR(255),
    programe_link TEXT,
);
