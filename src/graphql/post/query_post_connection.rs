use async_graphql::*;

use crate::graphql::schema::ContextState;

use sea_orm::{EntityTrait, QueryOrder};
use super::model as post;

#[derive(Default)]
pub struct PostConnectionQuery;

#[Object]
impl PostConnectionQuery {
    async fn post_connection(&self, ctx: &Context<'_>) -> Vec<post::Model> {
        let state = ctx.data_opt::<ContextState>().expect("The specified data type does not exist.");

        post::Entity::find()
            .order_by_asc(post::Column::Id)
            .all(&state.conn)
            .await
            .unwrap()
    }
}

// #[Object]
// impl PostConnectionQuery {
//     async fn post_connection(
//         &self,
//         after: Option<String>,
//         before: Option<String>,
//         first: Option<i32>,
//         last: Option<i32>,
//         ctx: &async_graphql::Context<'_>
//     ) -> async_graphql::Result<Connection<usize, i32, EmptyFields, EmptyFields>> {
//         query(
//             after,
//             before,
//             first,
//             last,
//             |after, before, first, last | async move {
//                 let state = ctx.data_opt::<ContextState>().expect("The specified data type does not exist.");

//             }
//         )
//     }
// }