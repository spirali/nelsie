use std::io::ErrorKind;
use std::path::Path;

pub(crate) fn ensure_directory(path: &Path) -> Result<(), std::io::Error> {
    if let Err(e) = std::fs::create_dir(path)
        && e.kind() != ErrorKind::AlreadyExists
    {
        return Err(e);
    }
    Ok(())
}
