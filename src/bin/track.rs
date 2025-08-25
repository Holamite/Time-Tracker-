use error_stack::Result;
use track::{ errors::AppError, init };

// Application Entry Point
// ===============================================
// Track Start
// Track Stop
// Track Status
// Track Report
fn main() -> Result<(), AppError> {
    init::error_reporter();
    init::tracing();

    Ok(())
}
