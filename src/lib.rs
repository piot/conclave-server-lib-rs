/*----------------------------------------------------------------------------------------------------------
 *  Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/conclave-room-rs
 *  Licensed under the MIT License. See LICENSE in the project root for license information.
 *--------------------------------------------------------------------------------------------------------*/
//! The Conclave Logic for a Server
//!

use std::{
    cell::Cell,
    collections::{HashMap, VecDeque},
};

use conclave_room::Room;

pub struct UniqueIdCollection<T> {
    items: HashMap<u8, T>,
    deleted_ids: VecDeque<u8>,
    id_counter: Cell<u8>,
}

impl<T> UniqueIdCollection<T> {
    pub fn new() -> Self {
        UniqueIdCollection {
            items: HashMap::new(),
            deleted_ids: VecDeque::new(),
            id_counter: Cell::new(0),
        }
    }

    pub fn add_item(&mut self, item: T) -> u8 {
        if let Some(reused_id) = self.deleted_ids.pop_front() {
            self.items.insert(reused_id, item);
            reused_id
        } else {
            let new_id = self.id_counter.get();
            self.id_counter.set(new_id + 1);
            self.items.insert(new_id, item);
            new_id
        }
    }

    /// .
    pub fn get_item(&self, id: u8) -> Option<&T> {
        self.items.get(&id)
    }

    fn get_item_mut(&mut self, id: u8) -> Option<&mut T> {
        self.items.get_mut(&id)
    }

    pub fn remove_item(&mut self, id: u8) -> Option<T> {
        if let Some(removed_item) = self.items.remove(&id) {
            self.deleted_ids.push_back(id);
            Some(removed_item)
        } else {
            None
        }
    }
}

#[derive(Default)]
pub struct Server {
    rooms: UniqueIdCollection<Room>,
}

impl Server {
    pub fn new() -> Self {
        eprintln!("===================\ncreating a server");
        Default::default()
    }

    pub fn create_room(&mut self) -> &mut Room {
        let room = Room::new();
        let index = self.rooms.add_item(room);
        let change_room = self.rooms.get_item_mut(index).expect("index should exist");
        change_room.id = index;
        change_room
    }
}

impl<T> Default for UniqueIdCollection<T> {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
mod tests {
    use crate::Server;

    #[test]
    fn create_room() {
        let mut server = Server::new();
        let room = server.create_room();
        assert_eq!(room.id, 0);
        let room2 = server.create_room();
        assert_eq!(room2.id, 1);
    }
}
