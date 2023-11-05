use clap::Parser;
use nelsie::{render_slide_deck, OutputConfig, Result};
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    debug: bool,

    #[arg(long, value_name = "FILE")]
    output_pdf: Option<PathBuf>,

    #[arg(long, value_name = "FILE")]
    output_svg: Option<PathBuf>,

    #[arg(long, value_name = "FILE")]
    output_png: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut builder = env_logger::Builder::default();
    builder.filter_level(if cli.debug {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    });

    let input = std::io::read_to_string(std::io::stdin())?;
    let output_cfg = OutputConfig {
        output_pdf: cli.output_pdf.as_deref(),
        output_svg: cli.output_svg.as_deref(),
        output_png: cli.output_png.as_deref(),
    };
    render_slide_deck(&input, &output_cfg)
}
