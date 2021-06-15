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

use crate::client::anticheat::cheating_offense::CheatingOffense;
use crate::client::maple_character::MapleCharacter;
use std::time::{SystemTime, UNIX_EPOCH};
use std::hash::{Hash, Hasher};

pub struct CheatingOffenseEntry {
    offense: CheatingOffense,
    count: i32,
    chrfor: MapleCharacter,
    first_offense: i64,
    last_offense: Option<i64>,
    param: Option<String>,
    dbid: Option<i32>,
}

impl CheatingOffenseEntry {
    pub fn get_offense(&self) -> &CheatingOffense {
        &(self.offense)
    }
    
    pub fn get_count(&self) -> i32 {
        self.count
    }

    pub fn get_chrfor(&self) -> &MapleCharacter {
        &(self.chrfor)
    }

    pub fn increment_count(&mut self) {
        self.count += 1;
        let time_now: i64 = SystemTime::now().duration_since(UNIX_EPOCH).expect("error").as_millis() as i64;
        self.last_offense = Some(time_now);
    }

    pub fn is_expired(&self) -> bool {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("error").as_millis() as i64;
        let validity_duration = self.offense.get_validity_duration();
        match self.last_offense {
            None    => true,
            Some(x) => x < (current_time - validity_duration),
        }
    }

    pub fn get_points(&self) -> i32 {
        self.count * self.offense.get_points()
    }

    pub fn get_param(&self) -> Option<&String> {
        self.param.as_ref()
    }

    pub fn set_param(&mut self, param: String) {
        self.param = Some(param);
    }

    pub fn get_last_offense_time(&self) -> Option<i64> {
        self.last_offense
    }

    pub fn get_db_id(&self) -> Option<i32> {
        self.dbid
    }

    pub fn set_db_id(&mut self, dbid: i32) {
        self.dbid = Some(dbid);
    }

    pub fn new(offense: CheatingOffense, chrfor: MapleCharacter) -> CheatingOffenseEntry {
        let time_now = SystemTime::now().duration_since(UNIX_EPOCH).expect("error").as_millis() as i64;
        CheatingOffenseEntry {
            offense,
            count:          0,
            chrfor,
            first_offense:   time_now,
            dbid:           None,
            last_offense:    None,
            param:          None,
        }
    }
}

impl Hash for CheatingOffenseEntry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let prime = 31;
        prime.hash(state);
        self.chrfor.get_id().hash(state);
        self.offense.hash(state);
        self.first_offense.hash(state);
    }
}

impl PartialEq for CheatingOffenseEntry {
    fn eq(&self, other: &Self) -> bool {
        if self == other                                        { return true; }
        if self.chrfor.get_id() != other.chrfor.get_id()          { return false; }
        if !(self.offense == other.offense)                     { return false; }
        if other.first_offense != self.first_offense              { return false; }
        true
    }
}
