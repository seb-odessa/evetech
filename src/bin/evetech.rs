use docopt::Docopt;
use serde::Deserialize;

use evetech::common;
use evetech::esi::EveSwaggerClient as EveClient;
use evetech::esi::Loadable;
use evetech::esi::LoadableById;
use evetech::esi::Searchable;
use evetech::market;
use evetech::universe;

use std::any::TypeId;
use std::fmt::Display;

const USAGE: &'static str = "
Evetech CLI Client

Usage:
    evetech <command>
    evetech <command> <args>...
    evetech <command> [--market] <args>...
    evetech (-h | --help)
    evetech --version

Commands
    evetech status
    evetech search <names>...
    evetech system <ids>...
    evetech star <ids>...
    evetech planet <ids>...
    evetech moon <ids>...
    evetech belt <ids>...
    evetech stargate <ids>...
    evetech station <ids>...
    evetech constellation <ids>...
    evetech region <ids>...
    evetech type <ids>...
    evetech group <ids>...
    evetech category <ids>...

Options:
  --market      Specify request for Market.
  -h --help     Show this screen.
  --version     Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_command: Command,
    flag_market: bool,
    arg_args: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
enum Command {
    Status,
    Search,
    System,
    Star,
    Planet,
    Moon,
    Belt,
    Stargate,
    Station,
    Constellation,
    Region,
    Type,
    Group,
    Category,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    println!("{:?}\n", args);

    let esc: EveClient = EveClient::new();
    match args.arg_command {
        Command::Status => status(&esc).await?,
        Command::Search => search(&esc, args.arg_args.clone()).await?,
        // ids
        Command::System => display::<universe::System>(&esc, args.arg_args.clone()).await?,
        Command::Star => display::<universe::Star>(&esc, args.arg_args.clone()).await?,
        Command::Planet => display::<universe::Planet>(&esc, args.arg_args.clone()).await?,
        Command::Moon => display::<universe::Moon>(&esc, args.arg_args.clone()).await?,
        Command::Belt => display::<universe::AsteroidBelt>(&esc, args.arg_args.clone()).await?,
        Command::Stargate => display::<universe::Stargate>(&esc, args.arg_args.clone()).await?,
        Command::Station => display::<universe::Station>(&esc, args.arg_args.clone()).await?,
        Command::Constellation => {
            display::<universe::Constellation>(&esc, args.arg_args.clone()).await?
        }
        Command::Region => display::<universe::Region>(&esc, args.arg_args.clone()).await?,
        Command::Type => display::<universe::Type>(&esc, args.arg_args.clone()).await?,
        Command::Group => {
            if args.flag_market {
                display::<market::Group>(&esc, args.arg_args.clone()).await?
            } else {
                display::<universe::Group>(&esc, args.arg_args.clone()).await?
            }
        }
        Command::Category => display::<universe::Category>(&esc, args.arg_args.clone()).await?,
    }

    Ok(())
}

async fn status(esc: &EveClient) -> anyhow::Result<()> {
    let status = <EveClient as Loadable<common::Status>>::load(&esc).await?;
    println!("{}", status);
    Ok(())
}

async fn search(esc: &EveClient, names: Vec<String>) -> anyhow::Result<()> {
    let result = <EveClient as Searchable<common::SearchResult>>::load(&esc, names).await?;

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

async fn display<T: 'static>(esc: &EveClient, args: Vec<String>) -> anyhow::Result<()>
where
    EveClient: LoadableById<T>,
    <EveClient as LoadableById<T>>::Output: Display,
{
    let mut ids = Vec::new();
    let mut names = Vec::new();
    for arg in args {
        if let Ok(id) = arg.parse::<u32>() {
            ids.push(id)
        } else {
            names.push(arg);
        }
    }

    let type_id = TypeId::of::<T>();
    let _b = TypeId::of::<universe::System>();

    if !ids.is_empty() {
        report::<T>(esc, ids).await?;
    }
    if !names.is_empty() {
        let sr = <EveClient as Searchable<common::SearchResult>>::load(&esc, names).await?;

        if type_id == TypeId::of::<universe::Constellation>() {
            try_report(esc, &sr.constellations)
                .await
                .unwrap_or_default();
        } else if type_id == TypeId::of::<universe::System>() {
            try_report(esc, &sr.systems).await.unwrap_or_default();
        } else if type_id == TypeId::of::<universe::Type>() {
            try_report(esc, &sr.inventory_types)
                .await
                .unwrap_or_default();
        } else if type_id == TypeId::of::<universe::Region>() {
            try_report(esc, &sr.regions).await.unwrap_or_default();
        } else if type_id == TypeId::of::<universe::Station>() {
            try_report(esc, &sr.stations).await.unwrap_or_default();
        }
        // try_report(esc, &sr.agents).await.unwrap_or_default();
        // try_report(esc, &sr.alliances).await.unwrap_or_default();
        // try_report(esc, &sr.characters).await.unwrap_or_default();
        // try_report(esc, &sr.corporations).await.unwrap_or_default();
        // try_report(esc, &sr.factions).await.unwrap_or_default();
    }

    Ok(())
}

async fn try_report<T>(esc: &EveClient, args: &Option<Vec<common::Object>>) -> anyhow::Result<()>
where
    EveClient: LoadableById<T>,
    <EveClient as LoadableById<T>>::Output: Display,
{
    if let Some(objects) = args {
        let ids = objects.into_iter().map(|obj| obj.id).collect::<Vec<_>>();
        report::<T>(esc, ids).await?;
    }

    Ok(())
}

async fn report<T>(esc: &EveClient, ids: Vec<u32>) -> anyhow::Result<()>
where
    EveClient: LoadableById<T>,
    <EveClient as LoadableById<T>>::Output: Display,
{
    for id in ids {
        let obj = <EveClient as LoadableById<T>>::load(&esc, id).await?;
        println!("{}", obj);
    }
    Ok(())
}
