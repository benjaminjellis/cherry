mod mutation_root;
mod query_root;

use async_graphql::{EmptySubscription, Schema};

pub(crate) use mutation_root::MutationRoot;
pub(crate) use query_root::QueryRoot;

pub(crate) type CherrySchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
