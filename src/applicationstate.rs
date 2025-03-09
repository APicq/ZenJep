use crate::renderable::filters::FlightFilter;
use async_std::{sync::Arc, sync::Mutex};
// todo rewrite with a singe arc mutex
#[derive(Clone)]
pub struct AppState {
    pub filename: Arc<Mutex<String>>,
    pub filter: Arc<Mutex<FlightFilter>>,
}
