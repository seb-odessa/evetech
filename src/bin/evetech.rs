use docopt::Docopt;
use serde::{Deserialize, Serialize};

use evetech::alliance;
use evetech::character;
use evetech::common;
use evetech::corporation;
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
    evetech status
    evetech names <ids>...
    evetech search [<search-cmd>] <ids>...
    evetech universe <universe-cmd> <ids>...
    evetech market <market-cmd> <ids>...
    evetech alliance <ids>...
    evetech corporation <ids>...
    evetech character <ids>...
    evetech (-h | --help)
    evetech --version

Commands:
    Search:    Agent Faction Alliance Corporation Character Region Constellation System Station InventoryType

    Universe:  Region Constellation System Station Stargate Star Planet Moon Belt Type Group Category

    Market:    Group

Options:
  -h --help     Show this screen.
  --version     Show version.
";

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
enum Search {
    Agent,
    Faction,
    Alliance,
    Corporation,
    Character,
    Region,
    Constellation,
    System,
    Station,
    InventoryType,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
enum Univesrse {
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

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
enum Market {
    Group,
}

#[derive(Debug, Deserialize, Serialize)]
struct Args {
    cmd_status: bool,
    cmd_names: bool,
    cmd_search: bool,
    cmd_universe: bool,
    cmd_market: bool,

    cmd_alliance: bool,
    cmd_corporation: bool,
    cmd_character: bool,

    arg_search_cmd: Option<Search>,
    arg_universe_cmd: Option<Univesrse>,
    arg_market_cmd: Option<Market>,

    arg_ids: Option<Vec<String>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    // display(&args);

    let api = EveApi::new();
    if args.cmd_status {
        status(&api).await?;
    } else if args.cmd_search {
        if let Some(names) = args.arg_ids {
            search(&api, &names, &args.arg_search_cmd).await?;
        }
    } else if args.cmd_names {
        if let Some(args) = args.arg_ids {
            names(&api, &args).await?;
        }
    } else if args.cmd_universe {
        if let Some(ids) = args.arg_ids {
            if let Some(cmd) = args.arg_universe_cmd {
                universe(&api, &ids, &cmd).await?;
            }
        }
    } else if args.cmd_market {
        if let Some(ids) = args.arg_ids {
            if let Some(cmd) = args.arg_market_cmd {
                market(&api, &ids, &cmd).await?;
            }
        }
    } else if args.cmd_alliance {
        if let Some(ids) = args.arg_ids {
            print::<alliance::Alliance>(&api, &ids).await?;
        }
    } else if args.cmd_corporation {
        if let Some(ids) = args.arg_ids {
            print::<corporation::Corporation>(&api, &ids).await?;
        }
    } else if args.cmd_character {
        if let Some(ids) = args.arg_ids {
            print::<character::Character>(&api, &ids).await?;
        }
    }

    Ok(())
}

async fn status(api: &EveApi) -> anyhow::Result<()> {
    let obj = api.load::<common::Status>(&Uid::Empty).await?;
    display(&obj);
    Ok(())
}

async fn search(api: &EveApi, names: &Vec<String>, cmd: &Option<Search>) -> anyhow::Result<()> {
    let obj = api.search(&names).await?;
    match cmd {
        None => display(&obj),
        Some(Search::Agent) => display(&obj.agents),
        Some(Search::Alliance) => display(&obj.alliances),
        Some(Search::Character) => display(&obj.characters),
        Some(Search::Constellation) => display(&obj.constellations),
        Some(Search::Corporation) => display(&obj.corporations),
        Some(Search::Faction) => display(&obj.factions),
        Some(Search::InventoryType) => display(&obj.inventory_types),
        Some(Search::Region) => display(&obj.regions),
        Some(Search::System) => display(&obj.systems),
        Some(Search::Station) => display(&obj.stations),
    }
    Ok(())
}

async fn names(api: &EveApi, args: &Vec<String>) -> anyhow::Result<()> {
    let ids: Result<Vec<i32>, _> = args.iter().map(|s| s.parse::<i32>()).collect();
    if let Ok(ids) = ids {
        let obj = api.names(&ids).await?;
        display(&obj);
    }

    Ok(())
}

async fn universe(api: &EveApi, args: &Vec<String>, cmd: &Univesrse) -> anyhow::Result<()> {
    match cmd {
        Univesrse::Region => print::<universe::Region>(api, &args).await?,
        Univesrse::Constellation => print::<universe::Constellation>(api, &args).await?,
        Univesrse::System => print::<universe::System>(api, &args).await?,
        Univesrse::Star => print::<universe::Star>(api, &args).await?,
        Univesrse::Planet => print::<universe::Planet>(api, &args).await?,
        Univesrse::Moon => print::<universe::Moon>(api, &args).await?,
        Univesrse::Stargate => print::<universe::Stargate>(api, &args).await?,
        Univesrse::Station => print::<universe::Station>(api, &args).await?,
        Univesrse::Belt => print::<universe::AsteroidBelt>(api, &args).await?,
        Univesrse::Type => print::<universe::Type>(api, &args).await?,
        Univesrse::Group => print::<universe::Group>(api, &args).await?,
        Univesrse::Category => print::<universe::Category>(api, &args).await?,
    }

    Ok(())
}

async fn market(api: &EveApi, args: &Vec<String>, cmd: &Market) -> anyhow::Result<()> {
    match cmd {
        Market::Group => print::<market::Group>(api, &args).await?,
    }
    Ok(())
}

fn display<T: 'static>(object: &T)
where
    T: for<'se> Serialize,
{
    match serde_json::to_string_pretty(&object) {
        Ok(json) => println!("{json}"),
        Err(err) => println!("{err}"),
    }
}

async fn print<T: 'static>(api: &EveApi, args: &Vec<String>) -> anyhow::Result<()>
where
    T: Uri + Debug + for<'de> Deserialize<'de> + for<'se> Serialize,
{
    for id in load_ids::<T>(api, args).await? {
        let obj = api.load::<T>(&Uid::Id(id)).await?;
        display(&obj);
    }
    Ok(())
}

async fn load_ids<T: 'static>(api: &EveApi, args: &Vec<String>) -> anyhow::Result<Vec<i32>>
where
    T: Uri + Debug + for<'de> Deserialize<'de> + for<'se> Serialize,
{
    let mut ids = Vec::new();
    let mut names = Vec::new();
    for arg in args.iter().cloned() {
        if let Ok(id) = arg.parse::<i32>() {
            ids.push(id)
        } else {
            names.push(arg);
        }
    }

    if !names.is_empty() {
        let search_result = api.search(&names).await?;

        let type_id = TypeId::of::<T>();
        // if type_id == TypeId::of::<::Agent>() {
        // } else if type_id == TypeId::of::<::Faction>() {

        if type_id == TypeId::of::<universe::Constellation>() {
            if let Some(constellations) = search_result.constellations {
                ids.extend(constellations.into_iter().map(|obj| obj.id));
            }
        } else if type_id == TypeId::of::<universe::Region>() {
            if let Some(regions) = search_result.regions {
                ids.extend(regions.into_iter().map(|obj| obj.id));
            }
        } else if type_id == TypeId::of::<universe::System>() {
            if let Some(systems) = search_result.systems {
                ids.extend(systems.into_iter().map(|obj| obj.id));
            }
        } else if type_id == TypeId::of::<universe::Station>() {
            if let Some(stations) = search_result.stations {
                ids.extend(stations.into_iter().map(|obj| obj.id));
            }
        } else if type_id == TypeId::of::<universe::Type>() {
            if let Some(inventory_types) = search_result.inventory_types {
                ids.extend(inventory_types.into_iter().map(|obj| obj.id));
            }
        } else if type_id == TypeId::of::<alliance::Alliance>() {
            if let Some(alliances) = search_result.alliances {
                ids.extend(alliances.into_iter().map(|obj| obj.id));
            }
        } else if type_id == TypeId::of::<corporation::Corporation>() {
            if let Some(corporations) = search_result.corporations {
                ids.extend(corporations.into_iter().map(|obj| obj.id));
            }
        } else if type_id == TypeId::of::<character::Character>() {
            if let Some(characters) = search_result.characters {
                ids.extend(characters.into_iter().map(|obj| obj.id));
            }
        }
    }

    Ok(ids)
}
