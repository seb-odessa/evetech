pub mod api;
mod attacker;
mod killmail;
mod victim;

pub use api::Api;

fn as_option(x: i32) -> Option<u32> {
    if 0 == x {
        None
    } else {
        Some(x as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::killmails;

    use diesel::prelude::*;
    use diesel::sqlite::SqliteConnection;
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    fn establish_connection<S: Into<String>>(uri: S) -> anyhow::Result<SqliteConnection> {
        let conn = SqliteConnection::establish(uri.into().as_str())?;
        Ok(conn)
    }
    pub fn run_migrations(conn: &mut SqliteConnection) {
        conn.run_pending_migrations(MIGRATIONS).unwrap();
    }

    #[test]
    fn test_create_and_query_killmail() -> anyhow::Result<()> {
        let mut conn = establish_connection(":memory:")?;
        run_migrations(&mut conn);

        let inserted = killmails::killmail::Killmail {
            killmail_id: 120461567,
            killmail_time: "2024-08-27T03:54:10Z".to_owned(),
            solar_system_id: 30004563,
            moon_id: None,
            war_id: None,
            attackers: vec![
                killmails::attacker::Attacker {
                    character_id: Some(3019582),
                    corporation_id: Some(1000274),
                    alliance_id: None,
                    faction_id: Some(500024),
                    damage_done: 6804,
                    final_blow: true,
                    security_status: 0.0,
                    ship_type_id: Some(34495),
                    weapon_type_id: Some(34580),
                },
                killmails::attacker::Attacker {
                    character_id: Some(3019581),
                    corporation_id: Some(1000274),
                    alliance_id: None,
                    faction_id: Some(500024),
                    damage_done: 4538,
                    final_blow: false,
                    security_status: 0.0,
                    ship_type_id: Some(34495),
                    weapon_type_id: Some(34580),
                },
                killmails::attacker::Attacker {
                    character_id: None,
                    corporation_id: None,
                    alliance_id: None,
                    faction_id: Some(500024),
                    damage_done: 0,
                    final_blow: false,
                    security_status: 0.0,
                    ship_type_id: Some(34495),
                    weapon_type_id: None,
                },
            ],

            victim: killmails::victim::Victim {
                character_id: Some(2120326223),
                corporation_id: Some(98316235),
                alliance_id: Some(1900696668),
                faction_id: None,
                damage_taken: 11342,
                position: None, // Unsupported in DB
                ship_type_id: 81008,
                items: None, // Unsupported in DB
            },
        };

        let mut api = Api::new(conn);
        api.insert(&inserted)?;

        let selected = api.select(120461567)?;

        assert_eq!(inserted.killmail_id, selected.killmail_id);
        assert_eq!(inserted.killmail_time, selected.killmail_time);
        assert_eq!(inserted.solar_system_id, selected.solar_system_id);
        assert!(inserted.attackers.iter().all(|item| selected.attackers.contains(item)));
        assert!(selected.attackers.iter().all(|item| inserted.attackers.contains(item)));
        assert_eq!(inserted.victim, selected.victim);

        Ok(())
    }
}


