CREATE TABLE rooms (
    room_id VARCHAR(20) PRIMARY KEY
);

CREATE TABLE users (
    user_id UUID PRIMARY KEY,
    name VARCHAR(20) NOT NULL,
    room_id VARCHAR(20) NOT NULL REFERENCES rooms(room_id)
);

CREATE TABLE messages (
    message_id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(user_id),
    content VARCHAR(250) NOT NULL,
    time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);