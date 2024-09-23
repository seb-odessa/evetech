use docopt::Docopt;
use serde::{Deserialize, Serialize};

use evetech::common;
use evetech::esi::api::Uid;
use evetech::esi::api::Uri;
use evetech::esi::EveApi;

use evetech::market;
use evetech::universe;

use std::any::TypeId;
use std::fmt::Debug;

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

    let api = EveApi::new();
    match args.arg_command {
        Command::Status => print::<common::Status>(&api, args.arg_args.clone()).await?,
        Command::Search => search(&api, args.arg_args.clone()).await?,
        // ids
        Command::System => print::<universe::System>(&api, args.arg_args.clone()).await?,
        Command::Star => print::<universe::Star>(&api, args.arg_args.clone()).await?,
        Command::Planet => print::<universe::Planet>(&api, args.arg_args.clone()).await?,
        Command::Moon => print::<universe::Moon>(&api, args.arg_args.clone()).await?,
        Command::Belt => print::<universe::AsteroidBelt>(&api, args.arg_args.clone()).await?,
        Command::Stargate => print::<universe::Stargate>(&api, args.arg_args.clone()).await?,
        Command::Station => print::<universe::Station>(&api, args.arg_args.clone()).await?,
        Command::Constellation => {
            print::<universe::Constellation>(&api, args.arg_args.clone()).await?
        }
        Command::Region => print::<universe::Region>(&api, args.arg_args.clone()).await?,
        Command::Type => print::<universe::Type>(&api, args.arg_args.clone()).await?,
        Command::Group => {
            if args.flag_market {
                print::<market::Group>(&api, args.arg_args.clone()).await?
            } else {
                print::<universe::Group>(&api, args.arg_args.clone()).await?
            }
        }
        Command::Category => print::<universe::Category>(&api, args.arg_args.clone()).await?,
    }

    Ok(())
}

async fn search(api: &EveApi, names: Vec<String>) -> anyhow::Result<()> {
    let search_result = api.search(&names).await?;
    display(&search_result);
    Ok(())
}

// async fn display<T: 'static>(esc: &EveClient, args: Vec<String>) -> anyhow::Result<()>
// where
//     EveClient: LoadableById<T>,
//     <EveClient as LoadableById<T>>::Output: Display,
// {
//     let mut ids = Vec::new();
//     let mut names = Vec::new();
//     for arg in args {
//         if let Ok(id) = arg.parse::<i32>() {
//             ids.push(id)
//         } else {
//             names.push(arg);
//         }
//     }

//     let type_id = TypeId::of::<T>();
//     let _b = TypeId::of::<universe::System>();

//     if !ids.is_empty() {
//         report::<T>(esc, ids).await?;
//     }
//     if !names.is_empty() {
//         let sr = <EveClient as Searchable<common::SearchResult>>::load(&esc, names).await?;

//         if type_id == TypeId::of::<universe::Constellation>() {
//             try_report(esc, &sr.constellations)
//                 .await
//                 .unwrap_or_default();
//         } else if type_id == TypeId::of::<universe::System>() {
//             try_report(esc, &sr.systems).await.unwrap_or_default();
//         } else if type_id == TypeId::of::<universe::Type>() {
//             try_report(esc, &sr.inventory_types)
//                 .await
//                 .unwrap_or_default();
//         } else if type_id == TypeId::of::<universe::Region>() {
//             try_report(esc, &sr.regions).await.unwrap_or_default();
//         } else if type_id == TypeId::of::<universe::Station>() {
//             try_report(esc, &sr.stations).await.unwrap_or_default();
//         }
//         // try_report(esc, &sr.agents).await.unwrap_or_default();
//         // try_report(esc, &sr.alliances).await.unwrap_or_default();
//         // try_report(esc, &sr.characters).await.unwrap_or_default();
//         // try_report(esc, &sr.corporations).await.unwrap_or_default();
//         // try_report(esc, &sr.factions).await.unwrap_or_default();
//     }

//     Ok(())
// }

// async fn try_report<T>(esc: &EveClient, args: &Option<Vec<common::Object>>) -> anyhow::Result<()>
// where
//     EveClient: LoadableById<T>,
//     <EveClient as LoadableById<T>>::Output: Display,
// {
//     if let Some(objects) = args {
//         let ids = objects.into_iter().map(|obj| obj.id).collect::<Vec<_>>();
//         report::<T>(esc, ids).await?;
//     }

//     Ok(())
// }

// async fn report<T>(esc: &EveClient, ids: Vec<i32>) -> anyhow::Result<()>
// where
//     EveClient: LoadableById<T>,
//     <EveClient as LoadableById<T>>::Output: Display,
// {
//     for id in ids {
//         let obj = <EveClient as LoadableById<T>>::load(&esc, id).await?;
//         println!("{}", obj);
//     }
//     Ok(())
// }

fn display<T: 'static>(object: &T)
where
    T: for<'se> Serialize,
{
    match serde_json::to_string_pretty(&object) {
        Ok(json) => println!("{json}"),
        Err(err) => println!("{err}"),
    }
}


async fn print<T: 'static>(api: &EveApi, args: Vec<String>) -> anyhow::Result<()>
where
    T: Uri + Debug + for<'de> Deserialize<'de> + for<'se> Serialize,
{
    let type_id = TypeId::of::<T>();

    if type_id == TypeId::of::<common::Status>() {
        let obj = api.load::<T>(&Uid::Empty).await?;
        display(&obj);
    } else {
        let mut ids = Vec::new();
        for arg in args {
            if let Ok(id) = arg.parse::<i32>() {
                ids.push(id)
            }
        }

        for id in ids {
            let obj = api.load::<T>(&Uid::Id(id)).await?;
            display(&obj);
        }
    }

    Ok(())
}
