// FIXME: refactor this
#[macro_export]
macro_rules! format_branch {
    ($tag:expr, $value:expr $(,$other:expr)?) => {{
        #[allow(unused_mut)]
        let mut res = format!(
            "{} {} {}",
            Paint::blue("=>").bold(),
            Paint::cyan($tag).bold(),
            Paint::green($value).underline(),
        );

        $(
            use std::fmt::Write;
            write!(&mut res, "{}", $other).ok();
        )?

        res
    }};
}

#[macro_export]
macro_rules! project_details {
    ($project:expr, $addr:expr) => {{
        use yansi::Paint;

        println!(
            "🏊‍ {} {} {}\n    {}\n    {}\n    {}",
            Paint::yellow(format!("{:?}", $project)).bold(),
            Paint::cyan(format!(
                "[v{}]",
                std::env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "unknown".to_string()) // determined at runtime
            )),
            (if (cfg!(debug_assertions)) {
                Paint::red("[DEBUG]")
            } else {
                Paint::green("[RELEASE]")
            }),
            format_branch!("host", $addr.ip()),
            format_branch!("port", $addr.port()),
            format_branch!("pwd", std::env::current_dir().unwrap_or_else(|_| ".".into()).display())
        );
    }};
}

#[macro_export]
macro_rules! app_details {
    ($app:expr) => {{
        use crate::sugar::get_route_emoji;
        use yansi::Paint;

        let __routes = $app
            .routes()
            .iter()
            .map(|r| r.path.to_owned())
            .collect::<Vec<_>>();

        println!(
            "🌊 {} {} {} {}\n    {}",
            Paint::cyan("Mounting"),
            Paint::green(format!("{:?}", $app)).bold(),
            Paint::cyan("at"),
            Paint::green(format!("{}", $app.mount())).bold(),
            (__routes
                .iter()
                .map(|r| format_branch!("route", r, format!(" {}", get_route_emoji(r))))
                .collect::<Vec<_>>()
                .join("\n    "))
        );
    }};
}

#[macro_export]
macro_rules! middleware_listing {
    ($middlewares:expr) => {{
        use yansi::Paint;

        println!(
            "🛡️  {} \n    {}",
            Paint::cyan("Applying Middleware"),
            ($middlewares
                .iter()
                .map(|m| format_branch!(format!("{:?}", m), ""))
                .collect::<Vec<_>>()
                .join("\n    "))
        );
    }};
}

#[macro_export]
macro_rules! launch_info {
    ($addr:expr) => {{
        use crate::sugar::choose_server_launch_message;
        use yansi::Paint;

        println!(
            "⛵️ {} {}",
            Paint::cyan(choose_server_launch_message()),
            Paint::green(format!("http://{}", $addr)).underline(),
        );
    }};
}

#[macro_export]
macro_rules! request_info {
    ($req:expr) => {{
        use yansi::Paint;

        println!(
            "{} {} {} {}",
            Paint::cyan(format!("{}", $req.method())).underline(),
            Paint::green(format!("{}", $req.uri())),
            (match $req.method() {
                &hyper::Method::POST => Paint::blue(format!(
                    "{} bytes",
                    $req.body().size_hint().upper().unwrap_or(0)
                )),
                _ => Paint::blue("".to_string()),
            }),
            Paint::black(format!(
                "[{}]",
                chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")
            ))
            .dimmed(),
        );
    }};
}

#[macro_export]
macro_rules! response_info {
    ($res:expr) => {{
        use super::format_branch;
        use yansi::Paint;

        println!(
            " {}\n {}",
            format_branch!("status", format!("{}", $res.status())),
            format_branch!(
                "size",
                format!("{:?} bytes", $res.body().size_hint().upper().unwrap_or(0))
            ),
        );
    }};
}
