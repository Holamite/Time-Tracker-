// !Application Initialization

use crate::errors::SuggestionError;
use error_stack::{ fmt::ColorMode, Report };
use owo_colors::OwoColorize;
use tracing_subscriber::EnvFilter;

pub fn error_reporter() {
    Report::set_color_mode(ColorMode::Color);
    Report::install_debug_hook::<SuggestionError>(|value, content| {
        let msg = value.0;
        let body = format!("Suggestion: {msg}");
        match content.color_mode() {
            ColorMode::Color => content.push_body(body.cyan().bold().to_string()),
            ColorMode::Emphasis => content.push_body(body.italic().to_string()),
            ColorMode::None => content.push_body(body),
        }
    });
}

pub fn tracing() {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    tracing_subscriber
        ::registry()
        .with(tracing_subscriber::fmt::layer().pretty())
        .with(EnvFilter::builder().from_env_lossy())
        .with(ErrorLayer::default())
        .init();
}
