use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "fdb-cli", about = "foundation db cli tool")]
pub enum Opts {
    // Deleting a keyspace
    Delete(Delete),

    // Moving a keyspace
    Move(Move),

    // Setup a foundation db instance
    Setup(Setup),

    // Reset the fdb indexes
    Reset
}

#[derive(Debug, StructOpt)]
pub struct Move {
    #[structopt(short = "k", long, help = "Key space to copy")]
    pub key_space: String,

    #[structopt(short = "t", long, help = "Target key space to move items to")]
    pub target_space: String
}

#[derive(Debug, StructOpt)]
pub struct Delete {
    #[structopt(short = "k", long, help = "Key space to delete")]
    pub key_space: String,
}

#[derive(Debug, StructOpt)]
pub struct Setup {
    #[structopt(short = "cf", long, help = "Path to cluster file")]
    pub cluster_file: String,
}

pub fn parse() -> Opts {
    return Opts::from_args();
}