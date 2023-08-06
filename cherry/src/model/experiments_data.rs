use async_graphql::{Object, Result, ID};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct ExperimentData {
    pub(crate) id: Uuid,
    pub(crate) date: NaiveDate,
    pub(crate) dose: u8,
    pub(crate) water_dose: u16,
    pub(crate) brewer: String,
    pub(crate) filter: String,
    pub(crate) water: String,
    pub(crate) water_temp: u8,
    pub(crate) pour_structure: String,
    pub(crate) grinder: String,
    pub(crate) grind_setting: String,
    pub(crate) rdt: bool,
    pub(crate) notes: String,
}

#[Object]
impl ExperimentData {
    async fn id(&self) -> Result<ID> {
        Ok(self.id.try_into()?)
    }

    async fn dose(&self) -> u8 {
        self.dose
    }

    async fn water_dose(&self) -> u16 {
        self.water_dose
    }

    async fn date(&self) -> NaiveDate {
        self.date
    }

    async fn brewer(&self) -> &String {
        &self.brewer
    }

    async fn filter(&self) -> &String {
        &self.filter
    }

    async fn water(&self) -> &String {
        &self.water
    }

    async fn water_temp(&self) -> u8 {
        self.water_temp
    }

    async fn grinder(&self) -> &String {
        &self.grinder
    }

    async fn grind_setting(&self) -> &String {
        &self.grind_setting
    }

    async fn rdt(&self) -> bool {
        self.rdt
    }

    async fn notes(&self) -> &String {
        &self.notes
    }

    async fn pour_structure(&self) -> &String {
        &self.pour_structure
    }
}
