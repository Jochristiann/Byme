-- Your SQL goes here
CREATE TABLE Reports(
    Id UUID PRIMARY KEY ,
    Description VARCHAR(255) NOT NULL,
    Image VARCHAR(255) NOT NULL,
    ViolationTypeId UUID NOT NULL REFERENCES ViolationTypes(Id),
    UserId UUID NOT NULL REFERENCES Users(Id),
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);