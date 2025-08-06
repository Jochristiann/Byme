-- Your SQL goes here
CREATE TABLE PostLikes (
    UserId UUID REFERENCES Users(Id) ON DELETE CASCADE ,
    PostId UUID REFERENCES Posts(Id) ON DELETE CASCADE ,
    PRIMARY KEY (UserId, PostId)
)