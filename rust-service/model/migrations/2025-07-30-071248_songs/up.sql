-- Your SQL goes here
CREATE TABLE Songs(
    Id UUID PRIMARY KEY ,
    Name VARCHAR(255) NOT NULL,
    Audio VARCHAR(255) NOT NULL,
    Image VARCHAR (255),
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);