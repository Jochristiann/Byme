-- Your SQL goes here
CREATE TABLE UserFollowings(
    UserId UUID REFERENCES Users(Id) ON DELETE CASCADE ,
    FollowedUserId UUID REFERENCES  Users(Id) ON DELETE CASCADE ,
    PRIMARY KEY (UserId,FollowedUserId)
);