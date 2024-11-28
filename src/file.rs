use std::path::Path;
use std::time::{Duration, SystemTime};

#[derive(Debug)]
pub struct FileObj {
    pub path: String,
    pub ext: String,
    pub bytes: i64,
    pub created: Duration,
    pub modified: Duration,
}

impl From<Box<Path>> for FileObj {
    fn from(value: Box<Path>) -> Self {
        let path = value.to_str().unwrap().to_owned();
        // let m = value.metadata();
        // let bytes = m.len() as i64;
        let ext: String = value
            .extension()
            .unwrap_or("".as_ref())
            .to_str()
            .unwrap()
            .to_owned();

        let mut created: Duration = Duration::default();
        let mut modified: Duration = Duration::default();
        let mut bytes: i64 = 0;
        if let Ok(m) = value.metadata() {
            bytes = m.len() as i64;

            created = m
                .created()
                .unwrap_or(SystemTime::UNIX_EPOCH)
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default();
            modified = m
                .modified()
                .unwrap_or(SystemTime::UNIX_EPOCH)
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default();
        }
        return Self {
            path,
            ext,
            bytes,
            created,
            modified,
        };
    }
}
