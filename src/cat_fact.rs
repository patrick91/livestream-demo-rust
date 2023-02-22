use async_graphql::SimpleObject;
use serde::Deserialize;

#[derive(SimpleObject, Deserialize)]
pub(crate) struct CatFact {
    pub(crate) fact: String,
}
