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

use std::collections::LinkedList;

pub struct Equip {
    item: Item,
    upgrade_slots: i8,
    level: i8,
    _str: i16,
    dex: i16,
    int: i16,
    luk: i16,
    hp: i16,
    mp: i16,
    watk: i16,
    matk: i16,
    wdef: i16,
    mdef: i16,
    acc: i16,
    avoid: i16,
    hands: i16,
    speed: i16,
    jump: i16,
}

impl Equip {
    pub fn new(id: i32, position: i8) -> Equip {
        Equip {
            item: Item::new(id, position 1),
            upgrade_slots: 0,
            level: 0,
            _str: 0,
            dex: 0,
            int: 0,
            luk: 0,
            hp: 0,
            mp: 0,
            watk: 0,
            matk: 0,
            wdef: 0,
            mdef: 0,
            acc: 0,
            avoid: 0,
            hands: 0,
            speed: 0,
            jump: 0,
        }
    }
}

impl Clone for Equip {
    pub fn clone(&self) -> Self {
        Equip {
            item: Item::new(id, position 1),
            upgrade_slots: self.upgrade_slots,
            level: self.level,
            _str: self._str,
            dex: self.dex,
            int: self.int,
            luk: self.luk,
            hp: self.hp,
            mp: self.mp,
            watk: self.watk,
            matk: self.matk,
            wdef: self.wdef,
            mdef: self.mdef,
            acc: self.acc,
            avoid: self.avoid,
            hands: self.hands,
            speed: self.speed,
            jump: self.jump,
        }
    }
}

impl IEquip for Equip {
    pub fn get_type(&self) -> i8 {
        self.IItem.equip
    }

    pub fn get_upgrade_slots(&self) -> i8 {
        self.upgrade_slots
    }

    pub fn get_str(&self) -> i16 {
        self._str
    }
}
