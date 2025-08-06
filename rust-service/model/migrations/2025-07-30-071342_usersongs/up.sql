-- Your SQL goes here
CREATE TABLE UserSongs(
    UserId UUID REFERENCES Users(Id) ON DELETE CASCADE ,
    SongId UUID REFERENCES Songs(Id) ON DELETE CASCADE ,
    PRIMARY KEY (UserId,SongId)
)
