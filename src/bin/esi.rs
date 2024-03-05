use evetech::common;
use evetech::esi::EveSwaggerClient;
use evetech::esi::Loadable;
use evetech::esi::LoadableById;
use evetech::esi::Searchable;
use evetech::universe;
use std::env;
use std::fmt::Display;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("warn"));

    let mut args = env::args();
    if let Some(cmd) = args.next() {
        if let Some(command) = args.next() {
            let esc: EveSwaggerClient = EveSwaggerClient::new();
            let tail = args.collect::<Vec<_>>();
            match command.as_str() {
                "status" => status(&esc).await,
                "search" => search(&esc, tail).await,
                "systems" => report::<universe::System>(&esc, tail).await,
                "stars" => report::<universe::Star>(&esc, tail).await,
                "planets" => report::<universe::Planet>(&esc, tail).await,
                "moons" => report::<universe::Moon>(&esc, tail).await,
                "belts" => report::<universe::AsteroidBelts>(&esc, tail).await,
                "asteroid_belts" => report::<universe::AsteroidBelts>(&esc, tail).await,
                "stargates" => report::<universe::Stargate>(&esc, tail).await,
                "stations" => report::<universe::Station>(&esc, tail).await,
                "constellations" => report::<universe::Constellation>(&esc, tail).await,
                "regions" => report::<universe::Region>(&esc, tail).await,
                "types" => report::<universe::Type>(&esc, tail).await,
                "groups" => report::<universe::Group>(&esc, tail).await,
                "categories" => report::<universe::Category>(&esc, tail).await,
                _ => wrong_command(cmd, command),
            }
        } else {
            usage(cmd)
        }
    } else {
        unreachable!("Unreachable");
    }
}

async fn status(esc: &EveSwaggerClient) -> anyhow::Result<()> {
    let status = <EveSwaggerClient as Loadable<common::Status>>::load(&esc).await?;
    println!("{}", status);
    Ok(())
}

async fn search(esc: &EveSwaggerClient, names: Vec<String>) -> anyhow::Result<()> {
    let result = <EveSwaggerClient as Searchable<common::SearchResult>>::load(&esc, names).await?;

    let print = |maybe_objects: Option<Vec<common::Object>>, title| {
        if let Some(objects) = maybe_objects {
            println!("{title}:");
            for obj in objects {
                println!("{}", obj);
            }
        }
    };

    print(result.agents, "Agents");
    print(result.alliances, "Alliances");
    print(result.characters, "Characters");
    print(result.constellations, "Constellations");
    print(result.corporations, "Corporations");
    print(result.factions, "Factions");
    print(result.inventory_types, "Items");
    print(result.regions, "Regions");
    print(result.stations, "Stations");
    print(result.systems, "Systems");

    Ok(())
}

async fn report<T>(esc: &EveSwaggerClient, ids: Vec<String>) -> anyhow::Result<()>
where
    EveSwaggerClient: LoadableById<T>,
    <EveSwaggerClient as LoadableById<T>>::Output: Display,
{
    for maybe_id in ids {
        if let Ok(id) = maybe_id.parse::<u32>() {
            let obj = <EveSwaggerClient as LoadableById<T>>::load(&esc, id).await?;
            println!("{}", obj);
        } else {
            println!("'{}' is not an `Id`", maybe_id);
        }
    }
    Ok(())
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
    println!("\t{cmd} regions <Id>...");
    println!("\t{cmd} types <Id>...");
    println!("\t{cmd} asteroid_belts <Id>...");
    Ok(())
}

fn wrong_command(cmd: String, command: String) -> anyhow::Result<()> {
    println!("Unknown command: {}", command);
    usage(cmd)
}
