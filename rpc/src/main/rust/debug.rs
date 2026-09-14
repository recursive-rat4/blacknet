/*
 * Copyright (c) 2026 Pavel Vasin
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

use axum::{Json, Router, extract::State, routing::get};
use core::time::Duration;
use serde::{Deserialize, Serialize};
use tokio::runtime::{Handle, RuntimeMetrics};

#[derive(Deserialize, Serialize)]
pub struct Worker {
    park_count: u64,
    park_unpark_count: u64,
    total_busy_duration: Duration,
}

impl Worker {
    fn new(metrics: &RuntimeMetrics, worker: usize) -> Self {
        Self {
            park_count: metrics.worker_park_count(worker),
            park_unpark_count: metrics.worker_park_unpark_count(worker),
            total_busy_duration: metrics.worker_total_busy_duration(worker),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Metrics {
    queue_depth: usize,
    num_alive_tasks: usize,
    workers: Vec<Worker>,
}

impl Metrics {
    fn new(metrics: &RuntimeMetrics) -> Self {
        let workers = (0..metrics.num_workers())
            .map(|worker| Worker::new(metrics, worker))
            .collect();
        Self {
            queue_depth: metrics.global_queue_depth(),
            num_alive_tasks: metrics.num_alive_tasks(),
            workers,
        }
    }
}

async fn metrics(State(runtime): State<Handle>) -> Json<Metrics> {
    Json(Metrics::new(&runtime.metrics()))
}

pub fn routes(runtime: Handle) -> Router<()> {
    Router::new()
        .route("/api/debug/tokio/metrics", get(metrics))
        .with_state(runtime)
}
