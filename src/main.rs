//! Cargo manifest format

// TODO:
// - serde_diff:
//      https://github.com/amethyst/serde-diff/pull/17.
//      https://github.com/amethyst/serde-diff/issues/6
// - Inline Auto
//   если `key.len() + " = ".len() + inline_value.len()` <= max line len
// - package.description
// - подлежащее сказуемое (format, config show --path --toml)

use self::options::{Options, SubCommand};
use anyhow::Result;

use log::{info, trace};

// set RUST_LOG=info
fn main() -> Result<()> {
    env_logger::init();
    let options = Options::new();
    trace!("options: {:?}", options);
    let Options::Pretty(pretty) = &options;
    match &pretty.subcommand {
        None => commands::pretty(&pretty)?,
        Some(SubCommand::Config(config)) => commands::pretty::config(&config)?,
    }
    Ok(())
}

mod commands;
mod options;
