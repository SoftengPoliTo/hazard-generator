use std::path::PathBuf;

use clap::builder::{PossibleValuesParser, TypedValueParser};
use clap::Parser;

use tracing_subscriber::EnvFilter;

use hazard_generator::{HazardsProducer, TemplateType};

#[derive(Parser, Debug)]
struct Args {
    /// Path to the ontology file.
    #[clap(short = 'p', value_hint = clap::ValueHint::FilePath)]
    ontology_path: PathBuf,
    /// Name of a builtin template.
    #[clap(long, short, value_parser = PossibleValuesParser::new(TemplateType::all())
    .map(|s| s.parse::<TemplateType>().unwrap()))]
    template: TemplateType,
    /// Parse the risk score associated to the hazards.
    #[clap(long)]
    with_risk: bool,
    /// Path to the output directory.
    #[clap(short, value_hint = clap::ValueHint::DirPath)]
    output_path: PathBuf,
    /// Output the generated template paths as they are produced.
    #[clap(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    // Enable filter to log the information contained in the lib.
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| {
            if args.verbose {
                EnvFilter::try_new("debug")
            } else {
                EnvFilter::try_new("info")
            }
        })
        .unwrap();

    // Run tracer.
    tracing_subscriber::fmt()
        .without_time()
        .with_env_filter(filter_layer)
        .with_writer(std::io::stderr)
        .init();

    // Run HazardsProducer.
    HazardsProducer::new()
        .with_risk(args.with_risk)
        .run(args.ontology_path, args.output_path, args.template)
        .unwrap()
}
