use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;
use record_audio::audio_clip::AudioClip;

#[derive(Debug, Subcommand)]
enum Commands {
    Record {
        name: Option<String>,
    },
    #[clap(arg_required_else_help = true)]
    Play {
        path: String,
    },
}

#[derive(Debug, Parser)]
#[clap(name = "record-audio")]
#[clap(about = "simple audio recorder")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

fn main() -> Result<()> {
    color_eyre::install().unwrap();
    let args = Cli::parse();

    match args.command {
        Commands::Record { name } => handle_record(name),
        Commands::Play { path } => handle_play(path),
    }
}

fn handle_record(name: Option<String>) -> Result<()> {
    let clip = AudioClip::record(name)?;
    let filename = format!("{}.wav", clip.name);
    clip.export(&filename)?;
    println!("Audio clip saved as {}.wav", clip.name);
    Ok(())
}

fn handle_play(path: String) -> Result<()> {
    let clip = AudioClip::import(path)?;
    clip.play()?;
    Ok(())
}
