use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AppStateData {

}

impl AppStateData {
    pub async fn new() -> Self {
        Self {

        }
    }
}

pub type AppState = Arc<AppStateData>;
