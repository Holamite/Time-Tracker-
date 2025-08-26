use assert_cmd::Command;
use testresult::TestResult;

#[test]
fn status_code_is_error_if_no_command_is_specify() -> TestResult {
    Command::cargo_bin("track")?.assert().failure().code(1);
    Ok(())
}

#[test]
fn start_command_start_tracking_time() -> TestResult {
    Command::cargo_bin("track")?.arg("Start").assert().success().code(1);
    todo!();
}

#[test]
fn stop_command_stop_tracking_time() -> TestResult {
    Command::cargo_bin("track")?.arg("Start").assert().success().code(1);
    Command::cargo_bin("track")?.arg("stop").assert().success().code(1);
    todo!();
}

#[test]
fn report_command_generated_report() -> TestResult {
    Command::cargo_bin("track")?.arg("Start").assert().success().code(1);
    Command::cargo_bin("track")?.arg("Start").assert().success().code(1);
    Command::cargo_bin("track")?.arg("Report").assert().stdout("00:00:00").success().code(1);
    todo!();
}
