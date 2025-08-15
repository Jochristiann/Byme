-- Your SQL goes here
CREATE TABLE resettokens (
      Id UUID PRIMARY KEY,
      UserId UUID NOT NULL REFERENCES users(id),
      created_at TIMESTAMP NOT NULL DEFAULT NOW()
);