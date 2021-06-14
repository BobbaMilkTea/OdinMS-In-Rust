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

/*
 * To change this template, choose Tools | Templates
 * and open the template in the editor.
 */

package net.sf.odinms.server.life;

import java.sql.Connection;
import java.sql.PreparedStatement;
import java.sql.ResultSet;
import java.util.HashMap;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;

import net.sf.odinms.database.DatabaseConnection;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

/**
 *
 * @author Matze
 */
public class MapleMonsterInformationProvider {
	public static class DropEntry {
		public DropEntry(int itemId, int chance) {
			this.itemId = itemId;
			this.chance = chance;
		}
		
		public int itemId;
		public int chance;
		public int assignedRangeStart;
		public int assignedRangeLength;
	}
	
	public static final int APPROX_FADE_DELAY = 90;
	
	private static MapleMonsterInformationProvider instance = null;
	
	private Map<Integer,List<DropEntry>> drops = new HashMap<Integer, List<DropEntry>>();
	
	private static final Logger log = LoggerFactory.getLogger(MapleMonsterInformationProvider.class);
	
	private MapleMonsterInformationProvider() {
			// util
	}
	
	public static MapleMonsterInformationProvider getInstance() {
		if (instance == null) instance = new MapleMonsterInformationProvider();
		return instance;
	}
	
	public List<DropEntry> retrieveDropChances(int monsterId) {
		if (drops.containsKey(monsterId)) return drops.get(monsterId);
		List<DropEntry> ret = new LinkedList<DropEntry>();
		try {
			Connection con = DatabaseConnection.getConnection();
			PreparedStatement ps = con.prepareStatement("SELECT itemid, chance FROM monsterdrops " +
				"WHERE monsterid = ?");
			ps.setInt(1, monsterId);
			ResultSet rs = ps.executeQuery();
			while (rs.next()) {
				ret.add(new DropEntry(rs.getInt("itemid"), rs.getInt("chance")));
			}
			rs.close();
			ps.close();
		} catch (Exception e) {
			log.error("lulz", e);
		}
		drops.put(monsterId, ret);
		return ret;
	}
	
	public void clearDrops() {
		drops.clear();
	}
}
