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

package net.sf.odinms.net;

import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.IOException;
import java.util.Properties;

public enum SendPacketOpcode implements WritableIntValueHolder {
	// GENERAL
	PING, // 0x11
	// LOGIN
	LOGIN_STATUS, // 1
	PIN_OPERATION, // 6
	SERVERLIST, // 0xa
	SERVERSTATUS, // 3
	SERVER_IP, // 0xc
	CHARLIST, // 0xb
	CHAR_NAME_RESPONSE, // 0xd
	RELOG_RESPONSE, // 0x16
	ADD_NEW_CHAR_ENTRY, // 0xe
	DELETE_CHAR_RESPONSE, // 0xf

	// CHANNEL
	CHANGE_CHANNEL, // 0x10
	UPDATE_STATS, // 0x1b
	FAME_RESPONSE,
	UPDATE_SKILLS, // 0x1e
	WARP_TO_MAP, // 0x49
	SERVERMESSAGE, // 0x37
	AVATAR_MEGA, // 0x42
	SPAWN_NPC, // 0xb1
	SPAWN_NPC_REQUEST_CONTROLLER, // 0xb3
	SPAWN_MONSTER, // 0x9E
	SPAWN_MONSTER_CONTROL, // 0xA0
	MOVE_MONSTER_RESPONSE, // 0xA3

	CHATTEXT, // 0x67
	SHOW_STATUS_INFO, // 0x21
	SHOW_MESO_GAIN, // 0x22
	SHOW_QUEST_COMPLETION, // 0x29
	WHISPER,
	SPAWN_PLAYER, // 0x64
	//ANNOUNCE_PLAYER_SHOP, // 0x67
	SHOW_SCROLL_EFFECT, // 0x6B
	SHOW_ITEM_GAIN_INCHAT, // 0x92
	KILL_MONSTER, // 0x9f
	DROP_ITEM_FROM_MAPOBJECT, // 0xC1
	FACIAL_EXPRESSION, // 0x85
	MOVE_PLAYER, // 0x7E
	MOVE_MONSTER, // 0xA2
	CLOSE_RANGE_ATTACK, // 0x7F
	RANGED_ATTACK, // 0x80
	MAGIC_ATTACK, // 0x81
	OPEN_NPC_SHOP, // 0xe5
	CONFIRM_SHOP_TRANSACTION, // 0xe6
	OPEN_STORAGE, // 0xe8
	MODIFY_INVENTORY_ITEM, // 0x19
	REMOVE_PLAYER_FROM_MAP, // 0x65
	REMOVE_ITEM_FROM_MAP, // 0xC2
	UPDATE_CHAR_LOOK, // 0x88
	SHOW_FOREIGN_EFFECT, //0x89
	GIVE_FOREIGN_BUFF, //0x8A
	CANCEL_FOREIGN_BUFF, //0x8B
	DAMAGE_PLAYER, // 0x84
	CHAR_INFO, // 0x31
	UPDATE_QUEST_INFO, // 0x97
	GIVE_BUFF, //0x1c
	CANCEL_BUFF, //0x1d
	PLAYER_INTERACTION, // 0xEF
	UPDATE_CHAR_BOX, // 0x69
	NPC_TALK,
	KEYMAP,
	SHOW_MONSTER_HP,
	PARTY_OPERATION,
	UPDATE_PARTYMEMBER_HP,
	MULTICHAT,
	APPLY_MONSTER_STATUS,
	CANCEL_MONSTER_STATUS,
	CLOCK,
	SPAWN_PORTAL,
	SPAWN_DOOR,
	REMOVE_DOOR,
	SPAWN_SPECIAL_MAPOBJECT,
	REMOVE_SPECIAL_MAPOBJECT,
	SUMMON_ATTACK,
	MOVE_SUMMON,
	SPAWN_MIST,
	REMOVE_MIST,
	DAMAGE_SUMMON,
	DAMAGE_MONSTER,
	BUDDYLIST,
	;

	private int code = -2;

	public void setValue(int code) {
		this.code = code;
	}

	@Override
	public int getValue() {
		return code;
	}
	
	public static Properties getDefaultProperties() throws FileNotFoundException, IOException {
		Properties props = new Properties();
		FileInputStream fileInputStream = new FileInputStream(System.getProperty("net.sf.odinms.sendops"));
		props.load(fileInputStream);
		fileInputStream.close();
		return props;
	}

	static {
		try {
			ExternalCodeTableGetter.populateValues(getDefaultProperties(), values());
		} catch (IOException e) {
			throw new RuntimeException("Failed to load recvops", e);
		}
	}
}
