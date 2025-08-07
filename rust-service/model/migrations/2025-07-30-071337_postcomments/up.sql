-- Your SQL goes here
CREATE TABLE PostComments(
    PostId UUID REFERENCES Posts(Id) ON DELETE CASCADE,
    CommentId UUID REFERENCES Comments(Id) ON DELETE CASCADE,
    PRIMARY KEY (PostId, CommentId)
);