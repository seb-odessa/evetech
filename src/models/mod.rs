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
    use api::RpType;
    use api::RqType;

    use diesel::prelude::*;
    use diesel::sqlite::SqliteConnection;
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    fn establish_connection<S: Into<String>>(uri: S) -> anyhow::Result<SqliteConnection> {
        let conn = SqliteConnection::establish(uri.into().as_str())?;
        Ok(conn)
    }

    pub fn run_migrations(conn: &mut SqliteConnection) {
        conn.run_pending_migrations(MIGRATIONS).unwrap();
    }

    fn create_killmail(id: u32) -> killmails::killmail::Killmail {
        killmails::killmail::Killmail {
            killmail_id: id,
            killmail_time: "".to_owned(),
            solar_system_id: 1,
            moon_id: None,
            war_id: None,
            attackers: Vec::new(),
            victim: killmails::victim::Victim {
                character_id: Some(1),
                corporation_id: Some(10),
                alliance_id: Some(100),
                faction_id: Some(1000),
                damage_taken: 0,
                position: None,
                ship_type_id: 42,
                items: None,
            },
        }
    }

    fn create_attacker(id: u32) -> killmails::attacker::Attacker {
        killmails::attacker::Attacker {
            character_id: Some(id),
            corporation_id: Some(10 * id),
            alliance_id: Some(100 * id),
            faction_id: Some(1000 * id),
            damage_done: 100,
            final_blow: true,
            security_status: 0.0,
            ship_type_id: None,
            weapon_type_id: None,
        }
    }

    fn generate_killmails(api: &mut Api, count: u32) -> anyhow::Result<()> {
        const OFFSET: u32 = 2;
        for i in 0..count {
            let mut killmail = create_killmail(i + OFFSET);
            for id in i..count {
                let attacker = create_attacker(id + OFFSET);
                killmail.victim.damage_taken += attacker.damage_done;
                killmail.attackers.push(attacker)
            }
            api.save(&killmail)?;
        }
        Ok(())
    }

    #[test]
    fn find_assistants() -> anyhow::Result<()> {
        let mut conn = establish_connection(":memory:")?;
        run_migrations(&mut conn);
        let mut api = Api::new(conn);
        generate_killmails(&mut api, 4)?;

        let assist = api.find_assistants(RqType::Character(2), RpType::Character)?;
        assert_eq!(assist, vec![(5, 1), (4, 1), (3, 1)]);

        let assist = api.find_assistants(RqType::Character(3), RpType::Character)?;
        assert_eq!(assist, vec![(5, 2), (4, 2), (2, 1)]);

        let assist = api.find_assistants(RqType::Character(4), RpType::Character)?;
        assert_eq!(assist, vec![(5, 3), (3, 2), (2, 1)]);

        let assist = api.find_assistants(RqType::Corporation(40), RpType::Corporation)?;
        assert_eq!(assist, vec![(50, 3), (30, 2), (20, 1)]);

        let assist = api.find_assistants(RqType::Alliance(400), RpType::Alliance)?;
        assert_eq!(assist, vec![(500, 3), (300, 2), (200, 1)]);

        let assist = api.find_assistants(RqType::Faction(4000), RpType::Faction)?;
        assert_eq!(assist, vec![(5000, 3), (3000, 2), (2000, 1)]);

        let assist = api.find_assistants(RqType::Corporation(20), RpType::Character)?;
        assert_eq!(assist, vec![(5, 1), (4, 1), (3, 1)]);

        Ok(())
    }

    #[test]
    fn find_attackers() -> anyhow::Result<()> {
        let mut conn = establish_connection(":memory:")?;
        run_migrations(&mut conn);
        let mut api = Api::new(conn);
        generate_killmails(&mut api, 4)?;

        let assist = api.find_attackers(RqType::Character(1), RpType::Character)?;
        assert_eq!(assist, vec![(5, 4), (4, 3), (3, 2), (2, 1)]);

        let assist = api.find_attackers(RqType::Corporation(10), RpType::Corporation)?;
        assert_eq!(assist, vec![(50, 4), (40, 3), (30, 2), (20, 1)]);

        let assist = api.find_attackers(RqType::Alliance(100), RpType::Alliance)?;
        assert_eq!(assist, vec![(500, 4), (400, 3), (300, 2), (200, 1)]);

        let assist = api.find_attackers(RqType::Faction(1000), RpType::Faction)?;
        assert_eq!(assist, vec![(5000, 4), (4000, 3), (3000, 2), (2000, 1)]);

        let assist = api.find_attackers(RqType::Character(1), RpType::Corporation)?;
        assert_eq!(assist, vec![(50, 4), (40, 3), (30, 2), (20, 1)]);

        let assist = api.find_attackers(RqType::Character(1), RpType::Alliance)?;
        assert_eq!(assist, vec![(500, 4), (400, 3), (300, 2), (200, 1)]);

        let assist = api.find_attackers(RqType::Corporation(10), RpType::Alliance)?;
        assert_eq!(assist, vec![(500, 4), (400, 3), (300, 2), (200, 1)]);

        Ok(())
    }

    #[test]
    fn load() -> anyhow::Result<()> {
        let mut conn = establish_connection(":memory:")?;
        run_migrations(&mut conn);

        let killmail = killmails::killmail::Killmail {
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
        api.save(&killmail)?;

        let selected = api.load(120461567)?;

        assert_eq!(killmail.killmail_id, selected.killmail_id);
        assert_eq!(killmail.killmail_time, selected.killmail_time);
        assert_eq!(killmail.solar_system_id, selected.solar_system_id);
        assert!(killmail
            .attackers
            .iter()
            .all(|item| selected.attackers.contains(item)));
        assert!(selected
            .attackers
            .iter()
            .all(|item| killmail.attackers.contains(item)));
        assert_eq!(killmail.victim, selected.victim);

        Ok(())
    }
}
