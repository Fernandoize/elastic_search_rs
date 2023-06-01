use elasticsearch::{Error, Elasticsearch, http::{transport::{SingleNodeConnectionPool, TransportBuilder}, Url}, auth::Credentials, SearchParts};
use serde_json::{json, Value};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let credentials = Credentials::Basic("username".into(), "password".into());
    let url = Url::parse("http://example.com")?;
    let conn_pool = SingleNodeConnectionPool::new(url);
    let transport = TransportBuilder::new(conn_pool).disable_proxy().auth(credentials).build()?;
    let client = Elasticsearch::new(transport);


    let search_response = client
        .search(SearchParts::Index(&["test_index"]))
        .body(json!({
            "query": {
                "match": {
                    "testNo": "000"
                }
            }
        }))
        .allow_no_indices(false)
        .send()
        .await?;

    // get the HTTP response status code
    let status_code = search_response.status_code();

    // read the response body. Consumes search_response
    let response_body = search_response.json::<Value>().await?;


    for hit in response_body["hits"]["hits"].as_array().unwrap() {
        println!("{:?}", hit["_source"]["createdAt"].as_str().unwrap());
    }

    Ok(())
}
