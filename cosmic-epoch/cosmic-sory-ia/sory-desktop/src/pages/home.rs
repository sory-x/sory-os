// SPDX-License-Identifier: GPL-3.0-only

use crate::events::AppEvent;
use cosmic::{Element, widget};

pub fn view() -> Element<'static, AppEvent> {
    widget::text("Accueil").into()
}
