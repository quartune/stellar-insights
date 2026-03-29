#[cfg(feature = "graphql-deprecated")]
use async_graphql::{EmptySubscription, Schema};
#[cfg(feature = "graphql-deprecated")]
use sqlx::SqlitePool;
#[cfg(feature = "graphql-deprecated")]
use std::sync::Arc;

#[cfg(feature = "graphql-deprecated")]
use super::resolvers::{MutationRoot, QueryRoot};

#[cfg(feature = "graphql-deprecated")]
pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[cfg(feature = "graphql-deprecated")]
pub fn build_schema(pool: Arc<SqlitePool>) -> AppSchema {
    Schema::build(
        QueryRoot { pool: pool.clone() },
        MutationRoot { pool },
        EmptySubscription,
    )
    .finish()
}
