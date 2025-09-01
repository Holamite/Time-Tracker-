use std::{ fs::{ remove_file, OpenOptions }, path::PathBuf };

use error_stack::{ Result, ResultExt };

#[derive(Debug, thiserror::Error)]
#[error("a flatfile tracking error has occurred")]
struct FlatfileError;

struct FlatfileTracker {
    db: PathBuf,
    lock_file: PathBuf,
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
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&self.lock_file)
            .change_context(FlatfileError)
            .attach_printable("Failed to create lock file")?;
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.lock_file.exists()
    }

    fn stop(&self) -> Result<(), FlatfileError> {
        remove_file(&self.lock_file)
            .change_context(FlatfileError)
            .attach_printable("Failed to remove lock file")?;
        Ok(())
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
}
