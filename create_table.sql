CREATE TYPE user_power AS ENUM ('admin', 'user');

CREATE TABLE user (

    id SERIAL PRIMARY KEY,

    password VARCHAR(255) NOT NULL,

);

CREATE TABLE programe (

    id SERIAL PRIMARY KEY,
    
    description VARCHAR(255) NOT NULL,
    
    link TEXT NOT NULL

    profile_id INTEGER,

    FOREIGN KEY(profile_id) REFERENCES profile(id)
);


CREATE TABLE profile (

    id SERIAL PRIMARY KEY,

    power user_power NOT NULL,

    email VARCHAR(255) UNIQUE NOT NULL,

    name VARCHAR(255) NOT NULL,

    user_id INTEGER,

    FOREIGN KEY(user_id) REFERENCES user(id)
);
