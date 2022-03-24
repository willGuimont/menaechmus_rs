CREATE TABLE blocks
(
    hash      TEXT PRIMARY KEY NOT NULL,
    prev_hash TEXT             NOT NULL,
    content   TEXT             NOT NULL,
    nonce     TEXT             NOT NULL
);
