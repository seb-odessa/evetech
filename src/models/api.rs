use crate::killmails;
use crate::models;
use crate::schema;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

pub struct Api {
    conn: SqliteConnection,
}

impl Api {
    pub fn new(conn: SqliteConnection) -> Self {
        Self { conn }
    }

    pub fn insert(&mut self, killmail: &killmails::killmail::Killmail) -> anyhow::Result<()> {
        self.conn.transaction::<_, _, _>(|conn| {
            let id = killmail.killmail_id;

            diesel::insert_into(schema::killmails::table)
                .values(models::killmail::Killmail::from(killmail))
                .execute(conn)?;

            diesel::insert_into(schema::victims::table)
                .values(models::victim::Victim::from((id, &killmail.victim)))
                .execute(conn)?;

            for attacker in &killmail.attackers {
                diesel::insert_into(schema::attackers::table)
                    .values(models::attacker::Attacker::from((id, attacker)))
                    .execute(conn)?;
            }
            Ok(())
        })
    }

    pub fn select_victim(&mut self, id: u32) -> anyhow::Result<killmails::victim::Victim> {
        let victim = schema::victims::table
            .filter(schema::victims::killmail_id.eq(id as i32))
            .first::<models::victim::Victim>(&mut self.conn)?;

        Ok(victim.into())
    }

    pub fn select_attackers(
        &mut self,
        id: u32,
    ) -> anyhow::Result<Vec<killmails::attacker::Attacker>> {
        let attackers = schema::attackers::table
            .filter(schema::attackers::killmail_id.eq(id as i32))
            .load::<models::attacker::Attacker>(&mut self.conn)?;

        Ok(attackers
            .into_iter()
            .map(|attacker| attacker.into())
            .collect())
    }

    pub fn select(&mut self, id: u32) -> anyhow::Result<killmails::killmail::Killmail> {
        let killmail = schema::killmails::table
            .filter(schema::killmails::killmail_id.eq(id as i32))
            .first::<models::killmail::Killmail>(&mut self.conn)?;
        let attackers = self.select_attackers(id)?;
        let victim = self.select_victim(id)?;

        Ok(killmails::killmail::Killmail {
            killmail_id: killmail.killmail_id as u32,
            killmail_time: killmail.killmail_time,
            solar_system_id: killmail.solar_system_id as u32,
            moon_id: killmail.moon_id.map(|x| x.try_into().ok()).flatten(),
            war_id: killmail.war_id.map(|x| x.try_into().ok()).flatten(),
            attackers: attackers,
            victim: victim,
        })
    }
}
