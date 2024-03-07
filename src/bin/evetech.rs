use docopt::Docopt;
use log::debug;
use serde::Deserialize;

use evetech::common;
use evetech::esi::EveSwaggerClient;
use evetech::esi::Loadable;
use evetech::esi::LoadableById;
use evetech::esi::Searchable;
use evetech::universe;

use std::fmt::Display;

const USAGE: &'static str = "
Evetech CLI Client

Usage:
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
  evetech (-h | --help)
  evetech --version

Options:
  -h --help     Show this screen.
  --version     Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    cmd_status: bool,
    cmd_search: bool,
    cmd_system: bool,
    cmd_star: bool,
    cmd_planet: bool,
    cmd_moon: bool,
    cmd_belt: bool,
    cmd_stargate: bool,
    cmd_station: bool,
    cmd_constellation: bool,
    cmd_region: bool,
    cmd_type: bool,
    cmd_group: bool,
    cmd_category: bool,
    arg_ids: Vec<u32>,
    arg_names: Vec<String>,
 }

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    debug!("{:?}", args);

    let esc: EveSwaggerClient = EveSwaggerClient::new();
    if args.cmd_status {
        status(&esc).await?;
    } else if args.cmd_search {
        search(&esc, args.arg_names.clone()).await?;
    } else if args.cmd_system {
        report::<universe::System>(&esc, args.arg_ids.clone()).await?;
    } else if args.cmd_star {
        report::<universe::Star>(&esc, args.arg_ids.clone()).await?;
    } else if args.cmd_planet {
        report::<universe::Planet>(&esc, args.arg_ids.clone()).await?;
    } else if args.cmd_moon {
        report::<universe::Moon>(&esc, args.arg_ids.clone()).await?;
    } else if args.cmd_belt {
        report::<universe::AsteroidBelt>(&esc, args.arg_ids.clone()).await?;
    } else if args.cmd_stargate {
        report::<universe::Stargate>(&esc, args.arg_ids.clone()).await?;
    } else if args.cmd_station {
        report::<universe::Station>(&esc, args.arg_ids.clone()).await?;
    } else if args.cmd_constellation {
        report::<universe::Constellation>(&esc, args.arg_ids.clone()).await?;
    } else if args.cmd_region {
        report::<universe::Region>(&esc, args.arg_ids.clone()).await?;
    } else if args.cmd_type {
        report::<universe::Type>(&esc, args.arg_ids.clone()).await?;
    } else if args.cmd_group {
        report::<universe::Group>(&esc, args.arg_ids.clone()).await?;
    } else if args.cmd_category {
        report::<universe::Category>(&esc, args.arg_ids.clone()).await?;
    }

    Ok(())
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

async fn report<T>(esc: &EveSwaggerClient, ids: Vec<u32>) -> anyhow::Result<()>
where
    EveSwaggerClient: LoadableById<T>,
    <EveSwaggerClient as LoadableById<T>>::Output: Display,
{
    for id in ids {
        let obj = <EveSwaggerClient as LoadableById<T>>::load(&esc, id).await?;
        println!("{}", obj);
    }
    Ok(())
}
