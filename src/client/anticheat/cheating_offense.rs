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

#[derive(PartialEq, Hash)]
pub enum COType {
    FastAttack,
    MoveMonsters,
    Tubi,
    FastHpRegen,
    FastMpRegen,
    SameDamage,
    AttackWithoutGettingHit,
    HighDamage,
    AttackFarAwayMonster,
    RegenHighHp,
    ItemVac,
    ShortItemVac,
    UsingFarAwayPortal,
    FastTakeDamage,
    FastMove,
    HighJump,
    MismatchingBulletCount,
    EtcExplosion,
    FastSummonAttack,
    AttackingWhileDead,
}

#[derive(PartialEq, Hash)]
pub struct CheatingOffense {
    offense_type: COType,
    points: i32,
    validity_duration: i64,
    autobancount: i32,
    enabled: bool,
}

impl CheatingOffense {
    pub fn get_points(&self) -> i32 {
        self.points
    }

    pub fn get_validity_duration(&self) -> i64 {
        self.validity_duration
    }
    
    pub fn should_autoban(&self, count: i32) -> bool {
        if self.autobancount == -1 {
            return false;
        }
        count > self.autobancount
    }
    
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    fn cheating_offense_four(offense_type: COType, points: i32, validity_duration: i64, autobancount: i32, enabled: bool) -> CheatingOffense {
        CheatingOffense {
            offense_type,
            points,
            validity_duration,
            autobancount,
            enabled,
        }
    }

    fn cheating_offense_three(offense_type: COType, points: i32, validity_duration: i64, autobancount: i32) -> CheatingOffense {
        Self::cheating_offense_four(offense_type, points, validity_duration, autobancount, true)
    }

    fn cheating_offense_two(offense_type: COType, points: i32, validity_duration: i64) -> CheatingOffense {
        Self::cheating_offense_three(offense_type, points, validity_duration, -1)
    }
    
    fn cheating_offense_one(offense_type: COType, points: i32) -> CheatingOffense {
        Self::cheating_offense_two(offense_type, points, 60000)
    }
    
    fn cheating_offense_zero(offense_type: COType) -> CheatingOffense {
        Self::cheating_offense_one(offense_type, 1)
    }

    pub fn new(offense_type: COType) -> CheatingOffense {
        match offense_type {
            COType::FastAttack             => Self::cheating_offense_three(offense_type, 1, 60000, 500),
            COType::FastMpRegen            => Self::cheating_offense_three(offense_type, 1, 60000, 500),
            COType::HighDamage             => Self::cheating_offense_two(offense_type, 10, 300000),
            COType::AttackFarAwayMonster   => Self::cheating_offense_one(offense_type, 5),
            COType::RegenHighHp            => Self::cheating_offense_one(offense_type, 50),
            COType::ItemVac                => Self::cheating_offense_one(offense_type, 5),
            COType::ShortItemVac           => Self::cheating_offense_one(offense_type, 2),
            COType::UsingFarAwayPortal     => Self::cheating_offense_two(offense_type, 30, 300000),
            COType::FastTakeDamage         => Self::cheating_offense_three(offense_type, 1, 60000, 500),
            COType::FastMove               => Self::cheating_offense_four(offense_type, 1, 60000, -1, false),
            COType::HighJump               => Self::cheating_offense_four(offense_type, 1, 60000, -1, false),
            COType::MismatchingBulletCount => Self::cheating_offense_one(offense_type, 50),
            COType::EtcExplosion           => Self::cheating_offense_two(offense_type, 50, 300000),
            COType::AttackingWhileDead     => Self::cheating_offense_two(offense_type, 10, 300000),
            _                              => Self::cheating_offense_zero(offense_type),
        }
    }
}
