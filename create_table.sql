CREATE TYPE user_power AS ENUM ('admin', 'user');

CREATE TABLE users (

    id SERIAL PRIMARY KEY,

    password VARCHAR(255) NOT NULL,

    profile_id INTEGER,

    FOREIGN KEY(profile_id) REFERENCES profiles(id)

);

CREATE TABLE programes (

    id SERIAL PRIMARY KEY,
    
    description VARCHAR(255) NOT NULL,
    
    link TEXT NOT NULL

);


CREATE TABLE profiles(

    id SERIAL PRIMARY KEY,

    power user_power NOT NULL,

    email VARCHAR(255) UNIQUE NOT NULL,

    name VARCHAR(255) NOT NULL,

    programe_id INTEGER,

    FOREIGN KEY(programe_id) REFERENCES programes(id)
);
