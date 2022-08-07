use crate::{
    parse::Token,
    utils::{DynString, PatchedMap},
};

pub struct Condition {}

impl Condition {
    pub fn template() -> Token {
        let mut root = PatchedMap::with_capacity(30);
        root.push_back("condition_type", 0_i32);
        root.push_back("static_value_21", 27_i32);
        root.push_back("quantity", -1_i32);
        root.push_back("attribute", -1_i32);
        root.push_back("unit_object", -1_i32);
        root.push_back("next_object", -1_i32);
        root.push_back("object_list", -1_i32);
        root.push_back("source_player", -1_i32);
        root.push_back("technology", -1_i32);
        root.push_back("timer", -1_i32);
        root.push_back("unknown", -1_i32);
        root.push_back("area_x1", -1_i32);
        root.push_back("area_y1", -1_i32);
        root.push_back("area_x2", -1_i32);
        root.push_back("area_y2", -1_i32);
        root.push_back("object_group", -1_i32);
        root.push_back("object_type", -1_i32);
        root.push_back("ai_signal", -1_i32);
        root.push_back("inverted", -1_i32);
        root.push_back("unknown_2", -1_i32);
        root.push_back("variable", -1_i32);
        root.push_back("comparison", -1_i32);
        root.push_back("target_player", -1_i32);
        root.push_back("unit_ai_action", -1_i32);
        root.push_back("unknown_4", -1_i32);
        root.push_back("object_state", -1_i32);
        root.push_back("timer_id", -1_i32);
        root.push_back("victory_timer_type", -1_i32);
        root.push_back("include_changeable_weapon_objects", -1_i32);
        root.push_back("xs_function", DynString::with_capacity(0_u32, ""));

        root.into()
    }
}
