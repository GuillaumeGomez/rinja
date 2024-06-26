use actix_web::http::header::CONTENT_TYPE;
use actix_web::web;
use bytes::Bytes;
use rinja_actix::Template;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

#[actix_rt::test]
async fn test_actix_web() {
    let srv = actix_test::start(|| {
        actix_web::App::new()
            .service(web::resource("/").to(|| async { HelloTemplate { name: "world" } }))
    });

    let request = srv.get("/");
    let mut response = request.send().await.unwrap();
    assert!(response.status().is_success());
    assert_eq!(
        response.headers().get(CONTENT_TYPE).unwrap(),
        "text/html; charset=utf-8"
    );

    let bytes = response.body().await.unwrap();
    assert_eq!(bytes, Bytes::from_static("Hello, world!".as_ref()));
}
