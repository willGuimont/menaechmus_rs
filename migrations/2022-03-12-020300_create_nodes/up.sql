CREATE TABLE nodes
(
    id           INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    next_content TEXT                              NOT NULL
);

CREATE TABLE peers
(
    id  INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    url TEXT                              NOT NULL
);

CREATE TABLE node_blocks
(
    id       INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    block_id INTEGER                           NOT NULL,
    hash     TEXT                              NOT NULL
);

CREATE TABLE node_peers
(
    id       INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    block_id INTEGER                           NOT NULL,
    peer_id  INTEGER                           NOT NULL
);
