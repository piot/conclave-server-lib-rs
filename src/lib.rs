/*----------------------------------------------------------------------------------------------------------
 *  Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/conclave-room-rs
 *  Licensed under the MIT License. See LICENSE in the project root for license information.
 *--------------------------------------------------------------------------------------------------------*/
//! The Conclave Logic for a Server
//!

use conclave_room::Room;
use conclave_types::SessionId;
use unique_id_collection::UniqueIdCollection;

pub struct UserSession {
    pub guise_user_id: u64,
    pub name: String,
}

#[derive(Default)]
pub struct Server {
    rooms: UniqueIdCollection<Room>,
    user_sessions: UniqueIdCollection<UserSession>,
}

impl Server {
    pub fn new() -> Self {
        eprintln!("===================\ncreating a server");
        Default::default()
    }

    pub fn create_room(&mut self, _: SessionId) -> &mut Room {
        let room = Room::new();
        let index = self.rooms.add_item(room);
        eprintln!("creating room  {}", index);
        let change_room = self.rooms.get_item_mut(index).expect("index should exist");
        change_room.id = index;
        change_room
    }

    pub fn get_user_session(&self, session_id: SessionId) -> Option<&UserSession> {
        self.user_sessions.get_item(session_id as u8)
    }

    pub fn remove_room(&mut self, id: u8, _: SessionId) {
        eprintln!("removing remove {}", id);
        self.rooms.remove_item(id);
    }

    pub fn login_user(&mut self, guise_session_id: u64) -> SessionId {
        let session = UserSession { guise_user_id: guise_session_id, name: "".to_string() };
        let unique_session_id = self.user_sessions.add_item(session);
        unique_session_id as SessionId
    }
}

#[cfg(test)]
mod tests {
    use crate::Server;

    #[test]
    fn create_room() {
        let mut server = Server::new();
        let guise_session_id = 999;
        let user_session_id = server.login_user(guise_session_id);
        let room = server.create_room(user_session_id);
        assert_eq!(room.id, 1);
        let room2 = server.create_room(user_session_id);
        assert_eq!(room2.id, 2);
    }

    #[test]
    fn remove_room() {
        let mut server = Server::new();
        let guise_session_id = 999;
        let user_session_id = server.login_user(guise_session_id);
        let room_id = server.create_room(user_session_id).id;
        assert_eq!(room_id, 1);
        assert_eq!(server.rooms.len(), 1);
        server.remove_room(room_id, user_session_id);
        assert_eq!(server.rooms.len(), 0);
    }
}
