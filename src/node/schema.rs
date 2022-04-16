table! {
    blockchains (head_block, tail_block) {
        head_block -> Nullable<Integer>,
        tail_block -> Nullable<Integer>,
    }
}

table! {
    blocks (id) {
        id -> Integer,
        hash -> Text,
        prev_hash -> Text,
        content -> Text,
        nonce -> Text,
    }
}

table! {
    nodes (id) {
        id -> Integer,
        url -> Text,
        timeout_ms -> Integer,
        start_block_id -> Nullable<Integer>,
    }
}

table! {
    peers (id) {
        id -> Integer,
        url -> Text,
    }
}

joinable!(nodes -> blocks (start_block_id));

allow_tables_to_appear_in_same_query!(
    blockchains,
    blocks,
    nodes,
    peers,
);
