use std::sync::LazyLock;

pub static PORT: LazyLock<u16> = LazyLock::new(|| {
    let port = std::env::var("PORT").unwrap_or_else(|_| panic!("PORT must be set!"));
    port.parse::<u16>()
        .unwrap_or_else(|_| panic!("PORT must be a number!"))
});
