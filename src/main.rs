use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /*
    See https://wiki.xmltv.org/index.php/XmltvCapabilities for a full
    description of CLI parameters needed to comply with the XMLTV standard.

    Targeted capabilities:
        - baseline
        - manualconfig
        - cache
    */
    /// Print a description that identifies this grabber
    #[arg(long = "description")]
    print_description: bool, // TODO

    /// Print a list of this grabber's capabilities.
    /// See https://wiki.xmltv.org/index.php/XmltvCapabilities for more
    /// information
    #[arg(long = "capabilities")]
    print_capabilities: bool, // TODO

    /// Bring up interactive configuration dialogue
    #[arg(long = "configure")]
    perform_configuration: bool, // TODO

    /// Suppress all progress information
    #[arg(short = 'q', long = "quiet")]
    be_quiet: bool, // TODO

    /// Redirect output to the specified file instead of printing to stdout
    #[arg(short, long = "output", value_name = "FILE")]
    output_file: Option<PathBuf>, // TODO

    /// The number of days to fetch programme data for
    #[arg(short, long)]
    days: Option<i32>, // TODO

    /// Start fetching programme data starting $DAYS days in the future
    #[arg(short = 'O', long, value_name = "N")]
    offset: Option<i32>, // TODO

    /// Read and write all configuration to and from this file
    #[arg(short = 'c', long = "config-file", value_name = "FILE")]
    config_file: Option<PathBuf>, // TODO

    /// Cache fetched data in the specified file
    #[arg(short = 'C', long = "cache", value_name = "FILE")]
    cache_file: Option<PathBuf>, // TODO
}

fn main() {
    let args = Args::parse();

    println!("Inputs: {:#?}", args);
}
