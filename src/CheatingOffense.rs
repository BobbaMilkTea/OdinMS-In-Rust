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

pub enum COType {
    FASTATTACK,
    MOVE_MONSTERS,
    TUBI,
    FAST_HP_REGEN,
    FAST_MP_REGEN,
    SAME_DAMAGE,
    ATTACK_WITHOUT_GETTING_HIT,
    HIGH_DAMAGE,
    ATTACK_FARAWAY_MONSTER,
    REGEN_HIGH_HP,
    ITEMVAC,
    SHORT_ITEMVAC,
    USING_FARAWAY_PORTAL,
    FAST_TAKE_DAMAGE,
    FAST_MOVE,
    HIGH_JUMP,
    MISMATCHING_BULLETCOUNT,
    ETC_EXPLOSION,
    FAST_SUMMON_ATTACK,
    ATTACKING_WHILE_DEAD,
}

pub struct CheatingOffense {
    offenseType: COType,
    points: i32,
    validityDuration: i64,
    autobancount: i32,
    enabled: bool,
}

impl CheatingOffense {
    pub fn getPoints(&self) -> i32 {
        self.points
    }

    pub fn getValidityDuration(&self) -> i64 {
        self.validityDuration
    }
    
    pub fn shouldAutoban(&self, count: i32) -> bool {
        if self.autobancount == -1 {
            return false;
        }
        count > self.autobancount
    }
    
    pub fn setEnabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    pub fn isEnabled(&self) -> bool {
        self.enabled
    }

    pub fn from(offenseType: COType) -> CheatingOffense {
        match offenseType {
            COType::FASTATTACK              => CheatingOffenseThree(offenseType, 1, 60000, 500),
            COType::FAST_MP_REGEN           => CheatingOffenseThree(offenseType, 1, 60000, 500),
            COType::HIGH_DAMAGE             => CheatingOffenseTwo(offenseType, 10, 300000),
            COType::ATTACK_FARAWAY_MONSTER  => CheatingOffenseOne(offenseType, 5),
            COType::REGEN_HIGH_HP           => CheatingOffenseOne(offenseType, 50),
            COType::ITEMVAC                 => CheatingOffenseOne(offenseType, 5),
            COType::SHORT_ITEMVAC           => CheatingOffenseOne(offenseType, 2),
            COType::USING_FARAWAY_PORTAL    => CheatingOffenseTwo(offenseType, 30, 300000),
            COType::FAST_TAKE_DAMAGE        => CheatingOffenseThree(offenseType, 1, 60000, 500),
            COType::FAST_MOVE               => CheatingOffenseFour(offenseType, 1, 60000, -1, false),
            COType::HIGH_JUMP               => CheatingOffenseFour(offenseType, 1, 60000, -1, false),
            COType::MISMATCHING_BULLETCOUNT => CheatingOffenseOne(offenseType, 50),
            COType::ETC_EXPLOSION           => CheatingOffenseTwo(offenseType, 50, 300000),
            COType::ATTACKING_WHILE_DEAD    => CheatingOffenseTwo(offenseType, 10, 300000),
            _                               => CheatingOffenseZero(offenseType),
        }
    }
}

fn CheatingOffenseZero(offenseType: COType) -> CheatingOffense {
    CheatingOffenseOne(offenseType, 1)
}

fn CheatingOffenseOne(offenseType: COType, points: i32) -> CheatingOffense {
    CheatingOffenseTwo(offenseType, points, 60000)
}

fn CheatingOffenseTwo(offenseType: COType, points: i32, validityDuration: i64) -> CheatingOffense {
    CheatingOffenseThree(offenseType, points, validityDuration, -1)
}

fn CheatingOffenseThree(offenseType: COType, points: i32, validityDuration: i64, autobancount: i32) -> CheatingOffense {
    CheatingOffenseFour(offenseType, points, validityDuration, autobancount, true)
}

fn CheatingOffenseFour(offenseType: COType, points: i32, validityDuration: i64, autobancount: i32, enabled: bool) -> CheatingOffense {
    CheatingOffense {
        offenseType,
        points,
        validityDuration,
        autobancount,
        enabled,
    }
}
