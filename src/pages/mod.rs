pub mod hello;
pub mod lua;

use std::sync::LazyLock;

pub fn init() {
    LazyLock::force(&lua::PAGES_STRUCTURE);
    LazyLock::force(&lua::PAGES_BY_PATH);
    LazyLock::force(&lua::PAGE_TITLE_BY_PATH);
}
