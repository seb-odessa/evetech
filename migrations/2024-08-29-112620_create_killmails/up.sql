-- Your SQL goes here
CREATE TABLE killmails(
    killmail_id INTEGER NOT NULL PRIMARY KEY,
    killmail_time TEXT NOT NULL,
    solar_system_id INTEGER NOT NULL,
    moon_id INTEGER,
    war_id INTEGER
);
CREATE TABLE attackers(
    killmail_id INTEGER NOT NULL REFERENCES killmails(killmail_id) ON DELETE CASCADE ON UPDATE CASCADE,
    character_id INTEGER NOT NULL,
    corporation_id INTEGER NOT NULL,
    alliance_id INTEGER NOT NULL,
    faction_id INTEGER NOT NULL,
    damage_done INTEGER NOT NULL,
    final_blow BOOLEAN NOT NULL CHECK (final_blow IN (0, 1)),
    security_status REAL NOT NULL,
    ship_type_id INTEGER NOT NULL,
    weapon_type_id INTEGER NOT NULL,
    PRIMARY KEY (
        killmail_id,
        character_id,
        corporation_id,
        alliance_id,
        faction_id
    )
) WITHOUT ROWID;
CREATE TABLE victims(
    killmail_id INTEGER NOT NULL REFERENCES killmails(killmail_id) ON DELETE CASCADE ON UPDATE CASCADE,
    character_id INTEGER NOT NULL,
    corporation_id INTEGER NOT NULL,
    alliance_id INTEGER NOT NULL,
    faction_id INTEGER NOT NULL,
    damage_taken INTEGER NOT NULL,
    ship_type_id INTEGER NOT NULL,
    PRIMARY KEY (
        killmail_id,
        character_id,
        corporation_id,
        alliance_id,
        faction_id
    )
) WITHOUT ROWID;