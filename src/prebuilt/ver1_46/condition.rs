use crate::{
    parse::Token,
    utils::{DynString, LinkedHashMap},
};

pub struct Condition {}

impl Condition {
    pub fn template() -> Token {
        let mut root = LinkedHashMap::with_capacity(30);
        root.push_back("condition_type", 0 as i32);
        root.push_back("static_value_21", 27 as i32);
        root.push_back("quantity", -1 as i32);
        root.push_back("attribute", -1 as i32);
        root.push_back("unit_object", -1 as i32);
        root.push_back("next_object", -1 as i32);
        root.push_back("object_list", -1 as i32);
        root.push_back("source_player", -1 as i32);
        root.push_back("technology", -1 as i32);
        root.push_back("timer", -1 as i32);
        root.push_back("unknown", -1 as i32);
        root.push_back("area_x1", -1 as i32);
        root.push_back("area_y1", -1 as i32);
        root.push_back("area_x2", -1 as i32);
        root.push_back("area_y2", -1 as i32);
        root.push_back("object_group", -1 as i32);
        root.push_back("object_type", -1 as i32);
        root.push_back("ai_signal", -1 as i32);
        root.push_back("inverted", -1 as i32);
        root.push_back("unknown_2", -1 as i32);
        root.push_back("variable", -1 as i32);
        root.push_back("comparison", -1 as i32);
        root.push_back("target_player", -1 as i32);
        root.push_back("unit_ai_action", -1 as i32);
        root.push_back("unknown_4", -1 as i32);
        root.push_back("object_state", -1 as i32);
        root.push_back("timer_id", -1 as i32);
        root.push_back("victory_timer_type", -1 as i32);
        root.push_back("include_changeable_weapon_objects", -1 as i32);
        root.push_back("xs_function", DynString::new(0 as u32, ""));

        root.into()
    }
}
