pub mod nhql;
pub mod nhentai;

use async_graphql::{ 
    Schema,
    MergedObject, 
    EmptyMutation, 
    EmptySubscription, 
    extensions::ApolloTracing 
};

use nhentai::NHentaiQueryRoot;
use nhql::NhqlQueryRoot;

#[derive(MergedObject, Default)]
pub struct Query(NHentaiQueryRoot, NhqlQueryRoot);

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> AppSchema {
    Schema::build(
        Query::default(),
        EmptyMutation,
        EmptySubscription
    )
    .extension(ApolloTracing)
    .finish()
}