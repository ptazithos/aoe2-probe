use std::fmt;

use crate::data::PrefixString;

pub mod ver1_46;

pub trait VersioIsh {
    fn file_header<'a>(&self) -> &(dyn FileHeaderIsh + 'a);
    fn file_header_mut<'a>(&mut self) -> &mut (dyn FileHeaderIsh + 'a);

    fn data_header<'a>(&self) -> &(dyn DataHeaderIsh + 'a);
    fn data_header_mut<'a>(&mut self) -> &mut (dyn DataHeaderIsh + 'a);

    fn messages<'a>(&self) -> &(dyn MessagesIsh + 'a);
    fn messages_mut<'a>(&mut self) -> &mut (dyn MessagesIsh + 'a);

    fn cinematics<'a>(&self) -> &(dyn CinematicsIsh + 'a);
    fn cinematics_mut<'a>(&mut self) -> &mut (dyn CinematicsIsh + 'a);

    fn background_image<'a>(&self) -> &(dyn BackgroundImageIsh + 'a);
    fn background_image_mut<'a>(&mut self) -> &mut (dyn BackgroundImageIsh + 'a);

    fn player_data_two<'a>(&self) -> &(dyn PlayerDataTwoIsh + 'a);
    fn player_data_two_mut<'a>(&mut self) -> &mut (dyn PlayerDataTwoIsh + 'a);

    fn global_victory<'a>(&self) -> &(dyn GlobalVictoryIsh + 'a);
    fn global_victory_mut<'a>(&mut self) -> &mut (dyn GlobalVictoryIsh + 'a);

    fn diplomacy<'a>(&self) -> &(dyn DiplomacyIsh + 'a);
    fn diplomacy_mut<'a>(&mut self) -> &mut (dyn DiplomacyIsh + 'a);

    fn options<'a>(&self) -> &(dyn OptionsIsh + 'a);
    fn options_mut<'a>(&mut self) -> &mut (dyn OptionsIsh + 'a);

    fn map<'a>(&self) -> &(dyn MapIsh + 'a);
    fn map_mut<'a>(&mut self) -> &mut (dyn MapIsh + 'a);

    fn units<'a>(&self) -> &(dyn UnitsIsh + 'a);
    fn units_mut<'a>(&mut self) -> &mut (dyn UnitsIsh + 'a);

    fn triggers<'a>(&self) -> &(dyn TriggersIsh + 'a);
    fn triggers_mut<'a>(&mut self) -> &mut (dyn TriggersIsh + 'a);

    fn files<'a>(&self) -> &(dyn FilesIsh + 'a);
    fn files_mut<'a>(&mut self) -> &mut (dyn FilesIsh + 'a);

    fn to_buffer(&self, buffer: &mut Vec<u8>);
}

impl fmt::Debug for dyn VersioIsh {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("VersioIsh")
            .field("file header", &self.file_header())
            .field("data header", &self.data_header())
            .field("messages", &self.messages())
            .field("cinematics", &self.cinematics())
            .field("background image", &self.background_image())
            .field("player data two", &self.player_data_two())
            .field("global victory", &self.global_victory())
            .field("diplomacy", &self.diplomacy())
            .field("options", &self.options())
            .field("map", &self.map())
            .field("units", &self.units())
            .field("triggers", &self.triggers())
            .field("files", &self.files())
            .finish()
    }
}

pub trait FileHeaderIsh {
    fn version(&self) -> &String;
    fn set_version(&mut self, version: String);
    fn header_length(&self) -> u32;
    fn set_header_length(&mut self, header_length: u32);
    fn savable(&self) -> i32;
    fn set_savable(&mut self, savable: i32);
    fn timestamp(&self) -> u32;
    fn set_timestamp(&mut self, timestamp: u32);
    fn instructions(&self) -> &PrefixString<u32>;
    fn set_instructions(&mut self, instructions: &str);
    fn player_count(&self) -> u32;
    fn set_player_count(&mut self, player_count: u32);
    fn unknown_data(&self) -> &[u32; 9];
    fn set_unknown_data(&mut self, unknown_data: [u32; 9]);
    fn creator_name(&self) -> &PrefixString<u32>;
    fn set_creator_name(&mut self, creator_name: &str);
    fn trigger_count(&self) -> u32;
    fn set_trigger_count(&mut self, trigger_count: u32);

    fn to_buffer(&self, buffer: &mut Vec<u8>);
}

impl fmt::Debug for dyn FileHeaderIsh {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("FileHeaderIsh")
            .field("version", &self.version())
            .field("header length", &self.header_length())
            .field("timestamp", &self.timestamp())
            .field("instructions", &self.instructions())
            .field("player count", &self.player_count())
            .field("unknown data", &self.unknown_data())
            .field("creator name", &self.creator_name())
            .field("trigger count", &self.trigger_count())
            .finish()
    }
}

pub trait DataHeaderIsh {
    fn next_unit_id_to_place(&self) -> u32;
    fn set_next_unit_id_to_place(&mut self, next_unit_id_to_place: u32);
    fn version(&self) -> f32;
    fn set_version(&mut self, version: f32);
    fn player_names(&self) -> &[String; 16];
    fn player_names_mut(&mut self) -> &mut [String; 16];
    fn string_table_player_names(&self) -> &[u32; 16];
    fn string_table_player_names_mut(&mut self) -> &mut [u32; 16];
    fn player_data_one(&self) -> &[Box<dyn PlayerDataOneIsh>; 16];
    fn player_data_one_mut(&mut self) -> &mut [Box<dyn PlayerDataOneIsh>; 16];
    fn conquest_mode(&self) -> u8;
    fn set_conquest_mode(&mut self, conquest_mode: u8);
    fn mission_items_counter(&self) -> u16;
    fn set_mission_items_counter(&mut self, mission_items_counter: u16);
    fn mission_available(&self) -> u16;
    fn set_mission_available(&mut self, mission_available: u16);
    fn mission_timeline(&self) -> f32;
    fn set_mission_timeline(&mut self, mission_timeline: f32);
    fn filename(&self) -> &PrefixString<u16>;
    fn set_filename(&mut self, filename: &str);

    fn to_buffer(&self, buffer: &mut Vec<u8>);
}

impl fmt::Debug for dyn DataHeaderIsh {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("DataHeaderIsh")
            .field("next unit id to place", &self.next_unit_id_to_place())
            .field("version", &self.version())
            .field("player names", &self.player_names())
            .field(
                "string table player names",
                &self.string_table_player_names(),
            )
            .field("player data one", &self.player_data_one())
            .field("conquest mode", &self.conquest_mode())
            .field("mission items counter", &self.mission_items_counter())
            .field("mission available", &self.mission_available())
            .field("mission timeline", &self.mission_timeline())
            .field("filename", &self.filename())
            .finish()
    }
}

pub trait PlayerDataOneIsh {
    fn actvie(&self) -> u32;
    fn set_active(&mut self, active: u32);
    fn human(&self) -> u32;
    fn set_human(&mut self, human: u32);
    fn civilization(&self) -> u32;
    fn set_civilization(&mut self, civiliztion: u32);
    fn architecture_set(&self) -> u32;
    fn set_architecture_set(&mut self, architecture_set: u32);
    fn cty_mode(&self) -> u32;
    fn set_cty_mode(&mut self, cty_mode: u32);

    fn to_buffer(&self, buffer: &mut Vec<u8>);
}

impl fmt::Debug for dyn PlayerDataOneIsh {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("PlayerDataOneIsh")
            .field("actvie", &self.actvie())
            .field("human", &self.human())
            .field("civilization", &self.civilization())
            .field("architecture set", &self.architecture_set())
            .field("cty mode", &self.cty_mode())
            .finish()
    }
}

pub trait MessagesIsh {
    fn instructions(&self) -> u32;
    fn set_instructions(&mut self, instructions: u32);
    fn hints(&self) -> u32;
    fn set_hints(&mut self, hints: u32);
    fn victory(&self) -> u32;
    fn set_victory(&mut self, victory: u32);
    fn loss(&self) -> u32;
    fn set_loss(&mut self, lost: u32);
    fn history(&self) -> u32;
    fn set_history(&mut self, history: u32);
    fn scouts(&self) -> u32;
    fn set_scouts(&mut self, scouts: u32);
    fn ascii_instructions(&self) -> &PrefixString<u16>;
    fn set_ascii_instructions(&mut self, ascii_instructions: &str);
    fn ascii_hints(&self) -> &PrefixString<u16>;
    fn set_ascii_hints(&mut self, ascii_hints: &str);
    fn ascii_victory(&self) -> &PrefixString<u16>;
    fn set_ascii_victory(&mut self, ascii_victory: &str);
    fn ascii_loss(&self) -> &PrefixString<u16>;
    fn set_ascii_loss(&mut self, ascii_loss: &str);
    fn ascii_history(&self) -> &PrefixString<u16>;
    fn set_ascii_history(&mut self, ascii_history: &str);
    fn ascii_scouts(&self) -> &PrefixString<u16>;
    fn set_ascii_scouts(&mut self, ascii_scouts: &str);

    fn to_buffer(&self, buffer: &mut Vec<u8>);
}

impl fmt::Debug for dyn MessagesIsh {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("MessagesIsh")
            .field("instructions", &self.instructions())
            .field("hints", &self.hints())
            .field("victory", &self.victory())
            .field("loss", &self.loss())
            .field("history", &self.history())
            .field("scouts", &self.scouts())
            .field("ascii instructions", &self.ascii_instructions())
            .field("ascii hints", &self.ascii_hints())
            .field("ascii victory", &self.ascii_victory())
            .field("ascii loss", &self.ascii_loss())
            .field("ascii history", &self.ascii_history())
            .field("ascii scouts", &self.ascii_scouts())
            .finish()
    }
}

pub trait CinematicsIsh {
    fn pregame_cinematic_filename(&self) -> &PrefixString<u16>;
    fn set_pregame_cinematic_filename(&mut self, pregame_cinematic_filename: &str);
    fn victory_cinematice_filename(&self) -> &PrefixString<u16>;
    fn set_victory_cinematice_filename(&mut self, victory_cinematice_filename: &str);
    fn loss_cinematic_filename(&self) -> &PrefixString<u16>;
    fn set_loss_cinematic_filename(&mut self, loss_cinematic_filename: &str);

    fn to_buffer(&self, buffer: &mut Vec<u8>);
}

impl fmt::Debug for dyn CinematicsIsh {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("CinematicsIsh")
            .field(
                "pregame cinematic filename",
                &self.pregame_cinematic_filename(),
            )
            .field(
                "victory cinematice filename",
                &self.victory_cinematice_filename(),
            )
            .field("loss cinematic filename", &self.loss_cinematic_filename())
            .finish()
    }
}

pub trait BackgroundImageIsh {
    fn filename(&self) -> &PrefixString<u16>;
    fn set_filename(&mut self, filename: &str);
    fn picture_version(&self) -> u32;
    fn set_picture_version(&mut self, picture_version: u32);
    fn bitmap_width(&self) -> u32;
    fn set_bitmap_width(&mut self, width: u32);
    fn bitmap_height(&self) -> u32;
    fn set_bitmap_height(&mut self, height: u32);
    fn picture_orientation(&self) -> i16;
    fn set_picture_orientation(&mut self, orientation: i16);
    fn bitmap_info<'a>(&self) -> &(dyn BitMapInfoIsh + 'a);
    fn bitmap_info_mut<'a>(&mut self) -> &mut (dyn BitMapInfoIsh + 'a);

    fn to_buffer(&self, buffer: &mut Vec<u8>);
}

impl fmt::Debug for dyn BackgroundImageIsh {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("BackgroundImageIsh")
            .field("filename", &self.filename())
            .field("picture version", &self.picture_version())
            .field("bitmap width", &self.bitmap_width())
            .field("bitmap height", &self.bitmap_height())
            .field("picture orientation", &self.picture_orientation())
            .field("bitmap info", &self.bitmap_info())
            .finish()
    }
}

pub trait BitMapInfoIsh {
    fn size(&self) -> i32;
    fn set_size(&mut self, size: i32);
    fn width(&self) -> u32;
    fn set_width(&mut self, width: u32);
    fn height(&self) -> u32;
    fn set_height(&mut self, height: u32);
    fn planes(&self) -> i16;
    fn set_planes(&mut self, planes: i16);
    fn bit_count(&self) -> i16;
    fn set_bit_count(&mut self, bit_count: i16);
    fn compression(&self) -> u32;
    fn set_compression(&mut self, compression: u32);
    fn image_size(&self) -> u32;
    fn set_image_szie(&mut self, image_size: u32);
    fn x_pels(&self) -> u32;
    fn set_x_pels(&mut self, x_pels: u32);
    fn y_pels(&self) -> u32;
    fn set_y_pels(&mut self, y_pels: u32);
    fn number_of_colors_used(&self) -> u32;
    fn set_number_of_colors_used(&mut self, num_of_colors: u32);
    fn important_colors(&self) -> u32;
    fn set_important_colors(&mut self, colors: u32);
    fn colors_used(&self) -> &Vec<u32>;
    fn set_colors_used(&mut self, color_used: Vec<u32>);
    fn image(&self) -> &Vec<u8>;
    fn set_image(&mut self, image: Vec<u8>);

    fn to_buffer(&self, buffer: &mut Vec<u8>);
}

impl fmt::Debug for dyn BitMapInfoIsh {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("BitMapInfoIsh")
            .field("size", &self.size())
            .field("width", &self.width())
            .field("height", &self.height())
            .field("planes", &self.planes())
            .field("bit_count", &self.bit_count())
            .field("compression", &self.compression())
            .field("image_size", &self.image_size())
            .field("x_pels", &self.x_pels())
            .field("y_pels", &self.y_pels())
            .field("number_of_colors_used", &self.number_of_colors_used())
            .field("important_colors", &self.important_colors())
            .field("colors_used", &self.colors_used())
            .field("image", &self.image())
            .finish()
    }
}

pub trait PlayerDataTwoIsh {
    fn unknown(&self) -> &[PrefixString<u16>; 32];
    fn unknown_mut(&mut self) -> &mut [PrefixString<u16>; 32];
    fn ai_names(&self) -> &[PrefixString<u16>; 16];
    fn ai_names_mut(&mut self) -> &mut [PrefixString<u16>; 16];
    fn ai_files(&self) -> &[Box<dyn AIFileIsh>; 16];
    fn ai_files_mut(&mut self) -> &mut [Box<dyn AIFileIsh>; 16];
    fn ai_types(&self) -> &[u8; 16];
    fn ai_types_mut(&mut self) -> &mut [u8; 16];
    fn separator(&self) -> u32;
    fn set_separator(&mut self, separator: u32);
    fn resources(&self) -> &[Box<dyn ResourcesIsh>; 16];
    fn resources_mut(&mut self) -> &mut [Box<dyn ResourcesIsh>; 16];

    fn to_buffer(&self, buffer: &mut Vec<u8>);
}

impl fmt::Debug for dyn PlayerDataTwoIsh {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("PlayerDataTwoIsh")
            .field("unknown", &self.unknown())
            .field("ai names", &self.ai_names())
            .field("ai files", &self.ai_files())
            .field("ai types", &self.ai_types())
            .field("separator", &self.separator())
            .field("resources", &self.resources())
            .finish()
    }
}

pub trait AIFileIsh {
    fn unknown_1(&self) -> u32;
    fn set_unknown_1(&mut self, unknown: u32);
    fn unknown_2(&self) -> u32;
    fn set_unknown_2(&mut self, unknown: u32);
    fn ai_per_file_text(&self) -> &PrefixString<u32>;
    fn set_ai_per_file_text(&mut self, text: &str);

    fn to_buffer(&self, buffer: &mut Vec<u8>);
}

impl fmt::Debug for dyn AIFileIsh {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("AIFileIsh")
            .field("unknown_1", &self.unknown_1())
            .field("unknown_2", &self.unknown_2())
            .field("ai_per_file_text", &self.ai_per_file_text())
            .finish()
    }
}

pub trait ResourcesIsh {
    fn gold(&self) -> u32;
    fn set_good(&mut self, gold: u32);
    fn wood(&self) -> u32;
    fn set_wood(&mut self, wood: u32);
    fn food(&self) -> u32;
    fn set_food(&mut self, food: u32);
    fn stone(&self) -> u32;
    fn set_stone(&mut self, stone: u32);
    fn ore_x(&self) -> u32;
    fn set_ore_x(&mut self, ore_x: u32);
    fn trade_goods(&self) -> u32;
    fn set_trade_goods(&mut self, trade_goods: u32);
    fn player_color(&self) -> u32;
    fn set_player_color(&mut self, player_color: u32);

    fn to_buffer(&self, buffer: &mut Vec<u8>);
}

impl fmt::Debug for dyn ResourcesIsh {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("ResourcesIsh")
            .field("gold", &self.gold())
            .field("wood", &self.wood())
            .field("food", &self.food())
            .field("ore_x", &self.ore_x())
            .field("trade_goods", &self.trade_goods())
            .field("player_color", &self.player_color())
            .finish()
    }
}

pub trait GlobalVictoryIsh {
    fn separator(&self) -> u32;
    fn set_separator(&mut self, separator: u32);
    fn conquest_required(&self) -> u32;
    fn set_conquest_required(&mut self, conquest: u32);
    fn ruins(&self) -> u32;
    fn set_ruins(&mut self, ruins: u32);
    fn artifacts(&self) -> u32;
    fn set_artifacts(&mut self, artifacts: u32);
    fn discovery(&self) -> u32;
    fn set_discovery(&mut self, discovery: u32);
    fn explored_percent_of_map_required(&self) -> u32;
    fn set_explored_percent_of_map_required(&mut self, percent: u32);
    fn gold_requred(&self) -> u32;
    fn set_gold_required(&mut self, gold: u32);
    fn all_custom_conditions_required(&self) -> u32;
    fn set_all_custom_conditions_required(&mut self, conditions: u32);
    fn mode(&self) -> u32;
    fn set_mode(&mut self, mode: u32);
    fn required_score_for_score_victory(&self) -> u32;
    fn set_required_score_for_score_victory(&mut self, score: u32);
    fn time_for_timed_gamed_in_10ths_of_a_year(&self) -> u32;
    fn set_time_for_timed_gamed_in_10(&mut self, time: u32);

    fn to_buffer(&self, buffer: &mut Vec<u8>);
}

impl fmt::Debug for dyn GlobalVictoryIsh {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("GlobalVictoryIsh")
            .field("separator", &self.separator())
            .field("conquest required", &self.conquest_required())
            .field("ruins", &self.ruins())
            .field("artifacts", &self.artifacts())
            .field("discovery", &self.discovery())
            .field(
                "explored percent of map required",
                &self.explored_percent_of_map_required(),
            )
            .field("gold requred", &self.gold_requred())
            .field(
                "all custom conditions required",
                &self.all_custom_conditions_required(),
            )
            .field("mode", &self.mode())
            .field(
                "required score for score victory",
                &self.required_score_for_score_victory(),
            )
            .field(
                "time for timed gamed in 10ths of a year",
                &self.time_for_timed_gamed_in_10ths_of_a_year(),
            )
            .finish()
    }
}

pub trait DiplomacyIsh {
    fn diplomacies(&self) -> &[Box<dyn PlayerDiplomacyIsh>; 16];
    fn diplomacies_mut(&mut self) -> &mut [Box<dyn PlayerDiplomacyIsh>; 16];
    fn individual_victories(&self) -> &Vec<u8>;
    fn individual_victories_mut(&mut self) -> &mut Vec<u8>;
    fn seperator(&self) -> u32;
    fn set_separator(&mut self, seperator: u32);
    fn per_player_allied_victory(&self) -> &[u32; 16];
    fn per_player_allied_victory_mut(&mut self) -> &mut [u32; 16];
    fn lock_teams(&self) -> u8;
    fn set_lock_teams(&mut self, lock_team: u8);
    fn allow_players_choose_teams(&self) -> u8;
    fn set_allow_players_choose_teams(&mut self, choose_team: u8);
    fn random_start_points(&self) -> u8;
    fn set_random_start_points(&mut self, random_start_points: u8);
    fn max_number_of_teams(&self) -> u8;
    fn set_max_number_of_teams(&mut self, max_of_teams: u8);

    fn to_buffer(&self, buffer: &mut Vec<u8>);
}

impl fmt::Debug for dyn DiplomacyIsh {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("DiplomacyIsh")
            .field("diplomacies", &self.diplomacies())
            .field("individual victories", &self.individual_victories())
            .field("seperator", &self.seperator())
            .field(
                "per player allied victory",
                &self.per_player_allied_victory(),
            )
            .field("lock teams", &self.lock_teams())
            .field(
                "allow players choose teams",
                &self.allow_players_choose_teams(),
            )
            .field("random start points", &self.random_start_points())
            .field("max number of teams", &self.max_number_of_teams())
            .finish()
    }
}

pub trait PlayerDiplomacyIsh {
    fn stance(&self) -> &[u32; 16];
    fn stance_mut(&mut self) -> &mut [u32; 16];

    fn to_buffer(&self, buffer: &mut Vec<u8>);
}

impl fmt::Debug for dyn PlayerDiplomacyIsh {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("PlayerDiplomacyIsh")
            .field("stance", &self.stance())
            .finish()
    }
}

pub trait OptionsIsh {
    fn per_player_number_of_disabled_techs(&self) -> &[u32; 16];
    fn per_player_number_of_disabled_techs_mut(&mut self) -> &mut [u32; 16];
    fn disabled_tech_ids_player(&self) -> &[Vec<u32>; 16];
    fn disabled_tech_ids_player_mut(&mut self) -> &mut [Vec<u32>; 16];
    fn per_player_number_of_disabled_units(&self) -> &[u32; 16];
    fn per_player_number_of_disabled_units_mut(&mut self) -> &mut [u32; 16];
    fn disabled_unit_ids_player(&self) -> &[Vec<u32>; 16];
    fn disabled_unit_ids_player_mut(&mut self) -> &mut [Vec<u32>; 16];
    fn per_player_number_of_disabled_buildings(&self) -> &[u32; 16];
    fn per_player_number_of_disabled_buildings_mut(&mut self) -> &mut [u32; 16];
    fn disabled_building_ids_player(&self) -> &[Vec<u32>; 16];
    fn disabled_building_ids_player_mut(&mut self) -> &mut [Vec<u32>; 16];
    fn combat_mode(&self) -> u32;
    fn set_combat_mode(&mut self, combat_mode: u32);
    fn naval_mode(&self) -> u32;
    fn set_naval_mode(&mut self, naval_mode: u32);
    fn all_techs(&self) -> u32;
    fn set_all_techs(&mut self, all_techs: u32);
    fn per_player_starting_age(&self) -> &[u32; 16];
    fn per_player_starting_age_mut(&mut self) -> &mut [u32; 16];
    fn unknown_1(&self) -> &Vec<u8>;
    fn unknown_1_mut(&mut self) -> &mut Vec<u8>;
    fn per_player_base_priority(&self) -> &[u8; 8];
    fn per_player_base_priority_mut(&mut self) -> &mut [u8; 8];
    fn unknown_2(&self) -> &Vec<u8>;
    fn unknown_2_mut(&mut self) -> &mut Vec<u8>;
    fn number_of_triggers(&self) -> u32;
    fn set_number_of_triggers(&mut self, number: u32);

    fn to_buffer(&self, buffer: &mut Vec<u8>);
}

impl fmt::Debug for dyn OptionsIsh {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("OptionsIsh")
            .field(
                "per_player_number_of_disabled_techs",
                &self.per_player_number_of_disabled_techs(),
            )
            .field("disabled_tech_ids_player", &self.disabled_tech_ids_player())
            .field(
                "per_player_number_of_disabled_units",
                &self.per_player_number_of_disabled_units(),
            )
            .field("disabled_unit_ids_player", &self.disabled_unit_ids_player())
            .field(
                "per_player_number_of_disabled_buildings",
                &self.per_player_number_of_disabled_buildings(),
            )
            .field(
                "disabled_building_ids_player",
                &self.disabled_building_ids_player(),
            )
            .field("combat_mode", &self.combat_mode())
            .field("naval_mode", &self.naval_mode())
            .field("all_techs", &self.all_techs())
            .field("per_player_starting_age", &self.per_player_starting_age())
            .field("unknown_1", &self.unknown_1())
            .field("per_player_base_priority", &self.per_player_base_priority())
            .field("unknown_2", &self.unknown_2())
            .field("number_of_triggers", &self.number_of_triggers())
            .finish()
    }
}

pub trait MapIsh {
    fn string_starter_1(&self) -> &Vec<u8>;
    fn string_starter_1_mut(&mut self) -> &mut Vec<u8>;
    fn water_definition(&self) -> &PrefixString<u16>;
    fn set_water_definition(&mut self, water_definition: &str);
    fn string_starter_2(&self) -> &Vec<u8>;
    fn string_starter_2_mut(&mut self) -> &mut Vec<u8>;
    fn map_color_mood(&self) -> &PrefixString<u16>;
    fn set_map_color_mood(&mut self, map_color_mood: &str);
    fn string_starter_3(&self) -> &Vec<u8>;
    fn string_starter_3_mut(&mut self) -> &mut Vec<u8>;
    fn script_name(&self) -> &PrefixString<u16>;
    fn set_script_name(&mut self, script_name: &str);
    fn collide_and_correct(&self) -> u8;
    fn set_collide_and_correct(&mut self, collide_and_correct: u8);
    fn villager_force_drop(&self) -> u8;
    fn set_villager_force_drop(&mut self, villager_force_drop: u8);
    fn unknown(&self) -> &Vec<u8>;
    fn unknown_mut(&mut self) -> &mut Vec<u8>;
    fn lock_coop_alliances(&self) -> u8;
    fn set_lock_coop_alliances(&mut self, lock_coop_alliances: u8);
    fn ai_map_type(&self) -> i32;
    fn set_ai_map_type(&mut self, ai_map_type: i32);
    fn per_player_population_cap(&self) -> &[u32; 16];
    fn per_player_population_cap_mut(&mut self) -> &mut [u32; 16];
    fn secondary_game_mode(&self) -> u32;
    fn set_secondary_game_mode(&mut self, seconday_game_mode: u32);
    fn unknown_2(&self) -> &Vec<u8>;
    fn unknown_2_mut(&mut self) -> &mut Vec<u8>;
    fn unknown_3(&self) -> &Vec<u8>;
    fn unknown_3_mut(&mut self) -> &mut Vec<u8>;
    fn no_waves_on_shore(&self) -> i8;
    fn set_no_wave_on_shore(&mut self, flag: i8);
    fn map_width(&self) -> i32;
    fn set_map_width(&mut self, width: i32);
    fn map_height(&self) -> i32;
    fn set_map_height(&mut self, height: i32);
    fn terrain(&self) -> &Vec<Box<dyn TerrainTileIsh>>;
    fn terrain_mut(&mut self) -> &mut Vec<Box<dyn TerrainTileIsh>>;

    fn to_buffer(&self, buffer: &mut Vec<u8>);
}

impl fmt::Debug for dyn MapIsh {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("MapIsh")
            .field("string_starter_1", &self.string_starter_1())
            .field("water_definition", &self.water_definition())
            .field("string_starter_2", &self.string_starter_2())
            .field("map_color_mood", &self.map_color_mood())
            .field("string_starter_3", &self.string_starter_3())
            .field("script_name", &self.script_name())
            .field("collide_and_correct", &self.collide_and_correct())
            .field("villager_force_drop", &self.villager_force_drop())
            .field("unknown", &self.unknown())
            .field("lock_coop_alliances", &self.lock_coop_alliances())
            .field("ai_map_type", &self.ai_map_type())
            .field(
                "per_player_population_cap",
                &self.per_player_population_cap(),
            )
            .field("secondary_game_mode", &self.secondary_game_mode())
            .field("unknown_2", &self.unknown_2())
            .field("unknown_3", &self.unknown_3())
            .field("no_waves_on_shore", &self.no_waves_on_shore())
            .field("map_width", &self.map_width())
            .field("map_heigth", &self.map_height())
            .field("terrain", &self.terrain())
            .finish()
    }
}

pub trait TerrainTileIsh {
    fn id(&self) -> u8;
    fn set_id(&mut self, id: u8);
    fn elevation(&self) -> u8;
    fn set_elevation(&mut self, elevation: u8);
    fn unused(&self) -> &Vec<u8>;
    fn unused_mut(&mut self) -> &mut Vec<u8>;
    fn layer(&self) -> i16;
    fn set_layer(&mut self, layer: i16);

    fn to_buffer(&self, buffer: &mut Vec<u8>);
}

impl fmt::Debug for dyn TerrainTileIsh {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("TerrainTileIsh")
            .field("id", &self.id())
            .field("elevation", &self.elevation())
            .field("unused", &self.unused())
            .field("layer", &self.layer())
            .finish()
    }
}

pub trait UnitsIsh {
    fn number_of_unit_sections(&self) -> u32;
    fn set_number_of_unit_sections(&mut self, number: u32);
    fn player_data_three(&self) -> &[Box<dyn PlayerDataThreeIsh>; 8];
    fn player_data_three_mut(&mut self) -> &mut [Box<dyn PlayerDataThreeIsh>; 8];
    fn number_of_players(&self) -> u32;
    fn set_number_of_players(&mut self, number: u32);
    fn player_data_four(&self) -> &[Box<dyn PlayerDataFourIsh>; 8];
    fn player_data_four_mut(&mut self) -> &mut [Box<dyn PlayerDataFourIsh>; 8];
    fn player_units(&self) -> &Vec<Box<dyn PlayerUnitsIsh>>;
    fn player_units_mut(&mut self) -> &mut Vec<Box<dyn PlayerUnitsIsh>>;

    fn to_buffer(&self, buffer: &mut Vec<u8>);
}

impl fmt::Debug for dyn UnitsIsh {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("UnitsIsh")
            .field("number of unit sections", &self.number_of_unit_sections())
            .field("player data three", &self.player_data_three())
            .field("number of players", &self.number_of_players())
            .field("player data four", &self.player_data_four())
            .field("player units", &self.player_units())
            .finish()
    }
}

pub trait PlayerDataThreeIsh {
    fn food_duplicate(&self) -> f32;
    fn set_food_duplicate(&mut self, food_duplicate: f32);
    fn wood_duplicate(&self) -> f32;
    fn set_wood_duplicate(&mut self, wood_duplicate: f32);
    fn gold_duplicate(&self) -> f32;
    fn set_gold_duplicate(&mut self, gold_duplicate: f32);
    fn stone_duplicate(&self) -> f32;
    fn set_stone_duplicate(&mut self, stone_duplicate: f32);
    fn ore_x_duplicate(&self) -> f32;
    fn set_ore_x_duplicate(&mut self, ore_x_duplicate: f32);
    fn trade_goods_duplicate(&self) -> f32;
    fn set_trade_goods_deplicate(&mut self, trade_good_duplicate: f32);
    fn population_limit(&self) -> f32;
    fn set_population_limit(&mut self, population: f32);

    fn to_buffer(&self, buffer: &mut Vec<u8>);
}

impl fmt::Debug for dyn PlayerDataThreeIsh {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("PlayerDataThreeIsh")
            .field("food_duplicate", &self.food_duplicate())
            .field("wood_duplicate", &self.wood_duplicate())
            .field("gold_duplicategold_duplicate", &self.gold_duplicate())
            .field("stone_duplicate", &self.stone_duplicate())
            .field("ore_x_duplicate", &self.ore_x_duplicate())
            .field("trade_goods_duplicate", &self.trade_goods_duplicate())
            .field("population_limit", &self.population_limit())
            .finish()
    }
}

pub trait PlayerDataFourIsh {
    fn constant_name(&self) -> &PrefixString<u16>;
    fn set_constant_name(&mut self, name: &str);
    fn editor_camera_x(&self) -> f32;
    fn set_editor_camera_x(&mut self, x: f32);
    fn editor_camera_y(&self) -> f32;
    fn set_editor_camera_y(&mut self, y: f32);
    fn initial_camera_x(&self) -> i16;
    fn set_initial_camera_x(&mut self, x: i16);
    fn initial_camera_y(&self) -> i16;
    fn aok_allied_victory(&self) -> u8;
    fn set_aok_allied_victory(&mut self, victory: u8);
    fn player_count_for_diplomacy(&self) -> u16;
    fn set_player_count_for_diplomacy(&mut self, count: u16);
    fn diplomacy_for_interaction(&self) -> &Vec<u8>;
    fn diplomacy_for_interaction_mut(&mut self) -> &mut Vec<u8>;
    fn diplomacy_for_ai_system(&self) -> &[u32; 9];
    fn diplomacy_for_ai_system_mut(&mut self) -> &mut [u32; 9];
    fn color(&self) -> u32;
    fn set_color(&mut self, color: u32);
    fn victory_version(&self) -> f32;
    fn set_victory_version(&mut self, version: f32);
    fn unknown(&self) -> u16;
    fn set_unknown(&mut self, unknown: u16);
    fn unknown_2(&self) -> &Vec<u8>;
    fn unknown_2_mut(&mut self) -> &mut Vec<u8>;
    fn unknown_structure_grand_theft_empires(&self) -> &Vec<u8>;
    fn unknown_structure_grand_theft_empires_mut(&mut self) -> &mut Vec<u8>;
    fn unknown_3(&self) -> &Vec<u8>;
    fn unknown_3_mut(&mut self) -> &mut Vec<u8>;
    fn unknown_4(&self) -> &[u8; 7];
    fn unknown_4_mut(&mut self) -> &mut [u8; 7];
    fn unknown_structure_ww_campaign_2(&self) -> &Vec<Vec<u8>>;
    fn unknown_structure_ww_campaign_2_mut(&mut self) -> &mut Vec<Vec<u8>>;
    fn unknown_5(&self) -> i32;
    fn set_unknown_5(&mut self, unknown: i32);

    fn to_buffer(&self, buffer: &mut Vec<u8>);
}

impl fmt::Debug for dyn PlayerDataFourIsh {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("PlayerDataFourIsh")
            .field("constant_name", &self.constant_name())
            .field("editor_camera_x", &self.editor_camera_x())
            .field("editor_camera_y", &self.editor_camera_y())
            .field("initial_camera_x", &self.initial_camera_x())
            .field("initial_camera_y", &self.initial_camera_y())
            .field("aok_allied_victory", &self.aok_allied_victory())
            .field(
                "player_count_for_diplomacy",
                &self.player_count_for_diplomacy(),
            )
            .field(
                "diplomacy_for_interaction",
                &self.diplomacy_for_interaction(),
            )
            .field("diplomacy_for_ai_system", &self.diplomacy_for_ai_system())
            .field("color", &self.color())
            .field("victory_version", &self.victory_version())
            .field("unknown", &self.unknown())
            .field("unknown_2", &self.unknown_2())
            .field(
                "unknown_structure_grand_theft_empires",
                &self.unknown_structure_grand_theft_empires(),
            )
            .field("unknown_3", &self.unknown_3())
            .field("unknown_4", &self.unknown_4())
            .field(
                "unknown_structure_ww_campaign_2",
                &self.unknown_structure_ww_campaign_2(),
            )
            .field("unknown_5", &self.unknown_5())
            .finish()
    }
}

pub trait PlayerUnitsIsh {
    fn unit_count(&self) -> u32;
    fn set_unit_count(&mut self, count: u32);
    fn units(&self) -> &Vec<Box<dyn UnitIsh>>;
    fn units_mut(&mut self) -> &mut Vec<Box<dyn UnitIsh>>;

    fn to_buffer(&self, buffer: &mut Vec<u8>);
}

impl fmt::Debug for dyn PlayerUnitsIsh {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("PlayerUnitsIsh")
            .field("unit_count", &self.unit_count())
            .field("units", &self.units())
            .finish()
    }
}

pub trait UnitIsh {
    fn x(&self) -> f32;
    fn set_x(&mut self, x: f32);
    fn y(&self) -> f32;
    fn set_y(&mut self, y: f32);
    fn z(&self) -> f32;
    fn set_z(&mut self, z: f32);
    fn id(&self) -> u32;
    fn set_id(&mut self, id: u32);
    fn constant(&self) -> u16;
    fn set_constant(&mut self, constant: u16);
    fn status(&self) -> u8;
    fn set_status(&mut self, status: u8);
    fn rotation(&self) -> f32;
    fn set_rotation(&mut self, rotation: f32);
    fn initial_frame(&self) -> u16;
    fn set_initial_frame(&mut self, initial_frame: u16);
    fn garrisoned_in_id(&self) -> i32;
    fn set_garrisoned_in_id(&mut self, id: i32);

    fn to_buffer(&self, buffer: &mut Vec<u8>);
}

impl fmt::Debug for dyn UnitIsh {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("UnitIsh")
            .field("x", &self.x())
            .field("y", &self.y())
            .field("z", &self.z())
            .field("id", &self.id())
            .field("constant", &self.constant())
            .field("status", &self.status())
            .field("rotation", &self.rotation())
            .field("initial_frame", &self.initial_frame())
            .field("garrisoned_in_id", &self.garrisoned_in_id())
            .finish()
    }
}

pub trait TriggersIsh {
    fn version(&self) -> f64;
    fn set_version(&mut self, version: f64);
    fn instruction_start(&self) -> i8;
    fn set_instruction_start(&mut self, start: i8);
    fn number_of_triggers(&self) -> i32;
    fn set_number_of_triggers(&mut self, number: i32);
    fn trigger_data(&self) -> &Vec<Box<dyn TriggerDataIsh>>;
    fn trigger_data_mut(&mut self) -> &mut Vec<Box<dyn TriggerDataIsh>>;
    fn trigger_display_order_array(&self) -> &Vec<u32>;
    fn trigger_display_order_array_mut(&mut self) -> &mut Vec<u32>;
    fn unknown_bytes(&self) -> &Vec<u8>;
    fn unknown_bytes_mut(&mut self) -> &mut Vec<u8>;
    fn number_of_variables(&self) -> u32;
    fn set_number_of_variables(&mut self, number_of_variables: u32);
    fn variable_data(&self) -> &Vec<Box<dyn VariableIsh>>;
    fn variable_data_mut(&mut self) -> &mut Vec<Box<dyn VariableIsh>>;
    fn useless_trigger_data(&self) -> &Vec<u8>;
    fn useless_trigger_data_mut(&mut self) -> &mut Vec<u8>;

    fn to_buffer(&self, buffer: &mut Vec<u8>);
}

impl fmt::Debug for dyn TriggersIsh {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("TriggersIsh")
            .field("version", &self.version())
            .field("instruction_start", &self.instruction_start())
            .field("number_of_triggers", &self.number_of_triggers())
            .field("trigger_data", &self.trigger_data())
            .field(
                "trigger_display_order_array",
                &self.trigger_display_order_array(),
            )
            .field("unknown_bytes", &self.unknown_bytes())
            .field("number_of_variables", &self.number_of_variables())
            .field("variable_data", &self.variable_data())
            .field("useless_trigger_data", &self.useless_trigger_data())
            .finish()
    }
}

pub trait TriggerDataIsh {
    fn enabled(&self) -> u32;
    fn set_enabled(&mut self, enable: u32);
    fn looping(&self) -> i8;
    fn set_looping(&mut self, looping: i8);
    fn description_string_table_id(&self) -> i32;
    fn set_description_string_table_id(&mut self, description_string_table_id: i32);
    fn display_as_objective(&self) -> u8;
    fn set_display_as_objective(&mut self, display_as_objective: u8);
    fn objective_description_order(&self) -> u32;
    fn set_objective_description_order(&mut self, objective_description_order: u32);
    fn make_header(&self) -> u8;
    fn set_make_header(&mut self, make_header: u8);
    fn short_description_string_table_id(&self) -> i32;
    fn set_short_description_string_table_id(&mut self, short_description_string_table_id: i32);
    fn display_on_screen(&self) -> u8;
    fn set_display_on_screen(&mut self, display_on_screen: u8);
    fn unknown(&self) -> &Vec<u8>;
    fn set_unknown(&mut self, unknown: Vec<u8>);
    fn mute_objectives(&self) -> u8;
    fn set_mute_objectives(&mut self, mute_objectives: u8);
    fn trigger_description(&self) -> &PrefixString<u32>;
    fn set_trigger_description(&mut self, trigger_description: &str);
    fn trigger_name(&self) -> &PrefixString<u32>;
    fn set_trigger_name(&mut self, trigger_name: &str);
    fn short_description(&self) -> &PrefixString<u32>;
    fn set_short_description(&mut self, short_description: &str);
    fn number_of_effects(&self) -> i32;
    fn set_number_of_effects(&mut self, number_of_effects: i32);
    fn effect_data(&self) -> &Vec<Box<dyn EffectDataIsh>>;
    fn effect_data_mut(&mut self) -> &mut Vec<Box<dyn EffectDataIsh>>;
    fn effect_display_order_array(&self) -> &Vec<i32>;
    fn effect_display_order_array_mut(&mut self) -> &mut Vec<i32>;
    fn number_of_conditions(&self) -> i32;
    fn set_number_of_conditions(&mut self, number_of_conditions: i32);
    fn condition_data(&self) -> &Vec<Box<dyn ConditionDataIsh>>;
    fn condition_data_mut(&mut self) -> &mut Vec<Box<dyn ConditionDataIsh>>;
    fn condition_display_order(&self) -> &Vec<i32>;
    fn condition_display_order_mut(&mut self) -> &mut Vec<i32>;

    fn to_buffer(&self, buffer: &mut Vec<u8>);
}

impl fmt::Debug for dyn TriggerDataIsh {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("TriggerDataIsh")
            .field("enabled", &self.enabled())
            .field("looping", &self.looping())
            .field(
                "description_string_table_id",
                &self.description_string_table_id(),
            )
            .field(
                "objective_description_order",
                &self.objective_description_order(),
            )
            .field("make_header", &self.make_header())
            .field(
                "short_description_string_table_id",
                &self.short_description_string_table_id(),
            )
            .field("display_on_screen", &self.display_on_screen())
            .field("unknown", &self.unknown())
            .field("mute_objectives", &self.mute_objectives())
            .field("trigger_description", &self.trigger_description())
            .field("trigger_name", &self.trigger_name())
            .field("short_description", &self.short_description())
            .field("number_of_effects", &self.number_of_effects())
            .field("effect_data", &self.effect_data())
            .field(
                "effect_display_order_array",
                &self.effect_display_order_array(),
            )
            .field("number_of_conditions", &self.number_of_conditions())
            .field("condition_data", &self.condition_data())
            .field("condition_display_order", &self.condition_display_order())
            .finish()
    }
}

pub trait EffectDataIsh {
    fn effect_type(&self) -> i32;
    fn set_effect_type(&mut self, effect_type: i32);
    fn static_value_46(&self) -> i32;
    fn set_static_value_46(&mut self, static_value: i32);
    fn ai_script_goal(&self) -> i32;
    fn set_ai_script_goal(&mut self, goal: i32);
    fn quatity(&self) -> i32;
    fn set_quatity(&mut self, quatity: i32);
    fn tribue_list(&self) -> i32;
    fn set_tribue_list(&mut self, list: i32);
    fn diplomacy(&self) -> i32;
    fn set_diplomacy(&mut self, diplomacy: i32);
    fn number_of_units_selected(&self) -> i32;
    fn set_number_of_units_selected(&mut self, number: i32);
    fn legacy_location_object_reference(&self) -> i32;
    fn set_legacy_location_object_reference(&mut self, reference: i32);
    fn object_list_unit_id(&self) -> i32;
    fn set_object_list_unit_id(&mut self, id: i32);
    fn source_player(&self) -> i32;
    fn set_source_player(&mut self, source_player: i32);
    fn target_player(&self) -> i32;
    fn set_target_player(&mut self, target_player: i32);
    fn technology(&self) -> i32;
    fn set_technology(&mut self, tech: i32);
    fn string_id(&self) -> i32;
    fn set_string_id(&mut self, string_id: i32);
    fn unknown_2(&self) -> i32;
    fn set_unknown_2(&mut self, unknown: i32);
    fn display_time(&self) -> i32;
    fn set_display_time(&mut self, display_time: i32);
    fn trigger_id(&self) -> i32;
    fn set_trigger_id(&mut self, trigger_id: i32);
    fn location_x(&self) -> i32;
    fn set_location_x(&mut self, location: i32);
    fn location_y(&self) -> i32;
    fn set_location_y(&mut self, location: i32);
    fn area_x1(&self) -> i32;
    fn set_area_x1(&mut self, area_x1: i32);
    fn area_y1(&self) -> i32;
    fn set_area_y1(&mut self, area_y1: i32);
    fn area_x2(&self) -> i32;
    fn set_area_x2(&mut self, area_x2: i32);
    fn area_y2(&self) -> i32;
    fn set_area_y2(&mut self, area_y2: i32);
    fn object_group(&self) -> i32;
    fn set_object_group(&mut self, object_group: i32);
    fn object_type(&self) -> i32;
    fn set_object_type(&mut self, object_type: i32);
    fn instruction_panel_position(&self) -> i32;
    fn set_instruction_panel_position(&mut self, position: i32);
    fn attack_stance(&self) -> i32;
    fn set_attack_stance(&mut self, attack_stance: i32);
    fn time_unit(&self) -> i32;
    fn set_time_unit(&mut self, time_unit: i32);
    fn enabled(&self) -> i32;
    fn set_enabled(&mut self, enable: i32);
    fn food(&self) -> i32;
    fn set_food(&mut self, food: i32);
    fn wood(&self) -> i32;
    fn set_wood(&mut self, wood: i32);
    fn stone(&self) -> i32;
    fn set_stone(&mut self, stone: i32);
    fn gold(&self) -> i32;
    fn set_gold(&mut self, gold: i32);
    fn item_id(&self) -> i32;
    fn set_item_id(&mut self, item_id: i32);
    fn flash_object(&self) -> i32;
    fn set_flash_object(&mut self, flash_object: i32);
    fn force_research_technology(&self) -> i32;
    fn set_force_research_technology(&mut self, force_research_force: i32);
    fn visibility_state(&self) -> i32;
    fn set_visibility_state(&mut self, visibility_state: i32);
    fn scroll(&self) -> i32;
    fn set_scroll(&mut self, scroll: i32);
    fn operation(&self) -> i32;
    fn set_operation(&mut self, operation: i32);
    fn object_list_unit_id_2(&self) -> i32;
    fn set_object_list_unit_id_2(&mut self, object_list_unit_id_2: i32);
    fn button_location(&self) -> i32;
    fn set_button_location(&mut self, button_location: i32);
    fn ai_signal_value(&self) -> i32;
    fn set_ai_signal_value(&mut self, ai_signal_value: i32);
    fn unknown_3(&self) -> i32;
    fn set_unknown_3(&mut self, unknown: i32);
    fn object_attributes(&self) -> i32;
    fn set_object_attributes(&mut self, object_attributes: i32);
    fn variable(&self) -> i32;
    fn set_variable(&mut self, variable: i32);
    fn timer(&self) -> i32;
    fn set_timer(&mut self, timer: i32);
    fn facet(&self) -> i32;
    fn set_facet(&mut self, facet: i32);
    fn location_object_reference(&self) -> i32;
    fn set_location_object_reference(&mut self, reference: i32);
    fn play_sound(&self) -> i32;
    fn set_play_sound(&mut self, play_sound: i32);
    fn player_color(&self) -> i32;
    fn set_player_color(&mut self, player_color: i32);
    fn unknown_4(&self) -> i32;
    fn set_unknown_4(&mut self, unknown: i32);
    fn color_mood(&self) -> i32;
    fn set_color_mood(&mut self, color_mood: i32);
    fn reset_time(&self) -> i32;
    fn set_reset_time(&mut self, reset_time: i32);
    fn object_state(&self) -> i32;
    fn set_object_state(&mut self, object_state: i32);
    fn action_type(&self) -> i32;
    fn set_action_type(&mut self, action_type: i32);
    fn message(&self) -> &PrefixString<u32>;
    fn set_message(&mut self, message: &str);
    fn sound_name(&self) -> &PrefixString<u32>;
    fn set_sound_name(&mut self, sound_name: &str);
    fn selected_object_ids(&self) -> &Vec<i32>;
    fn selected_object_ids_mut(&mut self) -> &mut Vec<i32>;

    fn to_buffer(&self, buffer: &mut Vec<u8>);
}

impl fmt::Debug for dyn EffectDataIsh {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("EffectDataIsh")
            .field("effect_type", &self.effect_type())
            .field("static_value_46", &self.static_value_46())
            .field("ai_script_goal", &self.ai_script_goal())
            .field("quatity", &self.quatity())
            .field("tribue_list", &self.tribue_list())
            .field("diplomacy", &self.diplomacy())
            .field("number_of_units_selected", &self.number_of_units_selected())
            .field(
                "legacy_location_object_reference",
                &self.legacy_location_object_reference(),
            )
            .field("object_list_unit_id", &self.object_list_unit_id())
            .field("source_player", &self.source_player())
            .field("target_player", &self.target_player())
            .field("technology", &self.technology())
            .field("string_id", &self.string_id())
            .field("unknown_2", &self.unknown_2())
            .field("display_time", &self.display_time())
            .field("trigger_id", &self.trigger_id())
            .field("location_x", &self.location_x())
            .field("location_y", &self.location_y())
            .field("area_x1", &self.area_x1())
            .field("area_y1", &self.area_y1())
            .field("area_x2", &self.area_x2())
            .field("area_y2", &self.area_y2())
            .field("object_group", &self.object_group())
            .field("object_type", &self.object_type())
            .field(
                "instruction_panel_position",
                &self.instruction_panel_position(),
            )
            .field("attack_stance", &self.attack_stance())
            .field("time_unit", &self.time_unit())
            .field("enabled", &self.enabled())
            .field("food", &self.food())
            .field("wood", &self.wood())
            .field("stone", &self.stone())
            .field("gold", &self.gold())
            .field("item_id", &self.item_id())
            .field("flash_object", &self.flash_object())
            .field(
                "force_research_technology",
                &self.force_research_technology(),
            )
            .field("visibility_state", &self.visibility_state())
            .field("scroll", &self.scroll())
            .field("operation", &self.operation())
            .field("object_list_unit_id_2", &self.object_list_unit_id_2())
            .field("button_location", &self.button_location())
            .field("ai_signal_value", &self.ai_signal_value())
            .field("unknown_3", &self.unknown_3())
            .field("object_attributes", &self.object_attributes())
            .field("variable", &self.variable())
            .field("timer", &self.timer())
            .field("facet", &self.facet())
            .field(
                "location_object_reference",
                &self.location_object_reference(),
            )
            .field("play_sound", &self.play_sound())
            .field("player_color", &self.player_color())
            .field("unknown_4", &self.unknown_4())
            .field("color_mood", &self.color_mood())
            .field("reset_time", &self.reset_time())
            .field("object_state", &self.object_state())
            .field("action_type", &self.action_type())
            .field("message", &self.message())
            .field("sound_name", &self.sound_name())
            .field("selected_object_ids", &self.selected_object_ids())
            .finish()
    }
}

pub trait ConditionDataIsh {
    fn condition_type(&self) -> i32;
    fn set_condition_type(&mut self, condition_type: i32);
    fn static_value_21(&self) -> i32;
    fn set_static_value_21(&mut self, static_value_32: i32);
    fn quantity(&self) -> i32;
    fn set_quantity(&mut self, quantity: i32);
    fn attribute(&self) -> i32;
    fn set_attribute(&mut self, attribute: i32);
    fn unit_object(&self) -> i32;
    fn set_unit_object(&mut self, unit_object: i32);
    fn next_object(&self) -> i32;
    fn set_next_object(&mut self, next_object: i32);
    fn object_list(&self) -> i32;
    fn set_object_list(&mut self, object_list: i32);
    fn source_player(&self) -> i32;
    fn set_source_player(&mut self, source_player: i32);
    fn technology(&self) -> i32;
    fn set_technology(&mut self, technology: i32);
    fn timer(&self) -> i32;
    fn set_timer(&mut self, timer: i32);
    fn unknown(&self) -> i32;
    fn set_unknown(&mut self, unknown: i32);
    fn area_x1(&self) -> i32;
    fn set_area_x1(&mut self, area_x1: i32);
    fn area_y1(&self) -> i32;
    fn set_area_y1(&mut self, area_y1: i32);
    fn area_x2(&self) -> i32;
    fn set_area_x2(&mut self, area_x2: i32);
    fn area_y2(&self) -> i32;
    fn set_area_y2(&mut self, area_y2: i32);
    fn object_group(&self) -> i32;
    fn set_object_group(&mut self, object_group: i32);
    fn object_type(&self) -> i32;
    fn set_object_type(&mut self, object_type: i32);
    fn ai_signal(&self) -> i32;
    fn set_ai_signal(&mut self, ai_signal: i32);
    fn inverted(&self) -> i32;
    fn set_inverted(&mut self, inverted: i32);
    fn unknown_2(&self) -> i32;
    fn set_unknown_2(&mut self, unknown_2: i32);
    fn variable(&self) -> i32;
    fn set_variable(&mut self, variable: i32);
    fn comparison(&self) -> i32;
    fn set_comparision(&mut self, comparison: i32);
    fn target_player(&self) -> i32;
    fn set_target_player(&mut self, target_player: i32);
    fn unit_ai_action(&self) -> i32;
    fn set_unit_ai_action(&mut self, unit_ai_action: i32);
    fn unknown_4(&self) -> i32;
    fn set_unknown_4(&mut self, unknown_4: i32);
    fn object_state(&self) -> i32;
    fn set_object_state(&mut self, object_state: i32);
    fn timer_id(&self) -> i32;
    fn set_timer_id(&mut self, timer_id: i32);
    fn victory_timer_type(&self) -> i32;
    fn set_victory_timer_type(&mut self, victory_timer_type: i32);
    fn include_changeable_weapon_objects(&self) -> i32;
    fn set_include_changeable_wapon_objects(&mut self, include_changeable_weapon_objects: i32);
    fn xs_function(&self) -> &PrefixString<u32>;
    fn set_xs_function(&mut self, xs_function: &str);

    fn to_buffer(&self, buffer: &mut Vec<u8>);
}

impl fmt::Debug for dyn ConditionDataIsh {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("ConditionDataIsh")
            .field("condition_type", &self.condition_type())
            .field("static_value_21", &self.static_value_21())
            .field("quantity", &self.quantity())
            .field("attribute", &self.attribute())
            .field("unit_object", &self.unit_object())
            .field("next_object", &self.next_object())
            .field("object_list", &self.object_list())
            .field("source_player", &self.source_player())
            .field("technology", &self.technology())
            .field("timer", &self.timer())
            .field("unknown", &self.unknown())
            .field("area_x1", &self.area_x1())
            .field("area_y1", &self.area_y1())
            .field("area_x2", &self.area_x2())
            .field("area_y2", &self.area_y2())
            .field("object_group", &self.object_group())
            .field("object_type", &self.object_type())
            .field("ai_signal", &self.ai_signal())
            .field("inverted", &self.inverted())
            .field("unknown_2", &self.unknown_2())
            .field("variable", &self.variable())
            .field("comparison", &self.comparison())
            .field("target_player", &self.target_player())
            .field("unit_ai_action", &self.unit_ai_action())
            .field("unknown_4", &self.unknown_4())
            .field("object_state", &self.object_state())
            .field("timer_id", &self.timer_id())
            .field("victory_timer_type", &self.victory_timer_type())
            .field(
                "include_changeable_weapon_objects",
                &self.include_changeable_weapon_objects(),
            )
            .field("xs_function", &self.xs_function())
            .finish()
    }
}

pub trait VariableIsh {
    fn variable_id(&self) -> u32;
    fn set_variable_id(&mut self, variable_id: u32);
    fn variable_name(&self) -> &PrefixString<u32>;
    fn set_variable_name(&mut self, variable_name: &str);

    fn to_buffer(&self, buffer: &mut Vec<u8>);
}

impl fmt::Debug for dyn VariableIsh {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("VariableIsh")
            .field("variable", &self.variable_id())
            .field("variable_name", &self.variable_name())
            .finish()
    }
}

pub trait FilesIsh {
    fn script_file_path(&self) -> &PrefixString<u16>;
    fn set_script_file_path(&mut self, script_file_path: &str);
    fn script_file_content(&self) -> &PrefixString<u32>;
    fn set_script_file_content(&mut self, script_file_content: &str);
    fn ai_files_present(&self) -> u32;
    fn set_ai_files_present(&mut self, ai_files_present: u32);
    fn unknown(&self) -> &Vec<u8>;
    fn unknown_mut(&mut self) -> &mut Vec<u8>;
    fn number_of_ai_files(&self) -> &Vec<u32>;
    fn number_of_ai_files_mut(&mut self) -> &mut Vec<u32>;
    fn ai_files(&self) -> &Vec<Box<dyn AI2Ish>>;
    fn ai_file_mut(&mut self) -> &mut Vec<Box<dyn AI2Ish>>;

    fn to_buffer(&self, buffer: &mut Vec<u8>);
}

impl fmt::Debug for dyn FilesIsh {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("FilesIsh")
            .field("script_file_path", &self.script_file_path())
            .field("script_file_content", &self.script_file_content())
            .field("ai_files_present", &self.ai_files_present())
            .field("unknown", &self.unknown())
            .field("number_of_ai_files", &self.number_of_ai_files())
            .field("ai_files", &self.ai_files())
            .finish()
    }
}

pub trait AI2Ish {
    fn ai_file_name(&self) -> &PrefixString<u32>;
    fn set_ai_file_name(&mut self, ai_file_name: &str);
    fn ai_file(&self) -> &PrefixString<u32>;
    fn set_ai_file(&mut self, ai_file: &str);

    fn to_buffer(&self, buffer: &mut Vec<u8>);
}

impl fmt::Debug for dyn AI2Ish {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("AI2Ish")
            .field("ai_file_name", &self.ai_file_name())
            .field("ai_file", &self.ai_file())
            .finish()
    }
}
