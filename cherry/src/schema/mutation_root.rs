use async_graphql::{Context, Object, Result, ID};
use chrono::NaiveDate;
use uuid::Uuid;

use crate::model::{CherryDataStorage, CoffeeData, ExperimentData};

pub(crate) struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn add_new_coffee(
        &self,
        ctx: &Context<'_>,
        name: String,
        roaster: String,
        process: String,
        varietal: String,
        farm: String,
        grower: Option<String>,
        country: String,
        tasting_notes: String,
        roast_date: NaiveDate,
    ) -> Result<ID> {
        let new_coffee_id = Uuid::new_v4();
        let mut coffees = ctx.data::<CherryDataStorage>()?.write().await;
        let coffee_data = CoffeeData {
            id: new_coffee_id,
            name,
            roaster,
            process,
            varietal,
            grower,
            farm,
            tasting_notes,
            roast_date,
            country,
            experiments: vec![],
            is_finished: false,
        };
        coffees.add_new_coffee(coffee_data);
        Ok(new_coffee_id.try_into()?)
    }

    async fn mark_coffee_as_finished(&self, ctx: &Context<'_>, coffee_id: ID) -> Result<bool> {
        let mut coffees = ctx.data::<CherryDataStorage>()?.write().await;
        let Some(coffee) = coffees.get_coffee_by_id_mut(coffee_id.try_into()?) else{
            return Ok(false);
        };
        coffee.is_finished = true;
        Ok(true)
    }

    async fn mark_coffee_as_unfinished(&self, ctx: &Context<'_>, coffee_id: ID) -> Result<bool> {
        let mut coffees = ctx.data::<CherryDataStorage>()?.write().await;
        let Some(coffee) = coffees.get_coffee_by_id_mut(coffee_id.try_into()?) else{
            return Ok(false);
        };
        coffee.is_finished = false;
        Ok(true)
    }

    async fn add_new_experiment(
        &self,
        ctx: &Context<'_>,
        coffee_id: ID,
        experiment_date: NaiveDate,
        brewer: String,
        filter: String,
        water: String,
        water_temp: u8,
        dose: u8,
        water_dose: u16,
        grinder: String,
        grind_setting: String,
        pour_structure: String,
        notes: String,
    ) -> Result<ID> {
        let new_experiment_id = Uuid::new_v4();
        let mut coffees = ctx.data::<CherryDataStorage>()?.write().await;
        let new_experiment = ExperimentData {
            id: new_experiment_id,
            date: experiment_date,
            brewer,
            filter,
            water,
            water_temp,
            dose,
            water_dose,
            pour_structure,
            grinder,
            grind_setting,
            notes,
        };
        coffees.add_new_experiment(coffee_id.try_into()?, new_experiment)?;
        Ok(new_experiment_id.try_into()?)
    }

    async fn delete_experiment(
        &self,
        ctx: &Context<'_>,
        coffee_id: ID,
        experiment_id: ID,
    ) -> Result<ID> {
        let mut coffees = ctx.data::<CherryDataStorage>()?.write().await;
        coffees.delete_experiment(coffee_id.try_into()?, experiment_id.clone().try_into()?)?;
        Ok(experiment_id)
    }

    async fn delete_coffee(&self, ctx: &Context<'_>, coffee_id: ID) -> Result<ID> {
        let mut coffees = ctx.data::<CherryDataStorage>()?.write().await;
        coffees.delete_coffee(coffee_id.clone().try_into()?)?;
        Ok(coffee_id)
    }
}
