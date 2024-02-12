use llm_chain::options::{ModelRef, Opt, OptionsBuilder};
use llm_chain::traits::Executor;
use llm_chain::{parameters, prompt};
use llm_chain_openai::chatgpt::{Executor as ChatGptExecutor, Model};

#[tokio::main(flavor = "current_thread")]
pub async fn chat() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    //let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY required");
    //let model = Model::Gpt4_32k;
    let mut builder = OptionsBuilder::new();
    builder.add_option(Opt::Model(ModelRef::from_model_name("gpt-4")));
    let options = builder.build();
    let exec = ChatGptExecutor::new_with_options(options)?;

    let res = prompt!(
        "You are an assistant for making personalized greetings",
        "Make a personalized greetings for Joe"
    )
    .run(&parameters!(), &exec)
    .await?;

    println!("{}", res);
    Ok(())
}
