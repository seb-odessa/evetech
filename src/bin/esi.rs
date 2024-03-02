use evetech::{esi::Esi, universe};
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let mut args = env::args();
    if let Some(cmd) = args.next() {
        if let Some(command) = args.next() {
            let esi = Esi::new();
            match command.as_str() {
                "status" => status(&esi).await,
                "search" => search(&esi, args.collect::<Vec<_>>()).await,
                "asteroid_belts" => asteroid_belts(&esi, args.collect::<Vec<_>>()).await,
                _ => wrong_command(cmd, command)
            }
        } else {
            usage(cmd)
        }
    } else {
        unreachable!("Unreachable");
    }
}

fn usage(cmd: String) -> anyhow::Result<()> {
    println!("Usage\n\t{} <Command> [argument]", cmd);
    println!("\t{cmd} status");
    println!("\t{cmd} search <AnyName>...");
    println!("\t{cmd} asteroid_belts <id|Asteroid Name> ...");
    Ok(())
}

fn wrong_command(cmd: String, command: String) -> anyhow::Result<()> {
    println!("Unknown command: {}", command);
    usage(cmd)
}

async fn status(esi: &Esi) -> anyhow::Result<()> {
    let status = esi.status().await?;
    println!("Players Online: {}", status.players);
    println!("Server Version: {}", status.server_version);
    println!("Start Time: {}", status.start_time);
    if let Some(vip) = status.vip {
        println!("Mode VIP only: {}", vip);
    }
    Ok(())
}

async fn search(esi: &Esi, names: Vec<String>) -> anyhow::Result<()> {
    let seach_result = esi.search(&names).await?;

    let print = |maybe_objects: Option<Vec<universe::Object>>, title| {
        if let Some(objects) = maybe_objects {
            println!("{title}:");
            for obj in objects {
                    println!("{} - {}", obj.id, obj.name);
            }
        }
    };

    print(seach_result.agents, "Agents");
    print(seach_result.alliances, "Alliances");
    print(seach_result.characters, "Characters");
    print(seach_result.constellations, "Constellations");
    print(seach_result.corporations, "Corporations");
    print(seach_result.factions, "Factions");
    print(seach_result.inventory_types, "Items");
    print(seach_result.regions, "Regions");
    print(seach_result.stations, "Stations");
    print(seach_result.systems, "Systems");

    Ok(())
}

async fn asteroid_belts(esi: &Esi, args: Vec<String>) -> anyhow::Result<()> {

    for arg in args {
        if let Ok(id) = arg.parse::<u32>() {
            let belt = esi.asteroid_belts(id).await?;
            println!("{:?}", belt);
        } else {

        }
    }
    Ok(())
}
