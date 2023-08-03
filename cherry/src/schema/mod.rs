use async_graphql::{
    types::ID, Context, EmptyMutation, EmptySubscription, Enum, Object, Result, Schema,
};
pub type CoffeeSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
use uuid::Uuid;

pub struct CoffeeData {
    id: Uuid,
    name: String,
    roaster: String,
    process: Process,
    grower: Option<String>,
    description: String,
    experiments: Vec<Experiment>,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
enum Process {
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

    async fn experiments(&self) -> &Vec<Experiment> {
        &self.experiments
    }
}

pub struct Experiment {
    id: Uuid,
}

#[Object]
impl Experiment {
    async fn id(&self) -> Result<ID> {
        Ok(self.id.try_into()?)
    }
}

pub struct Coffees {
    coffees: Vec<CoffeeData>,
}

impl Coffees {
    fn get(&self, coffee_id: Uuid) -> Option<&CoffeeData> {
        let m = self.coffees.iter().find(|a| a.id == coffee_id);
        m
    }
}

impl Coffees {
    pub fn new() -> Self {
        let mut coffees = vec![];

        coffees.push(CoffeeData {
            id: Uuid::new_v4(),
            name: "catalyst - jairo arcila".into(),
            experiments: vec![Experiment { id: Uuid::new_v4() }],
            roaster: "Catalyst".into(),
            process: Process::NaturalAnerobic,
            grower: Some("Jairo Arcila".into()),
            description: "Peachy!".into(),
        });

        Self { coffees }
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn coffee<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        #[graphql(desc = "The id of the coffee data to get")] id: ID,
    ) -> Result<Option<&'ctx CoffeeData>> {
        let coffees = ctx.data::<Coffees>()?;
        Ok(coffees.get(id.try_into()?))
    }

    async fn coffees<'ctx>(&self, ctx: &Context<'ctx>) -> Result<&'ctx Vec<CoffeeData>> {
        let coffees = ctx.data::<Coffees>()?;
        Ok(&coffees.coffees)
    }
}
