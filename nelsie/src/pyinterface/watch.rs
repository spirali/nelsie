use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use pyo3::exceptions::PyException;
use pyo3::{pyfunction, PyResult, Python};
use std::thread::sleep;
use std::time::Duration;

#[pyfunction]
pub(crate) fn watch(py: Python<'_>, paths: Vec<String>) -> PyResult<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())
        .map_err(|e| PyException::new_err(e.to_string()))?;

    for path in paths {
        watcher
            .watch(path.as_ref(), RecursiveMode::NonRecursive)
            .map_err(|e| PyException::new_err(e.to_string()))?;
    }
    loop {
        py.check_signals()?;
        if let Ok(Ok(event)) = rx.recv_timeout(Duration::from_secs(1)) {
            match event.kind {
                EventKind::Modify(_) | EventKind::Remove(_) => {
                    sleep(Duration::from_millis(10));
                    return Ok(());
                }
                _ => {}
            }
        }
    }
}
