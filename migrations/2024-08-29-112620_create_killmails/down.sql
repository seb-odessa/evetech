-- This file should undo anything in `up.sql`
DROP TABLE attakers;
DROP TABLE victims;
DROP TABLE killmails;
DROP INDEX attacker_character;
DROP INDEX attacker_corporation;
DROP INDEX attacker_alliance;
DROP INDEX attacker_faction;
DROP INDEX victim_character;
DROP INDEX victim_corporation;
DROP INDEX victim_alliance;
DROP INDEX victim_faction;
DROP INDEX killmail_time;
