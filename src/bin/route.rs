use std::collections::HashMap;

use docopt::Docopt;
use serde::Deserialize;

use evetech::apps::Route;
use evetech::apps::WayPoint;
use evetech::common::Position;
use evetech::esi::EveApi;
use evetech::esi::Uid;
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

    build_route(&args.arg_system, &args.flag_mode).await?;

    Ok(())
}

async fn build_route(system: &String, mode: &Mode) -> anyhow::Result<()> {
    let api = EveApi::new();
    let names = vec![system.clone()];
    let sr = api.search(&names).await?;
    if let Some(systems) = sr.systems {
        for obj in systems {
            let mut starts = Vec::new();
            println!("Solar System: '{}'", obj.name);
            let system = api.load::<universe::System>(&Uid::Id(obj.id)).await?;
            println!("{}", system);

            if let Some(id) = system.star_id {
                let star = api.load::<universe::Star>(&Uid::Id(id)).await?;
                println!("{:3} - {}", 1 + starts.len(), star.name);
                starts.push(WayPoint::new(id, &star.name, &Position::zero()));
            }

            if let Some(stations) = system.stations {
                for id in stations {
                    let station = api.load::<universe::Station>(&Uid::Id(id)).await?;
                    println!("{:3} - {}", 1 + starts.len(), station.name);
                    starts.push(WayPoint::new(id, &station.name, &station.position));
                }
            }

            if let Some(stargates) = system.stargates {
                for id in stargates {
                    let stargate = api.load::<universe::Stargate>(&Uid::Id(id)).await?;
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

            if selected <= starts.len() as i32 {
                let id = system.system_id;
                let index = (selected - 1) as usize;
                let start = &starts[index];
                best_route(&api, id, start, &mode).await?;
            }
        }
    }
    Ok(())
}

async fn best_route(api: &EveApi, id: i32, start: &WayPoint, _: &Mode) -> anyhow::Result<()> {
    let system = api.load::<universe::System>(&Uid::Id(id)).await?;
    if let Some(planets) = system.planets {
        let mut route = Route::new(start.clone());
        let mut routes = HashMap::new();
        for planet in &planets {
            if let Some(belts) = &planet.asteroid_belts {
                let planet = api.load::<universe::Planet>(&Uid::Id(planet.planet_id)).await?;

                route.add(WayPoint::new(
                    planet.planet_id,
                    &planet.name,
                    &planet.position,
                ));

                let subroute = routes
                    .entry(planet.planet_id)
                    .or_insert(Route::new(start.clone()));
                for id in belts {
                    let belt = api.load::<universe::AsteroidBelt>(&Uid::Id(*id)).await?;
                    subroute.add(WayPoint::new(*id, &belt.name, &belt.position));
                }
            }
        }

        let mut idx: u32 = 1u32;

        route.build_best();
        print(&route, &mut idx);

        idx = 1;
        let mut start = None;
        for id in route.complete() {
            if let Some(waypoint) = route.get(&id) {
                if let Some(route) = routes.get_mut(&waypoint.id) {
                    if let Some(start) = start {
                        route.set_departue(start);
                    }
                    route.build_best();
                    print(&route, &mut idx);
                }
                start = Some(waypoint.clone());
            }
        }
    }

    Ok(())
}

fn print(route: &Route, idx: &mut u32) {
    let mut skip = true;
    for id in route.complete() {
        if let Some(wp) = route.get(&id) {
            if skip {
                println!("   {}", wp);
                skip = false;
            } else {
                println!("{:02} {}", idx, wp);
                *idx += 1;
            }
        }
    }
    // println!("Total route length {:.0} Mm", route.len() / 1_000_000.0);
    // println!();
}
