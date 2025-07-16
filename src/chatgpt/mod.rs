use std::{
    error::Error,
    io::{stdout, Write},
};

use crate::{config::ManGPTConfig, tool::Tool};
use async_openai::{
    config::OpenAIConfig,
    error::OpenAIError,
    types::CreateChatCompletionRequestArgs,
    types::{ChatCompletionRequestSystemMessage, ChatCompletionRequestSystemMessageArgs},
    types::{ChatCompletionRequestUserMessage, ChatCompletionRequestUserMessageArgs},
    Client,
};
use futures::StreamExt;

pub async fn fetch_answer(
    config: ManGPTConfig,
    tool: Tool,
    question: String,
) -> Result<(), Box<dyn Error>> {
    let model = config.model;
    let max_tokens = config.max_tokens;
    let man_page_content = tool.fetch_man_page()?;

    let openai_config = OpenAIConfig::default().with_api_key(config.api_key);
    let client = Client::with_config(openai_config);

    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .max_tokens(max_tokens)
        .messages([
            create_system_message(&man_page_content)?.into(),
            create_user_message(&question)?.into(),
        ])
        .build()?;

    let mut stream = client.chat().create_stream(request).await?;

    // From Rust docs on print: https://doc.rust-lang.org/std/macro.print.html
    //
    //  Note that stdout is frequently line-buffered by default so it may be necessary
    //  to use io::stdout().flush() to ensure the output is emitted immediately.
    //
    //  The print! macro will lock the standard output on each call.
    //  If you call print! within a hot loop, this behavior may be the bottleneck of the loop.
    //  To avoid this, lock stdout with io::stdout().lock():

    let mut lock = stdout().lock();
    while let Some(result) = stream.next().await {
        match result {
            Ok(response) => {
                response.choices.iter().for_each(|chat_choice| {
                    if let Some(ref content) = chat_choice.delta.content {
                        write!(lock, "{content}").unwrap();
                    }
                });
            }
            Err(err) => {
                writeln!(lock, "error: {err}").unwrap();
            }
        }
        stdout().flush()?;
    }

    Ok(())
}

fn create_system_message(
    man_page_content: &str,
) -> Result<ChatCompletionRequestSystemMessage, OpenAIError> {
    let content = format!("You are manGPT. An AI Assistant which helps the User with his questions about an specific CLI-Tool.
    You are giving precise and short answers to help the user as fast as possible to reach his goal. Your answer should always contain an example.
    Following the Man-Page/Help of the Tool you should give answers to:
    ---
    {man_page_content}
    ---");

    ChatCompletionRequestSystemMessageArgs::default()
        .content(content)
        .build()
}

fn create_user_message(question: &str) -> Result<ChatCompletionRequestUserMessage, OpenAIError> {
    ChatCompletionRequestUserMessageArgs::default()
        .content(question)
        .build()
}
