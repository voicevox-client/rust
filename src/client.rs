use crate::restapi::RestAPI;

pub struct Client {
    restapi: RestAPI,
}

impl Client {
    pub fn new(base_path: String) -> Self {
        Self {
            restapi: RestAPI::new(base_path),
        }
    }
}