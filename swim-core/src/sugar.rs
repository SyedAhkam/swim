use rand::Rng;

pub const SERVER_LAUNCH_MESSAGES: [&'static str; 12] = [
    "Making a big splash at",
    "Leaving a lasting impact at",
    "Speeding ahead into",
    "Making waves of progress at",
    "Leaping forward into",
    "Surfing the waves of success at",
    "Riding the digital tide over",
    "Diving deep into the network at",
    "Going with the flow at",
    "Making waves of success at",
    "Sailing smoothly through cyberspace to",
    "Swimming effortlessly into",
];

// Non-exhaustive
pub const ROUTE_EMOJI_MAPPINGS: [(&'static str, &'static str); 19] = [
    ("home", "đ "),
    ("favicon.ico", "đŧī¸"),
    ("login", "đ"),
    ("logout", "đ"),
    ("search", "đ"),
    ("profile", "đ¤"),
    ("settings", "âī¸"),
    ("help", "â"),
    ("about", "âšī¸"),
    ("contact", "đ"),
    ("docs", "đ"),
    ("tutorials", "đĨ"),
    ("news", "đ°"),
    ("events", "đ"),
    ("products", "đī¸"),
    ("services", "đŧ"),
    ("api", "đĄ"),
    ("graphql", "đ"),
    ("hello", "đ"),
];

#[inline]
pub fn choose_server_launch_message() -> &'static str {
    SERVER_LAUNCH_MESSAGES[rand::thread_rng().gen_range(0..SERVER_LAUNCH_MESSAGES.len())]
}

// FIXME: create a better search algorithm
pub fn get_route_emoji(route: &str) -> &'static str {
    for (route_name, emoji) in ROUTE_EMOJI_MAPPINGS.iter() {
        if route.contains(route_name) {
            return emoji;
        }
    }

    "đ"
}
