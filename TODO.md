# API Duplication Resolution: Deprecate GraphQL, Make REST Primary
Status: In Progress | Plan Approved

## Step 1: ✅ Add feature flag to backend/Cargo.toml
- Added `default = []`, `graphql-deprecated = []`.

## Step 2: ✅ Guard GraphQL module with cfg 
- `backend/src/graphql/mod.rs`, `schema.rs`, `resolvers.rs`, `types.rs`: Wrapped with `#[cfg(feature = \"graphql-deprecated\")]`.

## Step 3: Update documentation.md with guidelines [PENDING]
- Add \"API Strategy\" section: REST primary.

## Step 4: Create api_guidelines.md [PENDING]
- REST use cases doc.

## Step 5: Verify no GraphQL router refs [PENDING]
- Search main.rs, api/mod.rs.

## Step 6: Test & Complete [PENDING]

**Next Action**: Step 3-4 (docs).

