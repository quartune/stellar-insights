#[cfg(feature = "graphql-deprecated")]
pub mod schema;
#[cfg(feature = "graphql-deprecated")]
pub mod types;
#[cfg(feature = "graphql-deprecated")]
pub mod resolvers;

#[cfg(test)]
mod tests;

#[cfg(feature = "graphql-deprecated")]
pub use schema::{build_schema, AppSchema};
