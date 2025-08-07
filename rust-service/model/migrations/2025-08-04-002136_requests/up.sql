-- Your SQL goes here
CREATE TABLE Requests(
    Id            UUID PRIMARY KEY,
    Description   VARCHAR(255) NOT NULL,
    UserId        UUID         NOT NULL REFERENCES Users (Id) ON DELETE CASCADE ,
    RequestTypeId UUID         NOT NULL REFERENCES RequestTypes (Id),
    created_at    TIMESTAMP    NOT NULL DEFAULT NOW()
);