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

    pub(crate) fn delete_coffee(&mut self, coffee_id: Uuid) -> eyre::Result<()> {
        let coffee_position = self
            .coffees
            .iter()
            .position(|coffee| coffee.id == coffee_id);
        let Some(coffee_position) =coffee_position else{
            return Err(eyre::eyre!("Failed to delete coffee with id {coffee_id}"));
        };

        self.coffees.remove(coffee_position);

        Ok(())
    }

    pub(crate) fn add_new_experiment(
        &mut self,
        coffee_id: Uuid,
        new_experiment: ExperimentData,
    ) -> eyre::Result<()> {
        let coffee = self
            .get_coffee_by_id_mut(coffee_id)
            .ok_or_else(|| eyre::eyre!("Failed to find coffee for requested id {coffee_id}"))?;

        coffee.add_new_experiment(new_experiment);
        Ok(())
    }

    pub(crate) fn delete_experiment(
        &mut self,
        coffee_id: Uuid,
        experiment_id: Uuid,
    ) -> eyre::Result<()> {
        let coffee = self
            .get_coffee_by_id_mut(coffee_id)
            .ok_or_else(|| eyre::eyre!("Failed to find coffee for requested id {coffee_id}"))?;

        coffee.delete_experiment(experiment_id)?;
        Ok(())
    }
}

impl CherryData {
    pub fn load() -> eyre::Result<Self> {
        load_data_from_disk()
    }
}
