-- Your SQL goes here
CREATE TABLE PostFiles (
    PostId UUID REFERENCES Posts(Id),
    Files VARCHAR(255),
    PRIMARY KEY (PostId,Files)
)