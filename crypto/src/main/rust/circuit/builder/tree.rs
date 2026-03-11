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

use alloc::fmt::{Display, Formatter, Result};
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

pub type NodeId = usize;

const ROOT_ID: NodeId = 0;

struct Node<T> {
    ascendant: Option<NodeId>,
    descendants: Vec<NodeId>,
    item: T,
}

impl<T> Node<T> {
    const fn leaf(ascendant: NodeId, item: T) -> Self {
        Self {
            ascendant: Some(ascendant),
            descendants: Vec::new(),
            item,
        }
    }

    const fn root(item: T) -> Self {
        Self {
            ascendant: None,
            descendants: Vec::new(),
            item,
        }
    }
}

pub struct Tree<T> {
    nodes: Vec<Node<T>>,
}

impl<T> Tree<T> {
    pub fn with_root(item: T) -> (Self, NodeId) {
        (
            Self {
                nodes: vec![Node::root(item)],
            },
            ROOT_ID,
        )
    }

    pub fn ascendant(&self, id: NodeId) -> Option<NodeId> {
        self.nodes.get(id).map(|node| node.ascendant)?
    }

    pub fn descend(&mut self, id: NodeId, item: T) -> Option<NodeId> {
        let new_id = self.nodes.len();
        let ascendant = self.nodes.get_mut(id)?;
        let node = Node::leaf(id, item);
        ascendant.descendants.push(new_id);
        self.nodes.push(node);
        Some(new_id)
    }

    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut T> {
        self.nodes.get_mut(id).map(|node| &mut node.item)
    }
}

impl<T: Display> Display for Tree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut stack = vec![(ROOT_ID, String::new(), String::new())];

        while let Some((id, prefix, postfix)) = stack.pop() {
            let node = self.nodes.get(id).expect("Node");
            writeln!(f, "{prefix}{}", node.item)?;

            if let Some(&last) = node.descendants.last() {
                stack.push((last, postfix.clone() + "└──", postfix.clone() + "   "));
            } else {
                continue;
            }

            for &descendant in node.descendants.iter().rev().skip(1) {
                stack.push((descendant, postfix.clone() + "├──", postfix.clone() + "│  "));
            }
        }

        Ok(())
    }
}
