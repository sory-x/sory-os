// SPDX-License-Identifier: GPL-3.0-only

mod app;
mod backend;
mod components;
mod events;
mod icons;
mod models;
mod pages;
mod platform;
mod state;
mod theme;
mod ui;

fn main() -> cosmic::iced::Result {
    env_logger::init();
    app::run()
}
