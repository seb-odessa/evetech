use evetech::common;
use evetech::esi::Esi;
use std::env;

enum ObjectKinds {
    System,
    Star,
    Planet,
    Moon,
    AsteroidBelt,
    Stargates,
    Stations,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("warn"));

    let mut args = env::args();
    if let Some(cmd) = args.next() {
        if let Some(command) = args.next() {
            let esi = Esi::new();
            let tail = args.collect::<Vec<_>>();
            match command.as_str() {
                "status" => status(&esi).await,
                "search" => search(&esi, tail).await,
                "systems" => systems(&esi, tail).await,
                "stars" => objects(&esi, ObjectKinds::Star, tail).await,
                "planets" => objects(&esi, ObjectKinds::Planet, tail).await,
                "moons" => objects(&esi, ObjectKinds::Moon, tail).await,
                "belts" => objects(&esi, ObjectKinds::AsteroidBelt, tail).await,
                "asteroid_belts" => objects(&esi, ObjectKinds::AsteroidBelt, tail).await,
                "stargates" => objects(&esi, ObjectKinds::Stargates, tail).await,
                "stations" => objects(&esi, ObjectKinds::Stations, tail).await,
                _ => wrong_command(cmd, command),
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
    println!("\t{cmd} systems <SystemName>... | <Id>...");
    println!("\t{cmd} stars <Id>...");
    println!("\t{cmd} planets <Id>...");
    println!("\t{cmd} moons <Id>...");
    println!("\t{cmd} stargates <Id>...");
    println!("\t{cmd} stations <Id>...");
    println!("\t{cmd} belts <Id>...");
    println!("\t{cmd} asteroid_belts <Id>...");
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

    let print = |maybe_objects: Option<Vec<common::Object>>, title| {
        if let Some(objects) = maybe_objects {
            println!("{title}:");
            for obj in objects {
                println!("{}", obj);
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

async fn systems(esi: &Esi, names: Vec<String>) -> anyhow::Result<()> {
    let seach_result = esi.search(&names).await?;
    if let Some(systems) = seach_result.systems {
        for object in systems {
            let system = esi.system(object.id).await?;
            println!("{}", system);
        }
    } else {
        for maybe_id in names {
            if let Ok(id) = maybe_id.parse::<u32>() {
                print(esi, &ObjectKinds::System, id).await?;
            } else {
                println!("{} is not an `Id`", maybe_id);
            }
        }
    }
    Ok(())
}

async fn print(esi: &Esi, kind: &ObjectKinds, id: u32) -> anyhow::Result<()> {
    match kind {
        ObjectKinds::System => println!("{}", esi.system(id).await?),
        ObjectKinds::Star => println!("{}", esi.star(id).await?),
        ObjectKinds::Planet => println!("{}", esi.planet(id).await?),
        ObjectKinds::Moon => println!("{}", esi.moon(id).await?),
        ObjectKinds::AsteroidBelt => println!("{}", esi.asteroid_belt(id).await?),
        ObjectKinds::Stargates => println!("{}", esi.stargate(id).await?),
        ObjectKinds::Stations => println!("{}", esi.station(id).await?),
    }
    Ok(())
}

async fn objects(esi: &Esi, kind: ObjectKinds, ids: Vec<String>) -> anyhow::Result<()> {
    for maybe_id in ids {
        if let Ok(id) = maybe_id.parse::<u32>() {
            print(esi, &kind, id).await?;
        } else {
            println!("{} is not an `Id`", maybe_id);
        }
    }
    Ok(())
}
