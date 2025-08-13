-- Your SQL goes here
CREATE TABLE Users(
    Id UUID PRIMARY KEY ,
    Name VARCHAR(255) NOT NULL,
    Gender VARCHAR(20) NOT NULL DEFAULT 'Prefer not to say' CHECK ( Gender IN ('Male', 'Female', 'Prefer not to say') ),
    DOB DATE,
    Username VARCHAR(255) NOT NULL UNIQUE,
    Email VARCHAR(255) NOT NULL UNIQUE,
    Password VARCHAR(255) NOT NULL,
    Bio VARCHAR(255) NOT NULL DEFAULT '',
    Image VARCHAR(255),
    Role VARCHAR(255) NOT NULL DEFAULT ('Member') CHECK ( Role IN ('Member', 'Admin', 'Artist')),
    isVerified BOOLEAN NOT NULL DEFAULT FALSE,
    OriginId UUID REFERENCES Origins(Id),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
