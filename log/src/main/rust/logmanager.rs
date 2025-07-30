/*
 * Copyright (c) 2025 Pavel Vasin
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

use crate::info;
use spdlog::Level;
use spdlog::sink::RotatingFileSink;
use spdlog::sink::RotationPolicy;
use spdlog::sink::Sink;
use spdlog::sink::StdStreamSink;
use spdlog::terminal_style::StyleMode;
use spdlog::{LevelFilter, Logger};
use std::env::VarError;
use std::error::Error;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;

//TODO logger->set_pattern("%+", spdlog::pattern_time_type::utc);

pub enum Strategy {
    Daemon,
    Desktop,
    UnitTest,
}

pub struct LogManager {
    logger: Logger,
    filter_level: Level,
    sinks: Vec<Arc<dyn Sink>>,
}

impl LogManager {
    pub fn new(strategy: Strategy, dir: &Path) -> Result<Self, Box<dyn Error>> {
        let (mut filter_level, sinks) = match strategy {
            Strategy::Daemon => (
                Level::Info,
                vec![Self::console_sink()?, Self::file_sink(dir)?],
            ),
            Strategy::Desktop => (Level::Info, vec![Self::file_sink(dir)?]),
            Strategy::UnitTest => (Level::Error, vec![Self::console_sink()?]),
        };
        match std::env::var("BLACKNET_LOGLEVEL") {
            Ok(val) => filter_level = Level::from_str(&val)?,
            Err(VarError::NotUnicode(_)) => {
                return Err("Not unicode data in environment variable BLACKNET_LOGLEVEL".into());
            }
            Err(VarError::NotPresent) => (),
        }
        let logger = Self::factory("LogManager", filter_level, &sinks)?;
        info!(logger, "Initialized logging");
        Ok(Self {
            logger,
            filter_level,
            sinks,
        })
    }

    pub fn logger(&self, name: &'static str) -> Result<Logger, Box<dyn Error>> {
        Self::factory(name, self.filter_level, &self.sinks)
    }

    fn factory(
        name: &'static str,
        filter_level: Level,
        sinks: &Vec<Arc<dyn Sink>>,
    ) -> Result<Logger, Box<dyn Error>> {
        Ok(Logger::builder()
            .name(name)
            .level_filter(LevelFilter::MoreSevereEqual(filter_level))
            .flush_level_filter(LevelFilter::MoreSevereEqual(Level::Error))
            .sinks(sinks.iter().map(Clone::clone))
            .build()?)
    }

    fn console_sink() -> Result<Arc<dyn Sink>, Box<dyn Error>> {
        Ok(StdStreamSink::builder()
            .stdout()
            .style_mode(StyleMode::Auto)
            .build()
            .map(|sink| Arc::new(sink) as Arc<dyn Sink>)?)
    }

    fn file_sink(dir: &Path) -> Result<Arc<dyn Sink>, Box<dyn Error>> {
        Ok(RotatingFileSink::builder()
            .base_path(dir.join("debug.log"))
            .rotation_policy(RotationPolicy::FileSize(5000000))
            .max_files(2)
            .rotate_on_open(false)
            .build()
            .map(|sink| Arc::new(sink) as Arc<dyn Sink>)?)
    }
}

impl Drop for LogManager {
    fn drop(&mut self) {
        info!(self.logger, "Shutting down logging");
        self.sinks.clear()
    }
}
