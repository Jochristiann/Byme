-- Your SQL goes here
CREATE TABLE Comments(
    Id UUID PRIMARY KEY ,
    Message VARCHAR(255) NOT NULL,
    UserId UUID NOT NULL REFERENCES Users(Id),
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);