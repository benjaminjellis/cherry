use async_graphql::{Context, Object, Result, ID};

use crate::model::{CherryDataStorage, CoffeeData};

pub(crate) struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn coffee<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        #[graphql(desc = "The id of the coffee data to get")] id: ID,
    ) -> Result<Option<CoffeeData>> {
        let coffees = ctx.data::<CherryDataStorage>()?.read().await;
        Ok(coffees.get_coffee_by_id(id.try_into()?).cloned())
    }

    async fn coffees(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Filter coffees by whether they've been marked as finished or not")]
        is_finished: Option<bool>,
    ) -> Result<Vec<CoffeeData>> {
        dbg!("getting coffees!");
        let coffees = ctx.data::<CherryDataStorage>()?.read().await;

        let Some(is_finished ) = is_finished else{
            return Ok(coffees.coffees.clone());
        };

        if is_finished {
            let filtered_coffees = coffees
                .coffees
                .clone()
                .into_iter()
                .filter(|coffee| coffee.is_finished)
                .collect::<Vec<_>>();
            Ok(filtered_coffees)
        } else {
            let filtered_coffees = coffees
                .coffees
                .clone()
                .into_iter()
                .filter(|coffee| !coffee.is_finished)
                .collect::<Vec<_>>();
            Ok(filtered_coffees)
        }
    }
}
