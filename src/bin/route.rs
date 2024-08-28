use docopt::Docopt;
use serde::Deserialize;

use evetech::apps::Route;
use evetech::apps::WayPoint;
use evetech::common;
use evetech::common::Position;
use evetech::esi::EveSwaggerClient as Client;
use evetech::esi::LoadableById as ById;
use evetech::esi::Searchable;
use evetech::universe;

const USAGE: &'static str = "
Eve Route Builder

Usage:
  route <system> [--mode=<mode>]
  route (-h | --help)
  route --version


Options:
  -h --help       Show this screen.
  --version       Show version.
  --mode=<mode>   Set route build algorithm  [default: None].
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_system: String,
    flag_mode: Mode,
}

#[derive(Debug, Deserialize)]
enum Mode {
    None,
    Bruteforce,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    // println!("{:?}\n", args);

    build_route(&args.arg_system, &args.flag_mode).await?;

    Ok(())
}

async fn build_route(system: &String, mode: &Mode) -> anyhow::Result<()> {
    let esc = Client::new();
    let names = vec![system];
    let sr = <Client as Searchable<common::SearchResult>>::load(&esc, names).await?;
    if let Some(systems) = sr.systems {
        for obj in systems {
            let mut starts = Vec::new();
            println!("Solar System: '{}'", obj.name);
            let system = <Client as ById<universe::System>>::load(&esc, obj.id).await?;
            println!("{}", system);

            if let Some(id) = system.star_id {
                let star = <Client as ById<universe::Star>>::load(&esc, id).await?;
                println!("{:3} - {}", 1 + starts.len(), star.name);
                starts.push(WayPoint::new(id, &star.name, &Position::zero()));
            }

            if let Some(stations) = system.stations {
                for id in stations {
                    let station = <Client as ById<universe::Station>>::load(&esc, id).await?;
                    println!("{:3} - {}", 1 + starts.len(), station.name);
                    starts.push(WayPoint::new(id, &station.name, &station.position));
                }
            }

            if let Some(stargates) = system.stargates {
                for id in stargates {
                    let stargate = <Client as ById<universe::Stargate>>::load(&esc, id).await?;
                    println!("{:3} - {}", 1 + starts.len(), stargate.name);
                    starts.push(WayPoint::new(id, &stargate.name, &stargate.position));
                }
            }

            println!();
            println!("Please select the departure object (Ctrl+C for break):");
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let selected = input
                .trim()
                .parse::<i32>()
                .map_err(|e| println!("{:?}", e))
                .expect("Can't parse input value");

            if selected as u32 <= starts.len() as u32 {
                let id = system.system_id;
                let index = (selected - 1) as usize;
                let start = &starts[index];
                best_route(&esc, id, start, &mode).await?;
            }
        }
    }
    Ok(())
}

async fn best_route(esc: &Client, id: u32, start: &WayPoint, _: &Mode) -> anyhow::Result<()> {
    let system = <Client as ById<universe::System>>::load(&esc, id).await?;
    if let Some(celestials) = system.planets {
        for celestial in celestials {
            if let Some(ref belts) = celestial.asteroid_belts {
                let mut route = Route::new(start.clone());
                for id in belts {
                    let belt = <Client as ById<universe::AsteroidBelt>>::load(&esc, *id).await?;
                    route.add(WayPoint::new(*id, &belt.name, &belt.position));
                }
                let (len, ids) = route.brute_force();
                for id in ids {
                    if let Some(wp) = route.belts.get(&id) {
                        println!("{}", wp);
                    } else {
                        println!("{}", route.start);
                    }
                }
                println!("Total route length {:.0} Mega Meters", len / 1_000_000.0);
                println!();
            }
        }
    }

    Ok(())
}
