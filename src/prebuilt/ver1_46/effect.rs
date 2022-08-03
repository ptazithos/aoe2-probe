use crate::{
    parse::Token,
    utils::{map::*, DynString},
};

pub struct Effect {}

impl Effect {
    pub fn template() -> Token {
        let mut root = LinkedHashMap::with_capacity(58);
        root.push_back("effect_type", 0 as i32);
        root.push_back("static_value_46", 53 as i32);
        root.push_back("ai_script_goal", -1 as i32);
        root.push_back("quantity", -1 as i32);
        root.push_back("tribute_list", -1 as i32);
        root.push_back("diplomacy", -1 as i32);
        root.push_back("number_of_units_selected", -1 as i32);
        root.push_back("legacy_location_object_reference", -1 as i32);
        root.push_back("object_list_unit_id", -1 as i32);
        root.push_back("source_player", -1 as i32);
        root.push_back("target_player", -1 as i32);
        root.push_back("technology", -1 as i32);
        root.push_back("string_id", -1 as i32);
        root.push_back("unknown_2", -1 as i32);
        root.push_back("display_time", -1 as i32);
        root.push_back("trigger_id", -1 as i32);
        root.push_back("location_x", -1 as i32);
        root.push_back("location_y", -1 as i32);
        root.push_back("area_x1", -1 as i32);
        root.push_back("area_y1", -1 as i32);
        root.push_back("area_x2", -1 as i32);
        root.push_back("area_y2", -1 as i32);
        root.push_back("object_group", -1 as i32);
        root.push_back("object_type", -1 as i32);
        root.push_back("instruction_panel_position", -1 as i32);
        root.push_back("attack_stance", -1 as i32);
        root.push_back("time_unit", -1 as i32);
        root.push_back("enabled", -1 as i32);
        root.push_back("food", -1 as i32);
        root.push_back("wood", -1 as i32);
        root.push_back("stone", -1 as i32);
        root.push_back("gold", -1 as i32);
        root.push_back("item_id", -1 as i32);
        root.push_back("flash_object", -1 as i32);
        root.push_back("force_research_technology", -1 as i32);
        root.push_back("visibility_state", -1 as i32);
        root.push_back("scroll", -1 as i32);
        root.push_back("operation", -1 as i32);
        root.push_back("object_list_unit_id_2", -1 as i32);
        root.push_back("button_location", -1 as i32);
        root.push_back("ai_signal_value", -1 as i32);
        root.push_back("unknown_3", -1 as i32);
        root.push_back("object_attributes", -1 as i32);
        root.push_back("variable", -1 as i32);
        root.push_back("timer", -1 as i32);
        root.push_back("facet", -1 as i32);
        root.push_back("location_object_reference", -1 as i32);
        root.push_back("play_sound", -1 as i32);
        root.push_back("player_color", -1 as i32);
        root.push_back("unknown_4", -1 as i32);
        root.push_back("color_mood", -1 as i32);
        root.push_back("reset_timer", -1 as i32);
        root.push_back("object_state", -1 as i32);
        root.push_back("action_type", -1 as i32);
        root.push_back("message", DynString::with_capacity(0 as u32, ""));
        root.push_back("sound_name", DynString::with_capacity(0 as u32, ""));
        root.push_back("selected_object_ids", vec![(0 as i32).into()]);
        root.patchs.insert(
            "selected_object_ids".to_string(),
            NumericPatch {
                source: vec!["number_of_units_selected".to_string()],
                dep_type: DepType::Calculate,
                manipulation: Manipulation::Equal,
            },
        );

        root.into()
    }
}
