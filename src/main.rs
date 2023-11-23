use tpulse::afk::{AFKWatcher, Settings};
use std::thread;
use dotenv::dotenv;
fn main() {
    dotenv().ok();
    env_logger::init();

    let afk_settings = Settings::new(5000, 500);
    let afk_watcher = AFKWatcher::new(&afk_settings, "AFK Watcher");
    afk_watcher.run();

    let afk_watch = thread::spawn(move || {
        afk_watcher.run()
    });
    afk_watch.join().unwrap()
}

