use error_stack::{ Result, ResultExt };
use track::{ errors::AppError, feature::cli, init };

// Application Entry Point
// ===============================================
// Track Start
// Track Stop
// Track Status
// Track Report
fn main() -> Result<(), AppError> {
    init::error_reporter();
    init::tracing();
    cli::run().change_context(AppError).attach_printable("Failed to run CLI")?;

    Ok(())
}
