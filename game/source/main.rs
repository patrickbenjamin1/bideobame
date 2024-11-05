mod core;
mod utils;

use core::app::App;

fn main() {
    // run the app - use pollster to block on the async run function
    pollster::block_on(App::run());
}
