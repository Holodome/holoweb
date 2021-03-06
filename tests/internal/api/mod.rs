mod account;
mod blog_posts;
mod change_name;
mod change_password;
mod comments;
mod health_check;
mod home;
mod login;
mod users;

fn strip_from_query_params(s: &str) -> &str {
    let url = s.split("#").next().unwrap();
    let mut url = url.split("?").next().unwrap();
    if url.len() != 1 {
        url = url.trim_end_matches('/');
    }
    url
}

pub fn assert_is_redirect_to_resource(response: &reqwest::Response, location: &str) {
    assert_eq!(
        response.status().as_u16(),
        303,
        "Response is not redirect as expected: {:?}",
        response
    );
    let url = strip_from_query_params(
        response
            .headers()
            .get("Location")
            .unwrap()
            .to_str()
            .unwrap(),
    );
    assert_eq!(url, location, "Response is redirect to different location");
}

pub fn assert_resp_ok(response: &reqwest::Response) {
    assert_eq!(
        response.status().as_u16(),
        200,
        "Response is not OK: {:?}",
        response
    )
}
