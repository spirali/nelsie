use std::io::ErrorKind;
use std::path::{Path, PathBuf};

pub(crate) fn ensure_directory(path: &Path) -> Result<(), std::io::Error> {
    if let Err(e) = std::fs::create_dir(path)
        && e.kind() != ErrorKind::AlreadyExists
    {
        return Err(e);
    }
    Ok(())
}

pub(crate) fn safe_write(path: &Path, mem: &[u8]) -> crate::Result<()> {
    let mut tmp_path = PathBuf::from(path);
    if let Some(ext) = tmp_path.extension() {
        let new_ext = format!("{}.tmp", ext.to_string_lossy());
        tmp_path.set_extension(&new_ext);
    } else {
        tmp_path.set_extension("tmp");
    }
    std::fs::write(&tmp_path, mem)?;
    std::fs::rename(&tmp_path, path)?;
    Ok(())
}
