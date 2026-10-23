/*
 * Copyright (c) 2025-2026 Pavel Vasin
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use blacknet_compat::{XDGDirectories, config::Config, mode};
use core::error::Error;
use std::process::ExitCode;

slint::include_modules!();

fn desktop() -> Result<(), Box<dyn Error>> {
    let mode = mode()?;
    let dirs = XDGDirectories::new(mode.subdirectory())?;
    #[expect(unused_variables)]
    let config = Config::load(dirs.config())?;

    let main_window = MainWindow::new()?;
    main_window.on_quit(|| {
        let _ = slint::quit_event_loop();
    });
    main_window.show()?;

    let tray_icon = TrayIcon::new()?;
    let main_window_weak = main_window.as_weak();
    tray_icon.on_toggle(move || {
        let Some(w) = main_window_weak.upgrade() else {
            return;
        };
        let _ = if w.window().is_visible() {
            w.hide()
        } else {
            w.show()
        };
    });
    tray_icon.on_quit(|| {
        let _ = slint::quit_event_loop();
    });
    tray_icon.show()?;

    Ok(slint::run_event_loop()?)
}

pub fn main() -> ExitCode {
    match desktop() {
        Ok(()) => ExitCode::SUCCESS,
        Err(msg) => {
            if ErrorDialog::new()
                .and_then(|ui| {
                    ui.set_message(msg.to_string().into());
                    ui.run()
                })
                .is_err()
            {
                eprintln!("{msg}");
            }
            ExitCode::FAILURE
        }
    }
}
