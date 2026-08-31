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

use crate::{
    UTC,
    error::{Error, Result},
    handle_error, info,
};
use spdlog::{
    Level, LevelFilter, Logger,
    formatter::{Formatter, PatternFormatter, pattern},
    sink::{RotatingFileSink, RotationPolicy, Sink, StdStreamSink},
    terminal_style::StyleMode,
};
use std::{env::VarError, path::Path, str::FromStr, sync::Arc};

#[derive(Clone, Copy)]
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
    pub fn new(strategy: Strategy, dir: &Path) -> Result<Self> {
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
                return Err(Error::NotUnicodeLogLevel);
            }
            Err(VarError::NotPresent) => (),
        }

        #[cfg(feature = "log")]
        {
            let logger = Arc::new(Self::factory(None, filter_level, &sinks)?);
            spdlog::set_default_logger(logger);
            spdlog::init_log_crate_proxy()?;
            spdlog::log_crate_proxy().set_filter(None);
            log::set_max_level(log::LevelFilter::Trace);
        }

        let logger = Self::factory(Some("LogManager"), filter_level, &sinks)?;
        info!(logger, "Initialized logging");
        Ok(Self {
            logger,
            filter_level,
            sinks,
        })
    }

    pub fn logger(&self, name: &'static str) -> Result<Logger> {
        Self::factory(Some(name), self.filter_level, &self.sinks)
    }

    fn factory(
        name: Option<&'static str>,
        filter_level: Level,
        sinks: &Vec<Arc<dyn Sink>>,
    ) -> Result<Logger> {
        let mut builder = Logger::builder();
        if let Some(name) = name {
            builder.name(name);
        }
        Ok(builder
            .level_filter(LevelFilter::MoreSevereEqual(filter_level))
            .flush_level_filter(LevelFilter::MoreSevereEqual(Level::Error))
            .sinks(sinks.iter().cloned())
            .build()?)
    }

    fn console_sink() -> Result<Arc<dyn Sink>> {
        Ok(StdStreamSink::builder()
            .stdout()
            .style_mode(StyleMode::Auto)
            .formatter(Self::formatter())
            .error_handler(handle_error)
            .build()
            .map(Arc::new)?)
    }

    fn file_sink(dir: &Path) -> Result<Arc<dyn Sink>> {
        Ok(RotatingFileSink::builder()
            .base_path(dir.join("debug.log"))
            .rotation_policy(RotationPolicy::FileSize(5000000))
            .max_files(2)
            .rotate_on_open(false)
            .formatter(Self::formatter())
            .error_handler(handle_error)
            .build()
            .map(Arc::new)?)
    }

    fn formatter() -> impl Formatter {
        let pattern = pattern!(
            "[{$utc}] [{logger}] [{^{level}}] {payload}{eol}",
            {$utc} => UTC::new,
        );
        PatternFormatter::new(pattern)
    }
}

impl Drop for LogManager {
    fn drop(&mut self) {
        info!(self.logger, "Shutting down logging");
        for sink in &self.sinks {
            if let Err(err) = sink.flush() {
                handle_error(err)
            }
        }
    }
}
