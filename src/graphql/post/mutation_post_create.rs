use async_graphql::*;

use crate::graphql::schema::ContextState;

use sea_orm::entity::*;
use super::model as post;

#[derive(InputObject)]
struct PostCreateInput {
    title: String,
    body: String,
}

#[derive(Default)]
pub struct PostCreateMutation;

#[Object]
impl PostCreateMutation {
    async fn post_create(&self, ctx: &Context<'_>, input: PostCreateInput) -> post::Model {
        let state = ctx.data_opt::<ContextState>().expect("The specified data type does not exist.");

        let post = post::ActiveModel {
            title: Set(input.title.to_owned()),
            body: Set(input.body.to_owned()),
            ..Default::default()
        };

        let res = post::Entity::insert(post)
            .exec(&state.conn)
            .await
            .expect("could not insert post");

        post::Entity::find_by_id(res.last_insert_id)
            .one(&state.conn)
            .await
            .expect("could not find post")
            .unwrap()
    }
}
