table! {
    users (discord_id) {
        discord_id -> BigInt,
        contributions -> Integer,
        expires -> Timestamp,
    }
}

table! {
    guilds (discord_id) {
        discord_id -> BigInt,
        guild_id -> BigInt,
    }
}

table! {
    connections (discord_id) {
        discord_id -> BigInt,
        github -> Text,
        expires -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(users, connections);
joinable!(connections -> users (discord_id));
