use lazy_static::lazy_static;
use linked_hash_map::LinkedHashMap;

lazy_static! {
    pub static ref ATTR_MAP: LinkedHashMap<i32, &'static str> = {
        let mut mapping = LinkedHashMap::new();

        mapping.insert(-1, "None");
        mapping.insert(0, "HIT_POINTS");
        mapping.insert(1, "LINE_OF_SIGHT");
        mapping.insert(2, "GARRISON_CAPACITY");
        mapping.insert(3, "UNIT_SIZE_X");
        mapping.insert(4, "UNIT_SIZE_Y");
        mapping.insert(5, "MOVEMENT_SPEED");
        mapping.insert(6, "ROTATION_SPEED");
        mapping.insert(8, "ARMOR");
        mapping.insert(9, "ATTACK");
        mapping.insert(10, "ATTACK_RELOAD_TIME");
        mapping.insert(11, "ACCURACY_PERCENT");
        mapping.insert(12, "MAX_RANGE");
        mapping.insert(13, "WORK_RATE");
        mapping.insert(14, "CARRY_CAPACITY");
        mapping.insert(15, "BASE_ARMOR");
        mapping.insert(16, "PROJECTILE_UNIT");
        mapping.insert(17, "BUILDING_ICON_OVERRIDE");
        mapping.insert(18, "TERRAIN_DEFENSE_BONUS");
        mapping.insert(19, "PROJECTILE_SMART_MODE");
        mapping.insert(20, "MINIMUM_RANGE");
        mapping.insert(21, "AMOUNT_OF_1ST_RESOURCE_STORAGE");
        mapping.insert(22, "BLAST_WIDTH");
        mapping.insert(23, "SEARCH_RADIUS");
        mapping.insert(24, "BONUS_DAMAGE_RESISTANCE");
        mapping.insert(25, "AMOUNT_OF_2ND_RESOURCE_STORAGE");
        mapping.insert(26, "AMOUNT_OF_3RD_RESOURCE_STORAGE");
        mapping.insert(27, "FOG_VISIBILITY");
        mapping.insert(28, "OCCLUSION_MODE");
        mapping.insert(29, "OCCLUSION_MODE");
        mapping.insert(30, "GARRISON_TYPE");

        mapping.insert(32, "UNIT_SIZE_Z");

        mapping.insert(40, "HERO_STATUS");
        mapping.insert(41, "FRAME_DELAY");
        mapping.insert(42, "TRAIN_LOCATION");
        mapping.insert(43, "TRAIN_BUTTON");
        mapping.insert(44, "BLAST_ATTACK_LEVEL");
        mapping.insert(45, "BLAST_DEFENSE_LEVEL");
        mapping.insert(46, "SHOWN_ATTACK");
        mapping.insert(47, "SHOWN_RANGE");
        mapping.insert(48, "SHOWN_MELEE_ARMOR");
        mapping.insert(49, "SHOWN_PIERCE_ARMOR");
        mapping.insert(50, "OBJECT_NAME_ID");
        mapping.insert(51, "SHORT_DESCRIPTION_ID");

        mapping.insert(53, "TERRAIN_RESTRICTION_ID");
        mapping.insert(54, "UNIT_TRAIT");

        mapping.insert(56, "TRAIT_PIECE");
        mapping.insert(57, "DEAD_UNIT_ID");
        mapping.insert(58, "HOTKEY_ID");
        mapping.insert(59, "MAXIMUM_CHARGE");
        mapping.insert(60, "RECHARGE_RATE");
        mapping.insert(61, "CHARGE_EVENT");
        mapping.insert(62, "CHARGE_TYPE");
        mapping.insert(63, "COMBAT_ABILITY");
        mapping.insert(64, "ATTACK_DISPERSION");
        mapping.insert(65, "SECONDARY_PROJECTILE_UNIT");
        mapping.insert(66, "BLOOD_UNIT");
        mapping.insert(67, "PROJECTILE_HIT_MODE");
        mapping.insert(68, "PROJECTILE_VANISH_MODE");
        mapping.insert(69, "PROJECTILE_ARC");
        mapping.insert(100, "RESOURCE_COSTS");
        mapping.insert(101, "TRAIN_TIME");
        mapping.insert(102, "TOTAL_MISSILES");
        mapping.insert(103, "FOOD_COSTS");
        mapping.insert(104, "WOOD_COSTS");
        mapping.insert(105, "GOLD_COSTS");
        mapping.insert(106, "STONE_COSTS");
        mapping.insert(107, "MAX_TOTAL_MISSILES");
        mapping.insert(108, "GARRISON_HEAL_RATE");
        mapping.insert(109, "REGENERATION_RATE");
        mapping.insert(110, "POPULATION");

        mapping
    };
}
