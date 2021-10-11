use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "fdb-cli", about = "foundation db cli tool")]
pub enum Opts {
    // Deleting a keyspace
    Delete(Space),

    // Get a key value
    Get(Space),

    // TODO: Moving a keyspace
    Move,

    // Setup a foundation db instance
    Setup(Setup),

    // TODO: Reset the fdb indexes
    Reset,
}

#[derive(Debug, StructOpt)]
pub enum Space {
    Key(Key),

    Range(Range)
}

#[derive(Debug, StructOpt)]
pub struct Key {
    pub key: String,
}

#[derive(Debug, StructOpt)]
pub struct Range {
    #[structopt(short = "s", long, help = "Starting key range identifier")]
    pub start: String,

    #[structopt(short = "e", long, help = "Ending key range identifier")]
    pub end: Option<String>
}

#[derive(Debug, StructOpt)]
pub enum Setup {
    Set(Set),

    View,
}

#[derive(Debug, StructOpt)]
pub struct Set {
    #[structopt(short = "cf", long, help = "Path to cluster file")]
    pub cluster_file: String,
}

pub fn parse() -> Opts {
    return Opts::from_args();
}