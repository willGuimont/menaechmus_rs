CREATE TABLE peers
(
    id  INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    url TEXT    NOT NULL
);

CREATE TABLE blocks
(
    id        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    hash      TEXT    NOT NULL,
    prev_hash TEXT    NOT NULL,
    content   TEXT    NOT NULL,
    nonce     TEXT    NOT NULL
);

CREATE TABLE blockchains
(
    head_block INTEGER NOT NULL,
    tail_block INTEGER NOT NULL,
    PRIMARY KEY (head_block, tail_block),
    FOREIGN KEY (head_block) REFERENCES blocks (id),
    FOREIGN KEY (tail_block) REFERENCES blocks (id)
);

CREATE TABLE nodes
(
    id                    INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    url                   TEXT    NOT NULL,
    start_block_id        INTEGER NOT NULL,
    hash_starting_pattern TEXT    NOT NULL,
    FOREIGN KEY (start_block_id) REFERENCES blocks (id)
);
