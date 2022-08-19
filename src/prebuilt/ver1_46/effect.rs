use std::rc::Rc;

use crate::{
    parse::Token,
    utils::{map::*, DynString},
};

pub struct Effect;

impl Effect {
    pub fn template() -> Token {
        let mut root = PatchedMap::with_capacity(58);
        root.push_back("effect_type", 0_i32);
        root.push_back("static_value_46", 53_i32);
        root.push_back("ai_script_goal", -1_i32);
        root.push_back("quantity", -1_i16);
        root.push_back("class", -1_i16);
        root.push_back("tribute_list", -1_i32);
        root.push_back("diplomacy", -1_i32);
        root.push_back("number_of_units_selected", -1_i32);
        root.push_back("legacy_location_object_reference", -1_i32);
        root.push_back("object_list_unit_id", -1_i32);
        root.push_back("source_player", -1_i32);
        root.push_back("target_player", -1_i32);
        root.push_back("technology", -1_i32);
        root.push_back("string_id", -1_i32);
        root.push_back("unknown_2", -1_i32);
        root.push_back("display_time", -1_i32);
        root.push_back("trigger_id", -1_i32);
        root.push_back("location_x", -1_i32);
        root.push_back("location_y", -1_i32);
        root.push_back("area_x1", -1_i32);
        root.push_back("area_y1", -1_i32);
        root.push_back("area_x2", -1_i32);
        root.push_back("area_y2", -1_i32);
        root.push_back("object_group", -1_i32);
        root.push_back("object_type", -1_i32);
        root.push_back("instruction_panel_position", -1_i32);
        root.push_back("attack_stance", -1_i32);
        root.push_back("time_unit", -1_i32);
        root.push_back("enabled", -1_i32);
        root.push_back("food", -1_i32);
        root.push_back("wood", -1_i32);
        root.push_back("stone", -1_i32);
        root.push_back("gold", -1_i32);
        root.push_back("item_id", -1_i32);
        root.push_back("flash_object", -1_i32);
        root.push_back("force_research_technology", -1_i32);
        root.push_back("visibility_state", -1_i32);
        root.push_back("scroll", -1_i32);
        root.push_back("operation", -1_i32);
        root.push_back("object_list_unit_id_2", -1_i32);
        root.push_back("button_location", -1_i32);
        root.push_back("ai_signal_value", -1_i32);
        root.push_back("unknown_3", -1_i32);
        root.push_back("object_attributes", -1_i32);
        root.push_back("variable", -1_i32);
        root.push_back("timer", -1_i32);
        root.push_back("facet", -1_i32);
        root.push_back("location_object_reference", -1_i32);
        root.push_back("play_sound", -1_i32);
        root.push_back("player_color", -1_i32);
        root.push_back("unknown_4", -1_i32);
        root.push_back("color_mood", -1_i32);
        root.push_back("reset_timer", -1_i32);
        root.push_back("object_state", -1_i32);
        root.push_back("action_type", -1_i32);
        root.push_back("message", DynString::with_capacity(0_u32, ""));
        root.push_back("sound_name", DynString::with_capacity(0_u32, ""));
        root.push_back("selected_object_ids", vec![0_i32.into()]);

        root.patches.insert(
            "selected_object_ids".to_string(),
            Rc::new(|map: &mut PatchedMap, template: &mut Token| {
                if map.contains("number_of_units_selected") {
                    let count = *map["number_of_units_selected"].try_i32();
                    let unit = template.try_vec()[0].clone();
                    let vec = template.try_mut_vec();
                    vec.clear();

                    for _ in 0..count {
                        vec.push(unit.clone());
                    }
                }
            }),
        );

        root.into()
    }
    pub fn default() -> Token {
        let mut root = PatchedMap::with_capacity(58);
        root.push_back("effect_type", 0_i32);
        root.push_back("static_value_46", 53_i32);
        root.push_back("ai_script_goal", -1_i32);
        root.push_back("quantity", -1_i32);
        root.push_back("tribute_list", -1_i32);
        root.push_back("diplomacy", -1_i32);
        root.push_back("number_of_units_selected", -1_i32);
        root.push_back("legacy_location_object_reference", -1_i32);
        root.push_back("object_list_unit_id", -1_i32);
        root.push_back("source_player", -1_i32);
        root.push_back("target_player", -1_i32);
        root.push_back("technology", -1_i32);
        root.push_back("string_id", -1_i32);
        root.push_back("unknown_2", -1_i32);
        root.push_back("display_time", -1_i32);
        root.push_back("trigger_id", -1_i32);
        root.push_back("location_x", -1_i32);
        root.push_back("location_y", -1_i32);
        root.push_back("area_x1", -1_i32);
        root.push_back("area_y1", -1_i32);
        root.push_back("area_x2", -1_i32);
        root.push_back("area_y2", -1_i32);
        root.push_back("object_group", -1_i32);
        root.push_back("object_type", -1_i32);
        root.push_back("instruction_panel_position", -1_i32);
        root.push_back("attack_stance", -1_i32);
        root.push_back("time_unit", -1_i32);
        root.push_back("enabled", -1_i32);
        root.push_back("food", -1_i32);
        root.push_back("wood", -1_i32);
        root.push_back("stone", -1_i32);
        root.push_back("gold", -1_i32);
        root.push_back("item_id", -1_i32);
        root.push_back("flash_object", -1_i32);
        root.push_back("force_research_technology", -1_i32);
        root.push_back("visibility_state", -1_i32);
        root.push_back("scroll", -1_i32);
        root.push_back("operation", -1_i32);
        root.push_back("object_list_unit_id_2", -1_i32);
        root.push_back("button_location", -1_i32);
        root.push_back("ai_signal_value", -1_i32);
        root.push_back("unknown_3", -1_i32);
        root.push_back("object_attributes", -1_i32);
        root.push_back("variable", -1_i32);
        root.push_back("timer", -1_i32);
        root.push_back("facet", -1_i32);
        root.push_back("location_object_reference", -1_i32);
        root.push_back("play_sound", -1_i32);
        root.push_back("player_color", -1_i32);
        root.push_back("unknown_4", -1_i32);
        root.push_back("color_mood", -1_i32);
        root.push_back("reset_timer", -1_i32);
        root.push_back("object_state", -1_i32);
        root.push_back("action_type", -1_i32);
        root.push_back("message", DynString::with_capacity(0_u32, ""));
        root.push_back("sound_name", DynString::with_capacity(0_u32, ""));
        root.push_back("selected_object_ids", vec![]);

        root.into()
    }
}
