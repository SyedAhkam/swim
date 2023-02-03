# Swim ⚡🏊

![Crates.io](https://img.shields.io/crates/v/swim)
![docs.rs](https://img.shields.io/docsrs/swim)
![Crates.io](https://img.shields.io/crates/d/swim)
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/SyedAhkam/swim/build-and-test.yml?style=plastic)
![Crates.io](https://img.shields.io/crates/l/swim)

<!-- cargo-rdme start -->

An opinionated batteries-included approach to a rust web framework.

The idea is to take the best parts of the rust ecosystem and combine them into a framework that is easy to use and provides a good developer experience.

## Features

- Go blazingly fast with [hyper](https://github.com/hyperium/hyper) and [tokio](https://github.com/tokio-rs/tokio)
- Powerful routing with [routerify](https://github.com/routerify/routerify)
- CLI tooling with [cargo-swim](cargo-swim) (coming soon)
- Database support with [SeaORM](https://github.com/SeaQL/sea-orm) (planned)
- Templating with [Tera](https://github.com/Keats/tera) (planned)
- Dependency injection (planned)

Note: This project is still in development.

## Building a project

You define a project by defining a struct that implements the `Project` trait. It is the highest-level abstraction in the framework. It is responsible for defining the settings, apps, and middleware for your project.

```rust
use swim::prelude::*;

struct MyProject;

impl Project for MyProject {
    fn settings(&self) -> Settings {
        Settings::builder()
            .extend_ron(relative! ("settings.ron"))
            .build()
    }

    fn apps(&self) -> Vec<Box<dyn App>> {
        vec! [
            MyApp.into()
        ]
    }

    fn middleware(&self) -> Vec<Box<dyn Middleware>> {
        vec! [
            MyMiddleware.into()
        ]
    }
}

```

## Building apps

You define an app by defining a struct that implements the `App` trait. It is responsible for defining the routes and views for your app.

```rust
use swim::prelude::*;

struct MyApp;

impl App for MyApp {
    fn mount(&self) -> &'static str {
        "/"
    }

    fn config(&self) -> AppConfig {
        AppConfig::with_name("MyApp")
    }

    fn models(&self) -> Vec<Box<dyn Model>> {
        vec! []
    }

    fn routes(&self) -> Vec<Route> {
        vec! [
            Route::new("/", IndexView),
            Route::new("/hello", HelloView)
        ]
    }
}

```

## Building views

You define a view by defining a struct that implements the `View` trait. It is responsible for handling the request and returning a response. You can implement the specific HTTP methods you want to handle.

```rust
#[derive(Debug)]
pub struct HelloView;

#[async_trait::async_trait]
impl View for HelloView {
    async fn get(&self, request: Request<Body>) -> Result<Response<Body>> {
        Ok(Response::builder()
            .status(StatusCode::OK)
            .body(Body::from("Say hello to Swim! "))
            .unwrap())
    }

    async fn post(&self, request: Request<Body>) -> Result<Response<Body>> {
        Ok(Response::builder()
            .status(StatusCode::OK)
            .body(Body::from("It's a post request! "))
            .unwrap())
    }
}

```

## Defining middlewares

You define a middleware by defining a struct that implements the `Middleware` trait. It is responsible for handling the request and returning a response. You can implement the specific HTTP methods you want to handle.

```rust
#[derive(Debug)]
pub struct Logger;

#[async_trait::async_trait]
impl Middleware for Logger {
    async fn pre(&self, request: Request<Body>) -> Result<Request<Body>> {
        println! ("New request: {:?}", request.uri());

        Ok(request)
    }

    async fn post(&self, response: Response<Body>) -> Result<Response<Body>> {
        println! ("Response: {:?}", response.status());

        Ok(response)
    }
}
```

## Running the project

You may use the elegant swim macro to run your project.

```rust
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    swim! (MyProject, host = "localhost", port = 8000);
}
```

<!-- cargo-rdme end -->


## Current status

The device has been built, but the batteries are not yet included.

## Contributing

Feel free to open an issue or a PR if you have any ideas or suggestions. This project is all about new ideas and making the developer experience better.
