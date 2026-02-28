use nakama_core::config::Config;
use nakama_core::error::NakamaResult;
use nakama_ui::NakamaUI;
use std::time::{Duration, Instant};

/// Enter focus mode with a Pomodoro-style timer.
pub async fn run(_config: &Config, ui: &NakamaUI) -> NakamaResult<()> {
    ui.panel("Focus Mode", "Starting a 25-minute focus session.\nPress Ctrl+C to stop early.");

    let duration = Duration::from_secs(25 * 60);
    let start = Instant::now();
    let spinner = ui.step_start("Focusing... 25:00 remaining");

    loop {
        tokio::time::sleep(Duration::from_secs(30)).await;
        let elapsed = start.elapsed();
        if elapsed >= duration {
            break;
        }
        let remaining = duration - elapsed;
        let mins = remaining.as_secs() / 60;
        let secs = remaining.as_secs() % 60;
        spinner.update_message(&format!("Focusing... {:02}:{:02} remaining", mins, secs));
    }

    spinner.finish_with_success("Focus session complete! Take a 5-minute break.");
    Ok(())
}
