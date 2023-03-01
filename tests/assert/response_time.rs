use crate::HttpMockServer;
use grillon::{dsl::is_less_than, Grillon, Result};

#[tokio::test]
async fn response_time_less_than() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.delete_valid_user();

    Grillon::new(&mock_server.server.url("/"))?
        .delete("users/1")
        .assert()
        .await
        .response_time(is_less_than(100));

    mock.assert();

    Ok(())
}
