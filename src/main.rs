use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};
use itertools::Itertools;

struct AppState {
    foo: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(app_config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .app_data(web::Data::new(AppState {
                foo: "bar".to_string(),
            }))
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/input").route(web::post().to(handle_input))),
    );
}

async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/form.html")))
}

#[derive(Serialize, Deserialize)]
pub struct MyParams {
    tag: String,
    length: String,
    count: String,
}

/// Simple handle POST request
async fn handle_input(params: web::Form<MyParams>) -> Result<HttpResponse> {
    let mut tags: Vec<&str> = Vec::new();

    tags.push("remote");
    tags.push("desktop");
    tags.push("experience");
    tags.push("secure");
    tags.push("reliable");
    tags.push("scalable");
    tags.push("scalable");
    tags.push("platform");
    //tags.push = ["remote", "desktop", "experience", "secure", "reliable", "scalable", "platform"];
    
    permutator(tags);
    
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Your name is {}", params.tag)))
}

pub struct Fragment {
    word_index: i32,
    length: i32,
}

/// Permutator
fn permutator(mut tags: Vec<&str>) {
    let word_index: usize = tags.len();

    //let perms = tags.permutations();
    let mut permutations: Vec<Vec<&&str>> = Vec::new();

    for permutation in tags.iter().permutations(word_index) {
        permutations.push(permutation);
        //permutation.first().unwrap().do_something(vpair.last().unwrap());
    }

    /*     
    itertools::assert_equal(perms, vec![
        vec![5, 6],
        vec![5, 7],
        vec![6, 5],
        vec![6, 7],
        vec![7, 5],
        vec![7, 6],
    ]);
    */
}

#[cfg(test)]
mod tests {
    use actix_web::{
        body::to_bytes,
        dev::{Service, ServiceResponse},
        http::{
            header::{HeaderValue, CONTENT_TYPE},
            StatusCode,
        },
        test::{self, TestRequest},
        web::{Bytes, Form},
    };

    use super::*;

    trait BodyTest {
        fn as_str(&self) -> &str;
    }

    impl BodyTest for Bytes {
        fn as_str(&self) -> &str {
            std::str::from_utf8(self).unwrap()
        }
    }

/*
    #[actix_web::test]
    async fn handle_post_1_unit_test() {
        let params = Form(MyParams {
            name: "John".to_string(),
        });
        let resp = handle_post_1(params).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("text/plain")
        );

        let body = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(body.as_str(), "Your name is John");
    }

    #[actix_web::test]
    async fn handle_post_1_integration_test() {
        let app = test::init_service(App::new().configure(app_config)).await;
        let req = test::TestRequest::post()
            .uri("/post1")
            .set_form(&MyParams {
                name: "John".to_string(),
            })
            .to_request();
        let resp: ServiceResponse = app.call(req).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("text/plain")
        );
        let body = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(body.as_str(), "Your name is John");
    }
*/
}
