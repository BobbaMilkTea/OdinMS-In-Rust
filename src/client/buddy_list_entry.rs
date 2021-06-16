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

use std::any;

pub struct BuddylistEntry {
    name: String,
    cid: i32,
    channel: i32,
    visible: bool,
}

impl BuddylistEntry {
    pub fn get_channel(&self) -> &i32 {
        &(self.channel)
    }

    pub fn set_channel(&mut self, channel: i32) {
        self.channel = channel;
    }

    pub fn is_online(&self) -> bool {
        self.channel >= 0
    }

    pub fn set_offline(&mut self) {
        self.channel = -1
    }

    pub fn get_name(&self) -> &String {
        &(self.name)
    }

    pub fn get_character_id(&self) -> i32 {
        self.cid
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn new(name: String, cid: i32, channel: i32, visible: bool) -> BuddylistEntry {
        BuddylistEntry {
            name,
            cid,
            channel,
            visible,
        }
    }
}

impl Hash for CheatingOffenseEntry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let prime = 31;
        prime.hash(state);
        self.cid.hash(state);
    }
}

impl PartialEq for CheatingOffenseEntry {
    fn eq(&self, other: &Self) -> bool {
        if self == other                                 { return true; }
        if self.cid != other.get_character_id()          { return false; }
        true
    }
}
