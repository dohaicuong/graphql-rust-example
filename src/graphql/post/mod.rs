use async_graphql::*;

pub mod model;

// merge queries
mod query_post_connection;

#[derive(MergedObject, Default)]
pub struct PostQueries(
    query_post_connection::PostConnectionQuery,
);

// merge mutations
mod mutation_post_create;

#[derive(MergedObject, Default)]
pub struct  PostMutations(
    mutation_post_create::PostCreateMutation,
);
