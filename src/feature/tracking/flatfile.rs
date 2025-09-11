use std::{
    fs::{ read_to_string, remove_file, OpenOptions },
    io::{ BufWriter, Write },
    path::PathBuf,
};
use serde::{ Deserialize, Serialize };

use error_stack::{ Result, ResultExt };

use crate::feature::tracking::{ EndTime, StartTime, TimeRecord };
// use super::TimeRecord;

#[derive(Debug, thiserror::Error)]
#[error("a flatfile tracking error has occurred")]
struct FlatfileError;

struct FlatfileTracker {
    db: PathBuf,
    lock_file: PathBuf,
}

#[derive(Serialize, Deserialize)]
struct LockFileData {
    start_time: StartTime,
}

impl FlatfileTracker {
    fn new<D, L>(db: D, lock_file: L) -> Self where D: Into<PathBuf>, L: Into<PathBuf> {
        let db = db.into();
        let lock_file = lock_file.into();

        Self {
            db,
            lock_file,
        }
    }

    fn start(&self) -> Result<(), FlatfileError> {
        // Save the current start time into the lock file
        let Lockfile_data = {
            let start_time = StartTime::now();

            let data = LockFileData {
                start_time,
            };

            serde_json
                ::to_string(&data)
                .change_context(FlatfileError)
                .attach_printable("Failed to serialize lock file data")?
        };

        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&self.lock_file)
            .change_context(FlatfileError)
            .attach_printable("Failed to create lock file")?
            .write_all(Lockfile_data.as_bytes())
            .change_context(FlatfileError)
            .attach_printable("Failed to write start time to lock file")?;

        // // let mut writer = BufWriter::new(file);

        // writer
        //     .write(StartTime::now().to_string().as_bytes())
        //     .change_context(FlatfileError)
        //     .attach_printable("Failed to write start time to lock file")?;

        // writer.flush().change_context(FlatfileError).attach_printable("Failed to flush lock file")?;
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.lock_file.exists()
    }

    fn stop(&self) -> Result<(), FlatfileError> {
        // 1. Read the time from the lock file and create a TimeRecord
        let _start_time = serde_json
            ::from_str(
                &read_to_string(&self.lock_file)
                    .change_context(FlatfileError)
                    .attach_printable("Failed to read lock file")?
            )
            .change_context(FlatfileError)
            .attach_printable("Failed to parse start time from lock file")?;

        // 2. Get end time (EndTime::now())
        let _end_time = EndTime::now();

        // 3. Create a TimeRecord
        let _time_record = TimeRecord {
            start: _start_time,
            end: _end_time,
        };

        // 4. Ensure the db file exists (create if not)
        if !self.db.exists() {
            OpenOptions::new()
                .write(true)
                .create(true)
                .open(&self.db)
                .change_context(FlatfileError)
                .attach_printable("Failed to create db file")?;
        } else {
            // 5. Save the TimeRecord to the db (append to JSON file)
            let file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(&self.db)
                .change_context(FlatfileError)
                .attach_printable("Failed to open db file")?;

            let mut writer = BufWriter::new(file);
            writer
                .write_all(serde_json::to_string(&_time_record).unwrap().as_bytes())
                .change_context(FlatfileError)
                .attach_printable("Failed to write time record to db file")?;
            writer
                .flush()
                .change_context(FlatfileError)
                .attach_printable("Failed to flush db file")?;
        }
        // 6. Remove the lock file
        remove_file(&self.lock_file)
            .change_context(FlatfileError)
            .attach_printable("Failed to remove lock file")?;
        Ok(())
    }

    fn record(&self) -> Result<impl Iterator<Item = TimeRecord>, FlatfileError> {
        // Load records from the db (JSON file) and return an iterator
        let content = read_to_string(&self.db)
            .change_context(FlatfileError)
            .attach_printable("Failed to read db file")?;
        let records = content
            .lines()
            .filter_map(|line| serde_json::from_str::<TimeRecord>(line).ok())
            .collect::<Vec<_>>();
        Ok(records.into_iter())
    }
}

#[cfg(test)]
mod tests {
    use assert_fs::{ prelude::PathChild, TempDir, fixture::ChildPath };
    use super::*;

    fn temp_dir() -> (TempDir, ChildPath, ChildPath) {
        let temp_dir = TempDir::new().unwrap();
        let db = temp_dir.child("db.json");
        let lock_file = temp_dir.child("lockfile.lock");
        (temp_dir, db, lock_file)
    }

    #[test]
    fn is_running_returns_true_after_starting_the_tracker() {
        // Create a temporary directory for testing
        let (_tempDir, db, lock_file) = temp_dir();
        // Given a default tracker has a temporary db and lock file
        let tracker = FlatfileTracker::new(db.to_path_buf(), lock_file.to_path_buf());
        // When starting tracking
        tracker.start().unwrap();
        // Then it should succeed
        assert!(tracker.is_running());
    }

    #[test]
    fn is_running_returns_false_after_stopping_the_tracker() {
        // Create a temporary directory for testing
        let (_tempDir, db, lock_file) = temp_dir();
        // Given a default tracker has a temporary db and lock file
        let tracker = FlatfileTracker::new(db.to_path_buf(), &lock_file.to_path_buf());
        tracker.start().unwrap();
        // When starting tracking
        tracker.stop().unwrap();
        // Then it should return false
        assert!(!tracker.is_running());
    }

    #[test]
    fn time_record_created_when_tracking_stops() {
        // Create a temporary directory for testing
        let (_tempDir, db, lock_file) = temp_dir();
        // Given a default tracker has a temporary db and lock file
        let tracker = FlatfileTracker::new(db.to_path_buf(), &lock_file.to_path_buf());
        tracker.start().unwrap();
        // When starting tracking
        tracker.stop().unwrap();

        // Then record should be created
        assert!(tracker.record().unwrap().next().is_some());
    }
}
