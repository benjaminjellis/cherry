use async_graphql::{Enum, Object, Result, ID};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::experiments_data::ExperimentData;

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct CoffeeData {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) roaster: String,
    pub(crate) process: Process,
    pub(crate) grower: Option<String>,
    pub(crate) description: String,
    pub(crate) roast_date: NaiveDate,
    pub(crate) experiments: Vec<ExperimentData>,
    pub(crate) is_finished: bool,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) enum Process {
    Natural,
    NaturalAnerobic,
    Washed,
    Honey,
    CarbonicMaceration,
}

#[Object]
impl CoffeeData {
    async fn id(&self) -> Result<ID> {
        Ok(self.id.try_into()?)
    }

    async fn name(&self) -> &String {
        &self.name
    }

    async fn description(&self) -> &String {
        &self.description
    }

    async fn grower(&self) -> &Option<String> {
        &self.grower
    }

    async fn roaster(&self) -> &String {
        &self.roaster
    }

    async fn process(&self) -> &Process {
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
}
