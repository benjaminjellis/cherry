use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{coffee_data::CoffeeData, experiments_data::ExperimentData};
use crate::db::load_data_from_disk;

#[derive(Serialize, Deserialize)]
pub(crate) struct CherryData {
    pub(crate) coffees: Vec<CoffeeData>,
}

impl CherryData {
    pub(crate) fn get_coffee_by_id(&self, coffee_id: Uuid) -> Option<&CoffeeData> {
        let coffee = self.coffees.iter().find(|a| a.id == coffee_id);
        coffee
    }

    pub(crate) fn get_coffee_by_id_mut(&mut self, coffee_id: Uuid) -> Option<&mut CoffeeData> {
        let coffee_mut = self.coffees.iter_mut().find(|a| a.id == coffee_id);
        coffee_mut
    }

    pub(crate) fn add_new_coffee(&mut self, new_coffee: CoffeeData) {
        self.coffees.push(new_coffee);
    }

    pub(crate) fn add_new_experiment(
        &mut self,
        coffee_id: Uuid,
        new_experiment: ExperimentData,
    ) -> eyre::Result<()> {
        let coffee = self
            .coffees
            .iter_mut()
            .find(|coffee| coffee.id == coffee_id)
            .ok_or_else(|| eyre::eyre!("Failed to find coffee for requested id"))?;

        coffee.add_new_experiment(new_experiment);
        Ok(())
    }
}

impl CherryData {
    pub fn load() -> eyre::Result<Self> {
        load_data_from_disk()
    }
}
