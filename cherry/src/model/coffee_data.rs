use async_graphql::{Object, Result, ID};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::experiments_data::ExperimentData;

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct CoffeeData {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) roaster: String,
    pub(crate) process: String,
    pub(crate) varietal: String,
    pub(crate) country: String,
    pub(crate) grower: Option<String>,
    pub(crate) farm: String,
    pub(crate) tasting_notes: String,
    pub(crate) roast_date: NaiveDate,
    pub(crate) experiments: Vec<ExperimentData>,
    pub(crate) is_finished: bool,
}

#[Object]
impl CoffeeData {
    async fn id(&self) -> Result<ID> {
        Ok(self.id.try_into()?)
    }

    async fn country(&self) -> &String {
        &self.country
    }

    async fn farm(&self) -> &String {
        &self.farm
    }

    async fn varietal(&self) -> &String {
        &self.varietal
    }

    async fn name(&self) -> &String {
        &self.name
    }

    async fn tasting_notes(&self) -> &String {
        &self.tasting_notes
    }

    async fn grower(&self) -> &Option<String> {
        &self.grower
    }

    async fn roaster(&self) -> &String {
        &self.roaster
    }

    async fn process(&self) -> &String {
        &self.process
    }

    async fn experiments(&self) -> &Vec<ExperimentData> {
        &self.experiments
    }

    async fn roast_date(&self) -> NaiveDate {
        self.roast_date
    }

    async fn is_finished(&self) -> bool {
        self.is_finished
    }
}

impl CoffeeData {
    pub(super) fn add_new_experiment(&mut self, new_experiment: ExperimentData) {
        self.experiments.push(new_experiment);
    }

    pub(super) fn delete_experiment(&mut self, experiment_id: Uuid) -> eyre::Result<()> {
        let experiment_idx = self
            .experiments
            .iter()
            .position(|experiment| experiment.id == experiment_id);

        let Some(idx) = experiment_idx else {
            return Err(eyre::eyre!("No experiment with id: {experiment_id} found fo this coffee"))
        };
        _ = self.experiments.remove(idx);
        Ok(())
    }
}
