INSERT INTO users (user_id, name, room_id) VALUES (
    $1,
    $2,
    $3
) RETURNING *;