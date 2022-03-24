table! {
    blocks (hash) {
        hash -> Text,
        prev_hash -> Text,
        content -> Text,
        nonce -> Text,
    }
}

table! {
    node_blocks (id) {
        id -> Integer,
        block_id -> Integer,
        hash -> Text,
    }
}

table! {
    node_peers (id) {
        id -> Integer,
        block_id -> Integer,
        peer_id -> Integer,
    }
}

table! {
    nodes (id) {
        id -> Integer,
        next_content -> Text,
    }
}

table! {
    peers (id) {
        id -> Integer,
        url -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    blocks,
    node_blocks,
    node_peers,
    nodes,
    peers,
);
