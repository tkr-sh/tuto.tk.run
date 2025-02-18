pub mod hello;
pub mod lua;

use std::sync::LazyLock;

pub fn init() {
    LazyLock::force(&lua::PAGES);
    LazyLock::force(&lua::PAGES_STRUCTURE);
}
