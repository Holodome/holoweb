mod account;
mod blog_posts;
mod change_name;
mod change_password;
mod comments;
mod health_check;
mod home;
mod login;

pub fn assert_is_redirect_to(response: &reqwest::Response, location: &str) {
    assert_eq!(
        response.status().as_u16(),
        303,
        "Response is not redirect as expected: {:?}",
        response
    );
    assert_eq!(
        response.headers().get("Location").unwrap(),
        location,
        "Response is redirect to different location"
    );
}

pub fn assert_resp_ok(response: &reqwest::Response) {
    assert_eq!(
        response.status().as_u16(),
        200,
        "Response is not OK: {:?}",
        response
    )
}

pub fn assert_resp_forbidden(response: &reqwest::Response) {
    assert_eq!(
        response.status().as_u16(),
        403,
        "Response is not FORBIDDEN: {:?}",
        response
    )
}
