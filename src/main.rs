use clap::Parser;

pub mod chatgpt;
pub mod config;
pub mod tool;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the executable to explain
    name: String,

    /// What do you want to do with the executable
    #[arg(short, long)]
    question: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let config = config::load()?;
    let tool = tool::find_for(&args.name)?;

    let question = match args.question {
        Some(parsed_question) => String::from(parsed_question),
        None => String::from("Give me a quick summary about this tool."),
    };

    chatgpt::fetch_answer(config, tool, question).await?;

    Ok(())
}
