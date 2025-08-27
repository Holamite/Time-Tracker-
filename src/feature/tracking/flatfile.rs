use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
#[error("a flatfile tracking error has occurred")]
struct FlatfileError;

struct FlatfileTracker {
    db: PathBuf,
    lockFile: PathBuf,
}

impl FlatfileTracker {
    fn new<D, L>(db: D, lockFile: L) -> Self where D: Into<PathBuf>, L: Into<PathBuf> {
        let db = db.into();
        let lockFile = lockFile.into();

        Self {
            db,
            lockFile,
        }
    }

    fn start(&self) {
        todo!()
    }

    fn is_running(&self) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_tracking_with_default_tracker() {
        // Given a default tracker
        let tracker = FlatfileTracker::new("db.json", "lockfile.lock");
        // When starting tracking
        tracker.start();
        // Then it should succeed
        assert!(tracker.is_running());
    }
}
