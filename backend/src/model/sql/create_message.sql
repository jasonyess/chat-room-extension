INSERT INTO messages (message_id, user_id, content) VALUES (
    $1,
    $2,
    $3
) RETURNING message_id, user_id, content, time;