use crate::common::models::MyError;
use super::graphql::{account_activity_query, AccountActivityQuery};
use graphql_client::reqwest::post_graphql;



pub async fn load_data(
    public_key: Option<String>,
) -> Result<account_activity_query::ResponseData, MyError> {
    let url = "https://graphql.minaexplorer.com";
    let variables = account_activity_query::Variables {
        blocks_sort_by: account_activity_query::BlockSortByInput::BLOCKHEIGHT_DESC,
        snarks_sort_by: account_activity_query::SnarkSortByInput::BLOCKHEIGHT_DESC,
        trans_sort_by: account_activity_query::TransactionSortByInput::BLOCKHEIGHT_DESC,
        blocks_limit: Some(3),
        snarks_limit: Some(3),
        trans_limit: Some(3),
        blocks_query: account_activity_query::BlockQueryInput {
            creator: public_key.clone(),
            canonical: Some(true),
            ..Default::default()
        },
        snarks_query: account_activity_query::SnarkQueryInput {
            prover: public_key.clone(),
            canonical: Some(true),
            ..Default::default()
        },
        trans_query: account_activity_query::TransactionQueryInput {
            from: public_key,
            canonical: Some(true),
            ..Default::default()
        }
    };

    let client = reqwest::Client::new();

    let response = post_graphql::<AccountActivityQuery, _>(&client, url, variables)
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if let Some(errors) = response.errors {
        return Err(MyError::GraphQLError(errors));
    }

    response
        .data
        .ok_or(MyError::GraphQLEmpty("No data available".to_string()))
}
