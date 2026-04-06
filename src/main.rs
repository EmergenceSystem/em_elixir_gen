use aleph_syntax_tree::syntax::AlephTree;
use em_filter::{async_trait, AgentConfig, EmFilterError, Filter, FilterRunner};
use serde_json::{json, Value};

struct ElixirGenAGent;

#[async_trait]
impl Filter for ElixirGenAGent {
    async fn handle(&mut self, body: &str) -> Result<Value, EmFilterError> {
        tracing::info!(len = body.len(), "generating elixir code");

        let embryo: Value = serde_json::from_str(body)?;
        let tree: AlephTree = serde_json::from_value(embryo["properties"]["tree"].clone())?;
        let source_lang = embryo["properties"]["source_language"]
            .as_str()
            .unwrap_or("unknown")
            .to_string();

        let code = elixirgen::generate(tree);

        tracing::info!(source_lang = %source_lang, "generated elixir code");

        Ok(json!([{
            "type": "code",
            "properties": {
                "language": "elixir",
                "source_language": source_lang,
                "content": code
            }
        }]))
    }

    fn capabilities(&self) -> Vec<String> {
        vec!["generator".into(), "elixir".into()]
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    FilterRunner::new("em_elixir_gen", ElixirGenAGent, AgentConfig::default())
        .run()
        .await
        .unwrap();
}
