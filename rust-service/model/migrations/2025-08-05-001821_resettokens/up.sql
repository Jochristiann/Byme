-- Your SQL goes here
CREATE TABLE resettokens (
      Id UUID PRIMARY KEY,
      UserId UUID NOT NULL REFERENCES users(id),
      NewPassword VARCHAR(255) NOT NULL,
      created_at TIMESTAMP NOT NULL DEFAULT NOW()
);