mod cherry_data;
mod coffee_data;
mod experiments_data;

use std::sync::Arc;
use tokio::sync::RwLock;

pub(crate) type CherryDataStorage = Arc<RwLock<CherryData>>;

pub(crate) use cherry_data::CherryData;
pub(crate) use coffee_data::CoffeeData;
pub(crate) use experiments_data::ExperimentData;
