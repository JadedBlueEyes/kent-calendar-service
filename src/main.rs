use std::{borrow::Cow, sync::LazyLock, time::Duration};

use axum::{
    error_handling::HandleErrorLayer,
    extract::{Request, State},
    http::{Method, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    Extension, Router,
};
use http::{header, HeaderValue};
use icalendar::Calendar;
use moka::future::Cache;
use tokio::{net::TcpListener, signal};
use tower::ServiceBuilder;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};
use tracing::info;

mod calendars;
mod kent_schema;
mod sums_pluto_schema;

// #[tokio::main]
// async fn main() -> Result<(), anyhow::Error> {
//     // let calendar = kent_calendar("https://student.kent.ac.uk/events").await?;
//     // calendar.print()?;
//     let calendar = calendars::sums_calendar(
//         "UutZYcRjdM5RzX2mnC8zPR",
//         "Kent SU Calendar",
//         "Hello Kent",
//         |e| format!("https://hellokent.co.uk/events/id/{}", e.id),
//     )
//     .await?;
//     calendar.print()?;
//     Ok(())
// }

git_testament::git_testament!(TESTAMENT);

static VERSION: LazyLock<String> = LazyLock::new(|| git_testament::render_testament!(TESTAMENT));

#[derive(rust_embed::Embed)]
#[folder = "html/"]
struct HtmlFiles;

#[tokio::main()]
async fn main() -> Result<(), anyhow::Error> {
    let _guard = tracing_setup()?;
    run_server().await?;
    Ok(())
}

fn tracing_setup() -> Result<(), anyhow::Error> {
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}

async fn run_server() -> Result<(), anyhow::Error> {
    info!("Starting server version {}", *VERSION);

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0:3779".to_string());
    let mut listenfd = listenfd::ListenFd::from_env();

    let listener = if let Ok(Some(listener)) = listenfd.take_tcp_listener(0) {
        tracing::info!("server listening on socket");
        listener.set_nonblocking(true).unwrap();
        TcpListener::from_std(listener).unwrap()
    } else {
        tracing::info!("server listening on {host}");
        TcpListener::bind(host).await.unwrap()
    };

    let app = Router::new()
        .route(
            "/",
            get(|| async {
                let page = HtmlFiles::get("index.html").unwrap();
                let body = page.data;
                Html(body).into_response()
            }),
        )
        .route("/kent_public_calendar.ics", get(kent_public_calendar))
        .route("/kent_student_calendar.ics", get(kent_student_calendar))
        .route("/kent_union_calendar.ics", get(kent_union_calendar))
        .fallback(not_found_handler);

    let load_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(handle_error))
        .load_shed()
        .concurrency_limit(2 ^ 12)
        .layer(TimeoutLayer::new(Duration::from_secs(60)));

    let app = app.layer(TraceLayer::new_for_http()).layer(load_service);

    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

#[tracing::instrument(skip(req))]
async fn not_found_handler(req: Request) -> Response {
    match *req.method() {
        Method::GET => {
            let page = HtmlFiles::get("404.html").unwrap();
            let body = page.data;
            (StatusCode::NOT_FOUND, Html(body)).into_response()
        }
        _ => {
            let page = HtmlFiles::get("405.html").unwrap();
            let body = page.data;
            (StatusCode::METHOD_NOT_ALLOWED, Html(body)).into_response()
        }
    }
}

static CACHE: LazyLock<Cache<&str, String>> = LazyLock::new(|| {Cache::builder()
    // Time to live (TTL): 60 minutes
    .time_to_live(Duration::from_secs(60 * 60))
    // Create the cache.
    .build()
  });

async fn kent_public_calendar() -> Response {
    let calendar = CACHE
        .try_get_with::<_, anyhow::Error>("kent-pub-cal", async move {
            println!("kent calendar retrieval");
            Ok(format!(
                "{}",
                calendars::kent_calendar("https://www.kent.ac.uk/whats-on").await?
            ))
        })
        .await;
    match calendar {
        Ok(cal) => {
            tracing::info!("kent pub calendar retrieved");
            (
                StatusCode::OK,
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static("text/calendar; charset=utf-8"),
                )],
                cal,
            )
                .into_response()
        }
        Err(e) => {
            tracing::error!("kent pub calendar retrieval failed: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
            )
                .into_response()
        }
    }
}


async fn kent_student_calendar() -> Response {
    let calendar = CACHE
        .try_get_with::<_, anyhow::Error>("kent-student-cal", async move {
            println!("kent calendar retrieval");
            Ok(format!(
                "{}",
                calendars::kent_calendar("https://student.kent.ac.uk/events").await?
            ))
        })
        .await;
    match calendar {
        Ok(cal) => {
            tracing::info!("kent pub calendar retrieved");
            (
                StatusCode::OK,
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static("text/calendar; charset=utf-8"),
                )],
                cal,
            )
                .into_response()
        }
        Err(e) => {
            tracing::error!("kent pub calendar retrieval failed: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
            )
                .into_response()
        }
    }
}



async fn kent_union_calendar() -> Response {
    let calendar = CACHE
        .try_get_with::<_, anyhow::Error>("kent-union-cal", async move {
            println!("kent calendar retrieval");
            Ok(format!(
                "{}",
                calendars::sums_calendar(
                            "UutZYcRjdM5RzX2mnC8zPR",
                            "Kent SU Calendar",
                            "Hello Kent",
                            |e| format!("https://hellokent.co.uk/events/id/{}", e.id),
                        ).await?)
            )
        })
        .await;
    match calendar {
        Ok(cal) => {
            tracing::info!("kent pub calendar retrieved");
            (
                StatusCode::OK,
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static("text/calendar; charset=utf-8"),
                )],
                cal,
            )
                .into_response()
        }
        Err(e) => {
            tracing::error!("kent pub calendar retrieval failed: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
            )
                .into_response()
        }
    }
}


async fn handle_error(error: tower::BoxError) -> impl IntoResponse {
    if error.is::<tower::timeout::error::Elapsed>() {
        return (StatusCode::REQUEST_TIMEOUT, Cow::from("request timed out"));
    }

    if error.is::<tower::load_shed::error::Overloaded>() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Cow::from("service is overloaded, try again later"),
        );
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Cow::from(format!("Unhandled internal error: {error}")),
    )
}

/// This future resolves when either
/// Ctrl+C or SIGTERM is received. It is
/// intended for Axum's `with_graceful_shutdown`
/// function.
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }
}
