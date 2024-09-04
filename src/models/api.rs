use crate::killmails;
use crate::models;
use crate::schema;

use diesel::dsl::count_star;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

pub enum RqType {
    Character(u32),
    Corporation(u32),
    Alliance(u32),
    Faction(u32),
}

pub enum RpType {
    Character,
    Corporation,
    Alliance,
    Faction,
}

pub struct Api {
    conn: SqliteConnection,
}

impl Api {
    pub fn new(conn: SqliteConnection) -> Self {
        Self { conn }
    }

    pub fn save(&mut self, killmail: &killmails::killmail::Killmail) -> anyhow::Result<()> {
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

    pub fn load(&mut self, id: u32) -> anyhow::Result<killmails::killmail::Killmail> {
        let killmail = schema::killmails::table
            .filter(schema::killmails::killmail_id.eq(id as i32))
            .first::<models::killmail::Killmail>(&mut self.conn)?;

        let attackers = schema::attackers::table
            .filter(schema::attackers::killmail_id.eq(id as i32))
            .load::<models::attacker::Attacker>(&mut self.conn)?
            .into_iter()
            .map(|attacker| attacker.into())
            .collect();

        let victim = schema::victims::table
            .filter(schema::victims::killmail_id.eq(id as i32))
            .first::<models::victim::Victim>(&mut self.conn)?
            .into();

        Ok(killmails::killmail::Killmail {
            killmail_id: id as u32,
            killmail_time: killmail.killmail_time,
            solar_system_id: killmail.solar_system_id as u32,
            moon_id: killmail.moon_id.map(|x| x.try_into().ok()).flatten(),
            war_id: killmail.war_id.map(|x| x.try_into().ok()).flatten(),
            attackers,
            victim,
        })
    }

    pub fn find_assistants(&mut self, rq: RqType, rp: RpType) -> QueryResult<Vec<(i32, i64)>> {
        use schema::attackers;
        use schema::attackers::dsl::*;

        let (attacker, assistant) = diesel::alias!(attackers as _1, attackers as _2);

        let attacker_filter: Box<dyn BoxableExpression<_, _, SqlType = diesel::sql_types::Bool>> =
            match rq {
                RqType::Character(id) => Box::new(attacker.field(character_id).eq(id as i32)),
                RqType::Corporation(id) => Box::new(attacker.field(corporation_id).eq(id as i32)),
                RqType::Alliance(id) => Box::new(attacker.field(alliance_id).eq(id as i32)),
                RqType::Faction(id) => Box::new(attacker.field(faction_id).eq(id as i32)),
            };
        let assist_filter: Box<dyn BoxableExpression<_, _, SqlType = diesel::sql_types::Bool>> =
            match rq {
                RqType::Character(id) => Box::new(assistant.field(character_id).ne(id as i32)),
                RqType::Corporation(id) => Box::new(assistant.field(corporation_id).ne(id as i32)),
                RqType::Alliance(id) => Box::new(assistant.field(alliance_id).ne(id as i32)),
                RqType::Faction(id) => Box::new(assistant.field(faction_id).ne(id as i32)),
            };
        let count = count_star();
        match rp {
            RpType::Character => attacker
                .inner_join(
                    assistant.on(attacker.field(killmail_id).eq(assistant.field(killmail_id))),
                )
                .filter(attacker_filter)
                .filter(assist_filter)
                .filter(assistant.field(character_id).ne(0))
                .group_by(assistant.field(character_id))
                .select((assistant.field(character_id), count))
                .order(count.desc())
                .load::<(i32, i64)>(&mut self.conn),
            RpType::Corporation => attacker
                .inner_join(
                    assistant.on(attacker.field(killmail_id).eq(assistant.field(killmail_id))),
                )
                .filter(attacker_filter)
                .filter(assist_filter)
                .filter(assistant.field(character_id).ne(0))
                .group_by(assistant.field(corporation_id))
                .select((assistant.field(corporation_id), count))
                .order(count.desc())
                .load::<(i32, i64)>(&mut self.conn),
            RpType::Alliance => attacker
                .inner_join(
                    assistant.on(attacker.field(killmail_id).eq(assistant.field(killmail_id))),
                )
                .filter(attacker_filter)
                .filter(assist_filter)
                .filter(assistant.field(character_id).ne(0))
                .group_by(assistant.field(alliance_id))
                .select((assistant.field(alliance_id), count))
                .order(count.desc())
                .load::<(i32, i64)>(&mut self.conn),
            RpType::Faction => attacker
                .inner_join(
                    assistant.on(attacker.field(killmail_id).eq(assistant.field(killmail_id))),
                )
                .filter(attacker_filter)
                .filter(assist_filter)
                .filter(assistant.field(character_id).ne(0))
                .group_by(assistant.field(faction_id))
                .select((assistant.field(faction_id), count))
                .order(count.desc())
                .load::<(i32, i64)>(&mut self.conn),
        }
    }

    pub fn find_attackers(&mut self, rq: RqType, rp: RpType) -> QueryResult<Vec<(i32, i64)>> {
        use schema::attackers;
        use schema::attackers::dsl::*;
        use schema::victims;
        use schema::victims::dsl::*;

        let victim: Box<dyn BoxableExpression<_, _, SqlType = diesel::sql_types::Bool>> = match rq {
            RqType::Character(id) => Box::new(victims::character_id.eq(id as i32)),
            RqType::Corporation(id) => Box::new(victims::corporation_id.eq(id as i32)),
            RqType::Alliance(id) => Box::new(victims::alliance_id.eq(id as i32)),
            RqType::Faction(id) => Box::new(victims::faction_id.eq(id as i32)),
        };

        let count = count_star();
        match rp {
            RpType::Character => attackers
                .inner_join(victims.on(attackers::killmail_id.eq(victims::killmail_id)))
                .filter(victim)
                .filter(attackers::character_id.ne(0))
                .group_by(attackers::character_id)
                .select((attackers::character_id, count))
                .order(count.desc())
                .load::<(i32, i64)>(&mut self.conn),
            RpType::Corporation => attackers
                .inner_join(victims.on(attackers::killmail_id.eq(victims::killmail_id)))
                .filter(victim)
                .filter(attackers::character_id.ne(0))
                .group_by(attackers::corporation_id)
                .select((attackers::corporation_id, count))
                .order(count.desc())
                .load::<(i32, i64)>(&mut self.conn),
            RpType::Alliance => attackers
                .inner_join(victims.on(attackers::killmail_id.eq(victims::killmail_id)))
                .filter(victim)
                .filter(attackers::character_id.ne(0))
                .group_by(attackers::alliance_id)
                .select((attackers::alliance_id, count))
                .order(count.desc())
                .load::<(i32, i64)>(&mut self.conn),
            RpType::Faction => attackers
                .inner_join(victims.on(attackers::killmail_id.eq(victims::killmail_id)))
                .filter(victim)
                .filter(attackers::character_id.ne(0))
                .group_by(attackers::faction_id)
                .select((attackers::faction_id, count))
                .order(count.desc())
                .load::<(i32, i64)>(&mut self.conn),
        }
    }
}
