use sc_cli::{KeySubcommand, RunCmd, SignCmd, VanityCmd, VerifyCmd};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Cli {
    #[structopt(subcommand)]
    pub subcommand: Option<Subcommand>,

    #[structopt(flatten)]
    pub run: RunCmd,
}

/// Possible subcommands of the main binary.
#[derive(Debug, StructOpt)]
pub enum Subcommand {
    /// A set of base subcommands handled by `sc_cli`.
    #[structopt(flatten)]
    Base(sc_cli::Subcommand),

    /// Key management cli utilities
    Key(KeySubcommand),

    /// Verify a signature for a message, provided on STDIN, with a given (public or secret) key.
    Verify(VerifyCmd),

    /// Generate a seed that provides a vanity address.
    Vanity(VanityCmd),

    /// Sign a message, with a given (secret) key.
    Sign(SignCmd),
}
