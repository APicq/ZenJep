use anyhow::bail;
use async_std::{self, sync::Mutex};
use clap::{Arg, Command};
use std::sync::Arc;
use zenjep::applicationstate::AppState;
use zenjep::pages::allpages::*;
use zenjep::renderable::filters::FlightFilter;


#[async_std::main]
async fn main() -> anyhow::Result<()> {
    // Command line arguments
    let matches = Command::new("zenjep")
        .version("0.1")
        .author("me")
        .about("Jeppesen logbook and additions, as http server")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("Input file, yaml logbook"),
        )
        .arg(
            Arg::new("loglevel")
                .short('l')
                .long("loglevel")
                .value_name("LOGLEVEL")
                .help("log level: info, warn, trace. Defaults to info"),
        )
        .get_matches();

    // get the name of the yaml jeppesen logbook
    let yaml_file = if let Some(input) = matches.get_one::<String>("file") {
        input
    } else {
        bail!("No input file. The yaml logook file is missing.");
    };

    // fetch the log level from the command line
    let raw_log_level = if let Some(log_level) = matches.get_one::<String>("loglevel") {
        log_level
    } else {
        "info"
    };

    let log_level = match raw_log_level {
        "error" => log::Level::Error,
        "info" => log::Level::Info,
        "warn" => log::Level::Warn,
        "debug" => log::Level::Debug,
        "trace" => log::Level::Trace,
        _ => {
            bail!("unknow log level")
        }
    };
    simple_logger::init_with_level(log_level).unwrap();

    let appstate = AppState {
        filename: Arc::new(Mutex::new(yaml_file.to_string())),
        filter: Arc::new(Mutex::new(FlightFilter::default())),
    };

    let mut app = tide::with_state(appstate);
    app.with(tide::log::LogMiddleware::new());
    app.at("/").get(page_home);
    app.at("/jeppesen").get(page_jeppesen);
    app.at("/years").get(page_years);
    app.at("/aircrafts").get(page_aircrafts);
    app.at("/familiesandmodels").get(page_families);
    app.at("/filters").get(page_filter);

    app.listen("127.0.0.1:2454").await?;
    Ok(())
}
