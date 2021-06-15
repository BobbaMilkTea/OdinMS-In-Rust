/*
	This file is part of the OdinMS Maple Story Server
    Copyright (C) 2008 Patrick Huy <patrick.huy@frz.cc> 
					   Matthias Butz <matze@odinms.de>
					   Jan Christian Meyer <vimes@odinms.de>

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as
    published by the Free Software Foundation version 3 as published by
    the Free Software Foundation. You may not use, modify or distribute
    this program under any other version of the GNU Affero General Public
    License.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use std::collections;
use std::collections::VecDeque;
use std::collections::LinkedHashMap;
use std::collections::LinkedList;
use std::collections::HashMap;

use crate::database::database_connection::DatabaseConnection;
use crate::tools::maple_packet_creator::MaplePacketCreator;

pub enum BuddyOperation {
    added,
    deleted,
}

pub enum BuddyAddResult {
    buddylist_full,
    already_on_list,
    ok,
}

pub struct BuddyList {
    buddies: HashMap<i32, BuddylistEntry>,
    capacity: i32,
    pending_requests: VecDeque<CharacterNameAndId>,
}

impl BuddyList {
    pub fn contains(&self, character_id: i32) -> bool {
        self.buddies.contains_key(character_id)
    }

    pub fn containsVisible(&self, character_id: i32) -> bool {
        match self.buddies.get(character_id) {
            None    => false,
            Some(x) => x.isVisible(),
        }
    }

    pub fn getCapacity(&self) -> i32 {
        self.capacity
    }

    pub fn setCapacity(&self, capacity: i32) {
        self.capacity = capacity;
    }

    pub fn get(&self, character_id: i32) -> BuddylistEntry {
        self.buddies.get(character_id)
    }

    pub fn get(&self, character_name: String) -> Option<BuddylistEntry> {
        String lower_case = character_name.to_lowercase();
        for ble in self.buddies.values() {
            if ble.get_name().to_lowercase() == lower_case {
                return Some(ble);
            }
        }
        None
    }

    pub fn put(&self, entry: BuddylistEntry) {
        self.buddies.insert(entry.get_character_id(), entry);
    }

    pub fn remove(&self, &character_id: i32) {
        self.buddies.remove(&character_id)
    }

    pub fn get_buddies(&self) -> hash_map::Values {
        self.buddies.values()
    }

    pub fn is_full(&self) -> bool {
        self.buddies.len() >= self.capacity
    }

    pub fn get_buddy_ids() -> Vec<i32> {
        let mut buddy_ids = Vec::new();
        for ble in self.buddies.values() {
            buddy_ids.push(ble.get_character_id());
        }
        buddy_ids
    }

    pub fn load_from_db(&self, character_id: i32) {
        
    }

    pub fn new(capacity: i32) -> BuddyList {
        BuddyList {
            buddies: HashMap::new(),
            capacity,
            pending_requests: LinkedList::new(),
        }
    }
}
