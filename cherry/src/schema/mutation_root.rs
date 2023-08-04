use async_graphql::{Context, Object, Result, ID};
use chrono::NaiveDate;
use uuid::Uuid;

use crate::model::{CherryDataStorage, CoffeeData, ExperimentData, Process};

pub(crate) struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn add_new_coffee(
        &self,
        ctx: &Context<'_>,
        name: String,
        roaster: String,
        process: Process,
        grower: Option<String>,
        description: String,
        roast_date: NaiveDate,
    ) -> Result<bool> {
        let mut coffees = ctx.data::<CherryDataStorage>()?.write().await;
        let coffee_data = CoffeeData {
            id: Uuid::new_v4(),
            name,
            roaster,
            process,
            grower,
            description,
            roast_date,
            experiments: vec![],
            is_finished: false,
        };
        coffees.add_new_coffee(coffee_data);
        Ok(true)
    }

    async fn mark_coffee_as_finished(&self, ctx: &Context<'_>, coffee_id: ID) -> Result<bool> {
        let mut coffees = ctx.data::<CherryDataStorage>()?.write().await;
        let Some(coffee) = coffees.get_coffee_by_id_mut(coffee_id.try_into()?) else{
            return Ok(false);
        };
        coffee.is_finished = true;
        Ok(true)
    }

    async fn add_new_experiment(
        &self,
        ctx: &Context<'_>,
        coffee_id: ID,
        experiment_date: NaiveDate,
        dripper: String,
        filter: String,
        water: String,
        water_temp: u8,
        grinder: String,
        grind_setting: String,
        rdt: bool,
        notes: String,
    ) -> Result<bool> {
        let mut coffees = ctx.data::<CherryDataStorage>()?.write().await;
        let new_experiment = ExperimentData {
            id: Uuid::new_v4(),
            date: experiment_date,
            dripper,
            filter,
            water,
            water_temp,
            grinder,
            grind_setting,
            rdt,
            notes,
        };
        coffees.add_new_experiment(coffee_id.try_into()?, new_experiment)?;
        Ok(true)
    }
}
