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

use blacknet_compat::size::Size;
use blacknet_rpc::v3::peer::PeerInfo;
use core::{
    any::Any,
    cell::{Cell, RefCell},
    cmp::Ordering,
    num::NonZero,
};
use slint::{
    Model, ModelNotify, ModelRc, ModelTracker, SharedString, language::StandardListViewItem,
};
use std::rc::Rc;

pub struct PeerModel {
    info: PeerInfo,
    notify: ModelNotify,
}

impl PeerModel {
    pub fn new(info: PeerInfo) -> Rc<Self> {
        Rc::new(Self {
            info,
            notify: ModelNotify::default(),
        })
    }

    pub fn as_model_rc(self: &Rc<Self>) -> ModelRc<StandardListViewItem> {
        ModelRc::from(self.clone())
    }
}

impl Model for PeerModel {
    type Data = StandardListViewItem;

    fn row_count(&self) -> usize {
        6
    }

    fn row_data(&self, row: usize) -> Option<Self::Data> {
        match row {
            0 => Some(StandardListViewItem::from(SharedString::from(
                &self.info.remote_endpoint,
            ))),
            1 => Some(StandardListViewItem::from(SharedString::from(
                &self.info.agent,
            ))),
            2 => Some(StandardListViewItem::from(SharedString::from(
                self.info.ping.to_string(),
            ))),
            3 => Some(StandardListViewItem::from(SharedString::from(
                if self.info.outgoing { "Out" } else { "In" },
            ))),
            4 => Some(StandardListViewItem::from(SharedString::from(
                Size::from(self.info.total_bytes_read).to_string_lossy(false),
            ))),
            5 => Some(StandardListViewItem::from(SharedString::from(
                Size::from(self.info.total_bytes_written).to_string_lossy(false),
            ))),
            _ => None,
        }
    }

    fn model_tracker(&self) -> &dyn ModelTracker {
        &self.notify
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct PeerTableModel {
    data: RefCell<Vec<Rc<PeerModel>>>,
    sort: Cell<Option<(i32, bool)>>,
    notify: ModelNotify,
}

#[expect(dead_code)]
impl PeerTableModel {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            data: RefCell::new(Vec::new()),
            sort: Cell::new(None),
            notify: ModelNotify::default(),
        })
    }

    pub fn as_model_rc(self: &Rc<Self>) -> ModelRc<ModelRc<StandardListViewItem>> {
        ModelRc::from(self.clone())
    }

    pub fn sort(&self, column: i32, ascending: bool) {
        self.data.borrow_mut().sort_by(|a, b| {
            let (a, b) = (&a.info, &b.info);
            let ord = match column {
                0 => a.remote_endpoint.cmp(&b.remote_endpoint),
                1 => a.agent.cmp(&b.agent),
                2 => a.ping.cmp(&b.ping),
                3 => a.outgoing.cmp(&b.outgoing),
                4 => a.total_bytes_read.cmp(&b.total_bytes_read),
                5 => a.total_bytes_written.cmp(&b.total_bytes_written),
                _ => Ordering::Equal,
            };
            if ascending { ord } else { ord.reverse() }
        });
        self.sort.set(Some((column, ascending)));
        self.notify.reset();
    }

    pub fn set_data(&self, data: Vec<PeerInfo>) {
        *self.data.borrow_mut() = data.into_iter().map(PeerModel::new).collect();
        if let Some((column, ascending)) = self.sort.get() {
            self.sort(column, ascending);
        } else {
            self.notify.reset();
        }
    }

    pub fn peer_id(&self, row: i32) -> Option<NonZero<u64>> {
        self.data
            .borrow()
            .get(row as usize)
            .map(|model| model.info.id)
    }
}

impl Model for PeerTableModel {
    type Data = ModelRc<StandardListViewItem>;

    fn row_count(&self) -> usize {
        self.data.borrow().len()
    }

    fn row_data(&self, row: usize) -> Option<Self::Data> {
        self.data.borrow().get(row).map(PeerModel::as_model_rc)
    }

    fn model_tracker(&self) -> &dyn ModelTracker {
        &self.notify
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
