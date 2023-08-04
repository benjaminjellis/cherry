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

    async fn coffees(&self, ctx: &Context<'_>) -> Result<Vec<CoffeeData>> {
        let coffees = ctx.data::<CherryDataStorage>()?.read().await;
        Ok(coffees.coffees.clone())
    }
}
