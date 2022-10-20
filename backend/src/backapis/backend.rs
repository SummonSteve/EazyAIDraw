use super::BackendType;

pub struct Backends {
    pub inner: Vec<Backend>
}

impl Backends {
    pub fn new(sd_backend: Backend, nai_backend: Backend) -> Self {
        Self { inner: vec![sd_backend, nai_backend] }
    }
}

pub struct Backend {
    pub backend_type: BackendType,
    pub busy: bool,
    pub url: String,
}

pub struct BackendStatus {
    pub busy: bool,

}


impl Backend {
    pub fn new(backend_type: BackendType, url: String) -> Self {
        Self { 
            backend_type,
            busy: false,
            url,
        }
    }

    pub fn get_status(&self) -> BackendStatus{
        todo!()
    }
}