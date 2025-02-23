use tracing_subscriber::fmt::Subscriber;
use tracing::Level;

// pub fn init() {
//     let subscriber = Subscriber::builder()
//         .with_max_level(Level::INFO)  // Can be set to DEBUG for more verbosity
//         .finish();
//     tracing::subscriber::set_global_default(subscriber)
//         .expect("Failed to set global subscriber.");
// }

pub fn init() {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::INFO)  // Can be set to DEBUG for more verbosity
        .init();
}
