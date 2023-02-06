#[macro_export]
macro_rules! format_branch {
    ($tag:expr, $value:expr) => {
        format!(
            "{} {} {}",
            Paint::blue("=>").bold(),
            Paint::cyan($tag).bold(),
            Paint::green($value).underline()
        )
    };
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
                .map(|r| format_branch!("route", format!("{} {}", r, get_route_emoji(r))))
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
                .join("\n  "))
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
