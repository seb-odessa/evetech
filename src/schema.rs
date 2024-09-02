// @generated automatically by Diesel CLI.

diesel::table! {
    attackers (killmail_id, character_id, corporation_id, alliance_id, faction_id) {
        killmail_id -> Integer,
        character_id -> Integer,
        corporation_id -> Integer,
        alliance_id -> Integer,
        faction_id -> Integer,
        damage_done -> Integer,
        final_blow -> Bool,
        security_status -> Float,
        ship_type_id -> Integer,
        weapon_type_id -> Integer,
    }
}

diesel::table! {
    killmails (killmail_id) {
        killmail_id -> Integer,
        killmail_time -> Text,
        solar_system_id -> Integer,
        moon_id -> Nullable<Integer>,
        war_id -> Nullable<Integer>,
    }
}

diesel::table! {
    victims (killmail_id, character_id, corporation_id, alliance_id, faction_id) {
        killmail_id -> Integer,
        character_id -> Integer,
        corporation_id -> Integer,
        alliance_id -> Integer,
        faction_id -> Integer,
        damage_taken -> Integer,
        ship_type_id -> Integer,
    }
}

diesel::joinable!(attackers -> killmails (killmail_id));
diesel::joinable!(victims -> killmails (killmail_id));

diesel::allow_tables_to_appear_in_same_query!(
    attackers,
    killmails,
    victims,
);
