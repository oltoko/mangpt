use clap::Parser;

pub mod chatgpt;
pub mod config;
pub mod tool;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the executable to explain
    name: Option<String>,

    /// What do you want to do with the executable
    #[arg(short, long)]
    question: Option<String>,

    /// Show the full path to the config file
    #[arg(short, long)]
    show_config_path: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.show_config_path {
        println!("{}", config::config_file_path()?.display());
        return Ok(());
    }

    if args.name.is_none() {
        return Err("Please provide the name of the executable".into());
    }

    let name = args.name.unwrap();
    let config = config::load()?;
    let tool = tool::find_for(&name)?;

    let question = args
        .question
        .unwrap_or_else(|| String::from("Give me a quick summary about this tool."));

    chatgpt::fetch_answer(config, tool, question).await?;
    println!();

    Ok(())
}
