use async_graphql::*;

// CONTEXT
use sea_orm::DatabaseConnection;

pub struct ContextState {
    pub conn: DatabaseConnection
}

// QUERY
use super::post::PostQueries;

#[derive(MergedObject, Default)]
pub struct Query(PostQueries);

// MUTATION
use super::post::PostMutations;

#[derive(MergedObject, Default)]
pub struct Mutation(PostMutations);

// SCHEMA

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn build_schema(context: ContextState) -> AppSchema {
    Schema::build(
        Query::default(),
        Mutation::default(),
        EmptySubscription
    )
    .data(context)
    .finish()
}
