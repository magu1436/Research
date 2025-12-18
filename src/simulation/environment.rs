use arrayfire::{Backend, set_backend};
use std::sync::atomic::{AtomicBool, Ordering};

static BACKEND_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub fn init_backend(backend: Backend) {
    set_backend(backend);
    BACKEND_INITIALIZED.store(true, Ordering::SeqCst);
}

pub(crate) fn assert_backend_initialized() {
    if !BACKEND_INITIALIZED.load(Ordering::SeqCst) {
        panic!("Backend not initialized. Please call init_backend() before using simulation functions.");
    }
}