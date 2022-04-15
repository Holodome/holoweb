use thirtyfour::prelude::*;
use crate::common::TestApp;

#[tokio::test]
async fn test_there_is_login_button_pointing_to_login() {
    let app = TestApp::spawn().await;
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:4444", caps).await.unwrap();

    driver.get(app.rel_addr("/")).await.unwrap();
    let anchor = driver.find_element(By::Id("login")).await.unwrap();
    anchor.click().await.unwrap();

    let url = driver.current_url().await.unwrap();
    assert_eq!(url, app.rel_addr("/login"));

    driver.quit().await.unwrap();
}