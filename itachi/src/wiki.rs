use crate::ai_helper::{ask_ai, make_provider};
use crate::atlassian::AtlassianClient;
use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::error::NakamaResult;
use nakama_core::trace::TraceContext;
use nakama_core::types::ModelTier;
use nakama_ui::NakamaUI;
use std::time::Instant;

/// Search Confluence with natural language, translating to CQL via AI.
pub async fn run(config: &Config, ui: &NakamaUI, query: &str) -> NakamaResult<()> {
    let trace = TraceContext::new("itachi", "wiki");
    let start = Instant::now();

    let spinner = ui.step_start("Translating query to CQL...");
    let (provider, model) = make_provider(config, ModelTier::Fast)?;

    let system_prompt = r#"You are a Confluence CQL expert. Convert the natural language query to a CQL search string.
Return ONLY the CQL query, nothing else.

Examples:
- "deployment docs" → type = page AND text ~ "deployment"
- "onboarding guide for new engineers" → type = page AND text ~ "onboarding" AND text ~ "engineer"
- "API documentation updated recently" → type = page AND text ~ "API" AND lastModified > now("-30d")"#;

    let cql = ask_ai(provider.as_ref(), system_prompt, query, &model, 256, 0.1).await?;
    let cql = cql.trim().trim_matches('`').trim();
    spinner.finish_with_success(&format!("CQL: {}", cql));

    let spinner = ui.step_start("Searching Confluence...");
    let client = AtlassianClient::new()?;
    let result = client.confluence_search(cql, 10).await?;

    let elapsed = start.elapsed().as_millis() as u64;

    // Parse results
    let results = result.get("results").and_then(|r| r.as_array());
    let total = result.get("totalSize").and_then(|t| t.as_u64()).unwrap_or(0);

    spinner.finish_with_success(&format!("Found {} pages", total));

    if let Some(pages) = results {
        let rows: Vec<Vec<String>> = pages.iter().map(|page| {
            vec![
                page.get("title").and_then(|t| t.as_str()).unwrap_or("?").to_string(),
                page.get("type").and_then(|t| t.as_str()).unwrap_or("?").to_string(),
                page.get("_links").and_then(|l| l.get("webui")).and_then(|w| w.as_str()).unwrap_or("").to_string(),
            ]
        }).collect();

        if !rows.is_empty() {
            ui.table(&["Title", "Type", "Path"], rows);
        }
    }

    if let Ok(audit) = AuditLog::new(&config.audit) {
        let entry = AuditEntry::new(
            &trace.trace_id, "itachi", "wiki", Category::ExternalApi,
            &format!("Confluence search: {}", &query[..query.len().min(80)]),
            serde_json::json!({ "query": query, "cql": cql, "results": total }),
            Outcome::Success, elapsed,
        );
        let _ = audit.log(entry);
    }

    Ok(())
}
