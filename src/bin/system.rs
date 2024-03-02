use std::env;

// use crate::universe;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    let args: Vec<String> = env::args().collect();

    if let Some((cmd, names_ref)) = args.split_first() {
        let names = names_ref.to_vec();
        if names.is_empty() {
            println!("Usage\n\t{} <EveSystemName>", cmd);
        } else {
            // let universe = Universe::load(&names);
        }
    }

    Ok(())
}