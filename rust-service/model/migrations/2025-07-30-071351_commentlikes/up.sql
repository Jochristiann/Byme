-- Your SQL goes here
CREATE TABLE CommentLikes(
     UserId UUID REFERENCES Users(Id) ON DELETE CASCADE ,
     CommentId UUID REFERENCES Comments(Id) ON DELETE CASCADE ,
     PRIMARY KEY (UserId,CommentId)
)