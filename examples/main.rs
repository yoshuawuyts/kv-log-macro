use kv_log_macro::{info, log};
use log::Level;

fn main() {
    info!("hello");
    info!("hello",);
    info!("hello {}", "cats");
    // info!("hello {}", "cats",);
    // info!("hello {}", "cats", {
    //     cat_1: "chashu",
    //     cat_2: "nori",
    //     cat_count: 2,
    // });

    log!(Level::Info, "hello {}", "cats");
}
