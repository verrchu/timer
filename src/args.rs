use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(StructOpt, Debug, Serialize, Deserialize)]
pub struct Args {
    #[structopt(long)]
    pub config: PathBuf,
}
