use miniz_oxide::{deflate::compress_to_vec, inflate::decompress_to_vec};

use super::{
    AI2Ish, AIFileIsh, BackgroundImageIsh, BitMapInfoIsh, CinematicsIsh, ConditionDataIsh,
    DataHeaderIsh, DiplomacyIsh, EffectDataIsh, FileHeaderIsh, FilesIsh, GlobalVictoryIsh, MapIsh,
    MessagesIsh, OptionsIsh, PlayerDataFourIsh, PlayerDataOneIsh, PlayerDataThreeIsh,
    PlayerDataTwoIsh, PlayerDiplomacyIsh, PlayerUnitsIsh, ResourcesIsh, TerrainTileIsh,
    TriggerDataIsh, TriggersIsh, UnitIsh, UnitsIsh, VariableIsh, VersioIsh,
};
use crate::{data::PrefixString, generator::IncrementalGenerator, serializer::Serializer};

pub struct Versio {
    file_header: FileHeader,
    data_header: DataHeader,
    messages: Messages,
    cinematics: Cinematics,
    background_image: BackgroundImage,
    player_data_two: PlayerDataTwo,
    global_victory: GlobalVictory,
    diplomacy: Diplomacy,
    options: Options,
    map: Map,
    units: Units,
    triggers: Triggers,
    files: Files,
}

impl Versio {
    pub fn from_buffer(generator: &mut IncrementalGenerator) -> Self {
        let file_header = FileHeader::from_buffer(generator);

        let compressed_data = generator.get_res_bytes();

        let res_bytes = decompress_to_vec(&compressed_data).unwrap();

        let mut uncompressed = IncrementalGenerator::init(res_bytes);

        Versio {
            file_header: file_header,
            data_header: DataHeader::from_buffer(&mut uncompressed),
            messages: Messages::from_buffer(&mut uncompressed),
            cinematics: Cinematics::from_buffer(&mut uncompressed),
            background_image: BackgroundImage::from_buffer(&mut uncompressed),
            player_data_two: PlayerDataTwo::from_buffer(&mut uncompressed),
            global_victory: GlobalVictory::from_buffer(&mut uncompressed),
            diplomacy: Diplomacy::from_buffer(&mut uncompressed),
            options: Options::from_buffer(&mut uncompressed),
            map: Map::from_buffer(&mut uncompressed),
            units: Units::from_buffer(&mut uncompressed),
            triggers: Triggers::from_buffer(&mut uncompressed),
            files: Files::from_buffer(&mut uncompressed),
        }
    }
}

impl VersioIsh for Versio {
    fn file_header<'a>(&self) -> &(dyn FileHeaderIsh + 'a) {
        &self.file_header
    }

    fn file_header_mut<'a>(&mut self) -> &mut (dyn FileHeaderIsh + 'a) {
        &mut self.file_header
    }

    fn data_header<'a>(&self) -> &(dyn DataHeaderIsh + 'a) {
        &self.data_header
    }

    fn data_header_mut<'a>(&mut self) -> &mut (dyn DataHeaderIsh + 'a) {
        &mut self.data_header
    }

    fn messages<'a>(&self) -> &(dyn MessagesIsh + 'a) {
        &self.messages
    }

    fn messages_mut<'a>(&mut self) -> &mut (dyn MessagesIsh + 'a) {
        &mut self.messages
    }

    fn cinematics<'a>(&self) -> &(dyn CinematicsIsh + 'a) {
        &self.cinematics
    }

    fn cinematics_mut<'a>(&mut self) -> &mut (dyn CinematicsIsh + 'a) {
        &mut self.cinematics
    }

    fn background_image<'a>(&self) -> &(dyn BackgroundImageIsh + 'a) {
        &self.background_image
    }

    fn background_image_mut<'a>(&mut self) -> &mut (dyn BackgroundImageIsh + 'a) {
        &mut self.background_image
    }

    fn player_data_two<'a>(&self) -> &(dyn PlayerDataTwoIsh + 'a) {
        &self.player_data_two
    }

    fn player_data_two_mut<'a>(&mut self) -> &mut (dyn PlayerDataTwoIsh + 'a) {
        &mut self.player_data_two
    }

    fn global_victory<'a>(&self) -> &(dyn GlobalVictoryIsh + 'a) {
        &self.global_victory
    }

    fn global_victory_mut<'a>(&mut self) -> &mut (dyn GlobalVictoryIsh + 'a) {
        &mut self.global_victory
    }

    fn diplomacy<'a>(&self) -> &(dyn DiplomacyIsh + 'a) {
        &self.diplomacy
    }

    fn diplomacy_mut<'a>(&mut self) -> &mut (dyn DiplomacyIsh + 'a) {
        &mut self.diplomacy
    }

    fn options<'a>(&self) -> &(dyn OptionsIsh + 'a) {
        &self.options
    }

    fn options_mut<'a>(&mut self) -> &mut (dyn OptionsIsh + 'a) {
        &mut self.options
    }

    fn map<'a>(&self) -> &(dyn MapIsh + 'a) {
        &self.map
    }

    fn map_mut<'a>(&mut self) -> &mut (dyn MapIsh + 'a) {
        &mut self.map
    }

    fn units<'a>(&self) -> &(dyn UnitsIsh + 'a) {
        &self.units
    }

    fn units_mut<'a>(&mut self) -> &mut (dyn UnitsIsh + 'a) {
        &mut self.units
    }

    fn triggers<'a>(&self) -> &(dyn TriggersIsh + 'a) {
        &self.triggers
    }

    fn triggers_mut<'a>(&mut self) -> &mut (dyn TriggersIsh + 'a) {
        &mut self.triggers
    }

    fn files<'a>(&self) -> &(dyn FilesIsh + 'a) {
        &self.files
    }

    fn files_mut<'a>(&mut self) -> &mut (dyn FilesIsh + 'a) {
        &mut self.files
    }

    fn to_buffer(&self, buffer: &mut Vec<u8>) {
        self.file_header.to_buffer(buffer);

        let mut uncompressed_buffer = Vec::<u8>::new();
        self.data_header.to_buffer(&mut uncompressed_buffer);
        self.messages.to_buffer(&mut uncompressed_buffer);
        self.cinematics.to_buffer(&mut uncompressed_buffer);
        self.background_image.to_buffer(&mut uncompressed_buffer);

        self.player_data_two.to_buffer(&mut uncompressed_buffer);

        self.global_victory.to_buffer(&mut uncompressed_buffer);

        self.diplomacy.to_buffer(&mut uncompressed_buffer);

        self.options.to_buffer(&mut uncompressed_buffer);
        self.map.to_buffer(&mut uncompressed_buffer);
        self.units.to_buffer(&mut uncompressed_buffer);
        self.triggers.to_buffer(&mut uncompressed_buffer);
        self.files.to_buffer(&mut uncompressed_buffer);

        let mut compressed = compress_to_vec(&uncompressed_buffer, 5);

        buffer.append(&mut compressed);
    }
}

pub struct FileHeader {
    version: String,
    header_length: u32,
    savable: i32,
    timestamp: u32,
    instructions: PrefixString<u32>,
    player_count: u32,
    unknown_data: [u32; 9],
    creator_name: PrefixString<u32>,
    trigger_count: u32,
}

impl FileHeader {
    pub fn from_buffer(generator: &mut IncrementalGenerator) -> Self {
        FileHeader {
            version: generator.get_str_with_len(4),
            header_length: generator.get_unsigned_int_32(),
            savable: generator.get_signed_int_32(),
            timestamp: generator.get_unsigned_int_32(),
            instructions: generator.get_str_32(),
            player_count: generator.get_unsigned_int_32(),
            unknown_data: [
                generator.get_unsigned_int_32(),
                generator.get_unsigned_int_32(),
                generator.get_unsigned_int_32(),
                generator.get_unsigned_int_32(),
                generator.get_unsigned_int_32(),
                generator.get_unsigned_int_32(),
                generator.get_unsigned_int_32(),
                generator.get_unsigned_int_32(),
                generator.get_unsigned_int_32(),
            ],
            creator_name: generator.get_str_32(),
            trigger_count: generator.get_unsigned_int_32(),
        }
    }
}

impl FileHeaderIsh for FileHeader {
    fn version(&self) -> &String {
        &self.version
    }

    fn set_version(&mut self, version: String) {
        self.version = version;
    }

    fn header_length(&self) -> u32 {
        self.header_length
    }

    fn set_header_length(&mut self, header_length: u32) {
        self.header_length = header_length;
    }

    fn savable(&self) -> i32 {
        self.savable
    }

    fn set_savable(&mut self, savable: i32) {
        self.savable = savable;
    }

    fn timestamp(&self) -> u32 {
        self.timestamp
    }

    fn set_timestamp(&mut self, timestamp: u32) {
        self.timestamp = timestamp;
    }

    fn instructions(&self) -> &PrefixString<u32> {
        &self.instructions
    }

    fn set_instructions(&mut self, instructions: &str) {
        self.instructions.raw = instructions.to_string();
    }

    fn player_count(&self) -> u32 {
        self.player_count
    }

    fn set_player_count(&mut self, player_count: u32) {
        self.player_count = player_count;
    }

    fn unknown_data(&self) -> &[u32; 9] {
        &self.unknown_data
    }

    fn set_unknown_data(&mut self, unknown_data: [u32; 9]) {
        self.unknown_data = unknown_data;
    }

    fn creator_name(&self) -> &PrefixString<u32> {
        &self.creator_name
    }

    fn set_creator_name(&mut self, creator_name: &str) {
        self.creator_name.raw = creator_name.to_string();
    }

    fn trigger_count(&self) -> u32 {
        self.trigger_count
    }

    fn set_trigger_count(&mut self, trigger_count: u32) {
        self.trigger_count = trigger_count;
    }

    fn to_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut Serializer::convert_to_bytes(&self.version));
        buffer.append(&mut Serializer::convert_to_bytes(&self.header_length));
        buffer.append(&mut Serializer::convert_to_bytes(&self.savable));
        buffer.append(&mut Serializer::convert_to_bytes(&self.timestamp));
        buffer.append(&mut Serializer::convert_to_bytes(&self.instructions));
        buffer.append(&mut Serializer::convert_to_bytes(&self.player_count));
        buffer.append(&mut Serializer::convert_to_bytes(&self.unknown_data));
        buffer.append(&mut Serializer::convert_to_bytes(&self.creator_name));
        buffer.append(&mut Serializer::convert_to_bytes(&self.trigger_count));
    }
}

pub struct DataHeader {
    next_unit_id_to_place: u32,
    version: f32,
    player_names: [String; 16],
    string_table_player_names: [u32; 16],
    player_data_one: [Box<dyn PlayerDataOneIsh>; 16],
    conquest_mode: u8,
    mission_items_counter: u16,
    mission_available: u16,
    mission_timeline: f32,
    _unknown: Vec<u8>,
    filename: PrefixString<u16>,
}

impl DataHeader {
    fn from_buffer(generator: &mut IncrementalGenerator) -> Self {
        let next_unit_id_to_place = generator.get_unsigned_int_32();
        let version = generator.get_float_32();
        let player_names = generator.get_str_with_len_repeat_16(256);
        let string_table_player_names = generator.get_unsigned_int_32_repeat_16();
        let player_data_one = PlayerDataOne::from_buffer_repeat_16(generator);
        let conquest_mode = generator.get_unsigned_int_8();
        let mission_items_counter = generator.get_unsigned_int_16();
        let mission_available = generator.get_unsigned_int_16();
        let mission_timeline = generator.get_float_32();
        let unknown = generator.skip_bytes((mission_items_counter * 30 + 64).into());
        let filename = generator.get_str_16();

        DataHeader {
            next_unit_id_to_place: next_unit_id_to_place,
            version: version,
            player_names: player_names,
            string_table_player_names: string_table_player_names,
            player_data_one: player_data_one,
            conquest_mode: conquest_mode,
            mission_items_counter: mission_items_counter,
            mission_available: mission_available,
            mission_timeline: mission_timeline,
            _unknown: unknown,
            filename: filename,
        }
    }
}

impl DataHeaderIsh for DataHeader {
    fn next_unit_id_to_place(&self) -> u32 {
        self.next_unit_id_to_place
    }

    fn set_next_unit_id_to_place(&mut self, next_unit_id_to_place: u32) {
        self.next_unit_id_to_place = next_unit_id_to_place;
    }

    fn version(&self) -> f32 {
        self.version
    }

    fn set_version(&mut self, version: f32) {
        self.version = version;
    }

    fn player_names(&self) -> &[String; 16] {
        &self.player_names
    }

    fn player_names_mut(&mut self) -> &mut [String; 16] {
        &mut self.player_names
    }

    fn string_table_player_names(&self) -> &[u32; 16] {
        &self.string_table_player_names
    }

    fn string_table_player_names_mut(&mut self) -> &mut [u32; 16] {
        &mut self.string_table_player_names
    }

    fn player_data_one(&self) -> &[Box<dyn PlayerDataOneIsh>; 16] {
        &self.player_data_one
    }

    fn player_data_one_mut(&mut self) -> &mut [Box<dyn PlayerDataOneIsh>; 16] {
        &mut self.player_data_one
    }

    fn conquest_mode(&self) -> u8 {
        self.conquest_mode
    }

    fn set_conquest_mode(&mut self, conquest_mode: u8) {
        self.conquest_mode = conquest_mode;
    }

    fn mission_items_counter(&self) -> u16 {
        self.mission_items_counter
    }

    fn set_mission_items_counter(&mut self, mission_items_counter: u16) {
        self.mission_items_counter = mission_items_counter;
    }

    fn mission_available(&self) -> u16 {
        self.mission_available
    }

    fn set_mission_available(&mut self, mission_available: u16) {
        self.mission_available = mission_available;
    }

    fn mission_timeline(&self) -> f32 {
        self.mission_timeline
    }

    fn set_mission_timeline(&mut self, mission_timeline: f32) {
        self.mission_timeline = mission_timeline;
    }

    fn filename(&self) -> &PrefixString<u16> {
        &self.filename
    }

    fn set_filename(&mut self, filename: &str) {
        self.filename.raw = filename.to_string();
    }

    fn to_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.next_unit_id_to_place,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(&self.version));
        buffer.append(&mut Serializer::convert_to_bytes(&self.player_names));
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.string_table_player_names,
        ));

        for player_data in &self.player_data_one {
            player_data.to_buffer(buffer);
        }
        buffer.append(&mut Serializer::convert_to_bytes(&self.conquest_mode));
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.mission_items_counter,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(&self.mission_available));
        buffer.append(&mut Serializer::convert_to_bytes(&self.mission_timeline));
        buffer.append(&mut Serializer::convert_to_bytes(&self._unknown));
        buffer.append(&mut Serializer::convert_to_bytes(&self.filename));
    }
}

pub struct PlayerDataOne {
    active: u32,
    human: u32,
    civiliztion: u32,
    architecture_set: u32,
    cty_mode: u32,
}

impl PlayerDataOne {
    fn from_buffer(generator: &mut IncrementalGenerator) -> Box<dyn PlayerDataOneIsh> {
        Box::new(PlayerDataOne {
            active: generator.get_unsigned_int_32(),
            human: generator.get_unsigned_int_32(),
            civiliztion: generator.get_unsigned_int_32(),
            architecture_set: generator.get_unsigned_int_32(),
            cty_mode: generator.get_unsigned_int_32(),
        })
    }

    fn from_buffer_repeat_16(
        generator: &mut IncrementalGenerator,
    ) -> [Box<dyn PlayerDataOneIsh>; 16] {
        [
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
        ]
    }
}

impl Default for PlayerDataOne {
    fn default() -> Self {
        Self {
            active: Default::default(),
            human: Default::default(),
            civiliztion: Default::default(),
            architecture_set: Default::default(),
            cty_mode: Default::default(),
        }
    }
}

impl PlayerDataOneIsh for PlayerDataOne {
    fn actvie(&self) -> u32 {
        self.active
    }

    fn set_active(&mut self, active: u32) {
        self.active = active;
    }

    fn human(&self) -> u32 {
        self.human
    }

    fn set_human(&mut self, human: u32) {
        self.human = human;
    }

    fn civilization(&self) -> u32 {
        self.civiliztion
    }

    fn set_civilization(&mut self, civiliztion: u32) {
        self.civiliztion = civiliztion;
    }

    fn architecture_set(&self) -> u32 {
        self.architecture_set
    }

    fn set_architecture_set(&mut self, architecture_set: u32) {
        self.architecture_set = architecture_set;
    }

    fn cty_mode(&self) -> u32 {
        self.cty_mode
    }

    fn set_cty_mode(&mut self, cty_mode: u32) {
        self.cty_mode = cty_mode;
    }

    fn to_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut Serializer::convert_to_bytes(&self.active));
        buffer.append(&mut Serializer::convert_to_bytes(&self.human));
        buffer.append(&mut Serializer::convert_to_bytes(&self.civiliztion));
        buffer.append(&mut Serializer::convert_to_bytes(&self.architecture_set));
        buffer.append(&mut Serializer::convert_to_bytes(&self.cty_mode));
    }
}

pub struct Messages {
    instructions: u32,
    hints: u32,
    victory: u32,
    loss: u32,
    history: u32,
    scouts: u32,
    ascii_instructions: PrefixString<u16>,
    ascii_hints: PrefixString<u16>,
    ascii_victory: PrefixString<u16>,
    ascii_loss: PrefixString<u16>,
    ascii_history: PrefixString<u16>,
    ascii_scouts: PrefixString<u16>,
}

impl Messages {
    pub fn from_buffer(generator: &mut IncrementalGenerator) -> Self {
        Messages {
            instructions: generator.get_unsigned_int_32(),
            hints: generator.get_unsigned_int_32(),
            victory: generator.get_unsigned_int_32(),
            loss: generator.get_unsigned_int_32(),
            history: generator.get_unsigned_int_32(),
            scouts: generator.get_unsigned_int_32(),
            ascii_instructions: generator.get_str_16(),
            ascii_hints: generator.get_str_16(),
            ascii_victory: generator.get_str_16(),
            ascii_loss: generator.get_str_16(),
            ascii_history: generator.get_str_16(),
            ascii_scouts: generator.get_str_16(),
        }
    }
}

impl MessagesIsh for Messages {
    fn instructions(&self) -> u32 {
        self.instructions
    }

    fn set_instructions(&mut self, instructions: u32) {
        self.instructions = instructions;
    }

    fn hints(&self) -> u32 {
        self.hints
    }

    fn set_hints(&mut self, hints: u32) {
        self.hints = hints;
    }

    fn victory(&self) -> u32 {
        self.victory
    }

    fn set_victory(&mut self, victory: u32) {
        self.victory = victory;
    }

    fn loss(&self) -> u32 {
        self.loss
    }

    fn set_loss(&mut self, loss: u32) {
        self.loss = loss;
    }

    fn history(&self) -> u32 {
        self.history
    }

    fn set_history(&mut self, history: u32) {
        self.history = history;
    }

    fn scouts(&self) -> u32 {
        self.scouts
    }

    fn set_scouts(&mut self, scouts: u32) {
        self.scouts = scouts;
    }

    fn ascii_instructions(&self) -> &PrefixString<u16> {
        &self.ascii_instructions
    }

    fn set_ascii_instructions(&mut self, ascii_instructions: &str) {
        self.ascii_instructions.raw = ascii_instructions.to_string();
    }

    fn ascii_hints(&self) -> &PrefixString<u16> {
        &self.ascii_hints
    }

    fn set_ascii_hints(&mut self, ascii_hints: &str) {
        self.ascii_hints.raw = ascii_hints.to_string();
    }

    fn ascii_victory(&self) -> &PrefixString<u16> {
        &self.ascii_victory
    }

    fn set_ascii_victory(&mut self, ascii_victory: &str) {
        self.ascii_victory.raw = ascii_victory.to_string();
    }

    fn ascii_loss(&self) -> &PrefixString<u16> {
        &self.ascii_loss
    }

    fn set_ascii_loss(&mut self, ascii_loss: &str) {
        self.ascii_loss.raw = ascii_loss.to_string();
    }

    fn ascii_history(&self) -> &PrefixString<u16> {
        &self.ascii_history
    }

    fn set_ascii_history(&mut self, ascii_history: &str) {
        self.ascii_history.raw = ascii_history.to_string();
    }

    fn ascii_scouts(&self) -> &PrefixString<u16> {
        &self.ascii_scouts
    }

    fn set_ascii_scouts(&mut self, ascii_scouts: &str) {
        self.ascii_scouts.raw = ascii_scouts.to_string();
    }

    fn to_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut Serializer::convert_to_bytes(&self.instructions));
        buffer.append(&mut Serializer::convert_to_bytes(&self.hints));
        buffer.append(&mut Serializer::convert_to_bytes(&self.victory));
        buffer.append(&mut Serializer::convert_to_bytes(&self.loss));
        buffer.append(&mut Serializer::convert_to_bytes(&self.history));
        buffer.append(&mut Serializer::convert_to_bytes(&self.scouts));
        buffer.append(&mut Serializer::convert_to_bytes(&self.ascii_instructions));
        buffer.append(&mut Serializer::convert_to_bytes(&self.ascii_hints));
        buffer.append(&mut Serializer::convert_to_bytes(&self.ascii_victory));
        buffer.append(&mut Serializer::convert_to_bytes(&self.ascii_loss));
        buffer.append(&mut Serializer::convert_to_bytes(&self.ascii_history));
        buffer.append(&mut Serializer::convert_to_bytes(&self.ascii_scouts));
    }
}

pub struct Cinematics {
    pregame_cinematic_filename: PrefixString<u16>,
    victory_cinematice_filename: PrefixString<u16>,
    loss_cinematic_filename: PrefixString<u16>,
}

impl Cinematics {
    pub fn from_buffer(generator: &mut IncrementalGenerator) -> Self {
        Cinematics {
            pregame_cinematic_filename: generator.get_str_16(),
            victory_cinematice_filename: generator.get_str_16(),
            loss_cinematic_filename: generator.get_str_16(),
        }
    }
}

impl CinematicsIsh for Cinematics {
    fn pregame_cinematic_filename(&self) -> &PrefixString<u16> {
        &self.pregame_cinematic_filename
    }

    fn set_pregame_cinematic_filename(&mut self, pregame_cinematic_filename: &str) {
        self.pregame_cinematic_filename.raw = pregame_cinematic_filename.to_string();
    }

    fn victory_cinematice_filename(&self) -> &PrefixString<u16> {
        &self.victory_cinematice_filename
    }

    fn set_victory_cinematice_filename(&mut self, victory_cinematice_filename: &str) {
        self.victory_cinematice_filename.raw = victory_cinematice_filename.to_string();
    }

    fn loss_cinematic_filename(&self) -> &PrefixString<u16> {
        &self.loss_cinematic_filename
    }

    fn set_loss_cinematic_filename(&mut self, loss_cinematic_filename: &str) {
        self.loss_cinematic_filename.raw = loss_cinematic_filename.to_string()
    }

    fn to_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.pregame_cinematic_filename,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.victory_cinematice_filename,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.loss_cinematic_filename,
        ));
    }
}

pub struct BackgroundImage {
    filename: PrefixString<u16>,
    picture_version: u32,
    bitmap_width: u32,
    bitmap_height: u32,
    picture_orientation: i16,
    bitmap_info: BitMapInfo,
}

impl BackgroundImage {
    pub fn from_buffer(generator: &mut IncrementalGenerator) -> Self {
        BackgroundImage {
            filename: generator.get_str_16(),
            picture_version: generator.get_unsigned_int_32(),
            bitmap_width: generator.get_unsigned_int_32(),
            bitmap_height: generator.get_unsigned_int_32(),
            picture_orientation: generator.get_signed_int_16(),
            bitmap_info: BitMapInfo::from_buffer(generator),
        }
    }
}

impl BackgroundImageIsh for BackgroundImage {
    fn filename(&self) -> &PrefixString<u16> {
        &self.filename
    }

    fn set_filename(&mut self, filename: &str) {
        self.filename.raw = filename.to_string();
    }

    fn picture_version(&self) -> u32 {
        self.picture_version
    }

    fn set_picture_version(&mut self, picture_version: u32) {
        self.picture_version = picture_version;
    }

    fn bitmap_width(&self) -> u32 {
        self.bitmap_width
    }

    fn set_bitmap_width(&mut self, width: u32) {
        self.bitmap_width = width;
    }

    fn bitmap_height(&self) -> u32 {
        self.bitmap_height
    }

    fn set_bitmap_height(&mut self, height: u32) {
        self.bitmap_height = height;
    }

    fn picture_orientation(&self) -> i16 {
        self.picture_orientation
    }

    fn set_picture_orientation(&mut self, orientation: i16) {
        self.picture_orientation = orientation;
    }

    fn bitmap_info<'a>(&self) -> &(dyn BitMapInfoIsh + 'a) {
        &self.bitmap_info
    }

    fn bitmap_info_mut<'a>(&mut self) -> &mut (dyn BitMapInfoIsh + 'a) {
        &mut self.bitmap_info
    }

    fn to_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut Serializer::convert_to_bytes(&self.filename));
        buffer.append(&mut Serializer::convert_to_bytes(&self.picture_version));
        buffer.append(&mut Serializer::convert_to_bytes(&self.bitmap_width));
        buffer.append(&mut Serializer::convert_to_bytes(&self.bitmap_height));
        buffer.append(&mut Serializer::convert_to_bytes(&self.picture_orientation));
        self.bitmap_info.to_buffer(buffer);
    }
}

pub struct BitMapInfo {
    size: i32,
    width: u32,
    height: u32,
    planes: i16,
    bit_count: i16,
    compression: u32,
    image_size: u32,
    x_pels: u32,
    y_pels: u32,
    number_of_colors_uesd: u32,
    important_colors: u32,
    colors_used: Vec<u32>,
    image: Vec<u8>,
}

impl BitMapInfo {
    pub fn from_buffer(generator: &mut IncrementalGenerator) -> Self {
        let size = generator.get_signed_int_32();
        let width = generator.get_unsigned_int_32();
        let height = generator.get_unsigned_int_32();
        let planes = generator.get_signed_int_16();
        let bit_count = generator.get_signed_int_16();
        let compression = generator.get_unsigned_int_32();
        let image_size = generator.get_unsigned_int_32();
        let x_pels = generator.get_unsigned_int_32();
        let y_pels = generator.get_unsigned_int_32();
        let number_of_colors_uesd = generator.get_unsigned_int_32();
        let important_colors = generator.get_unsigned_int_32();

        let mut color_used = Vec::<u32>::new();
        for _ in 0..number_of_colors_uesd {
            color_used.push(generator.get_unsigned_int_32());
        }

        let mut image = Vec::<u8>::new();
        for _ in 0..width * height {
            image.push(generator.get_unsigned_int_8());
        }

        BitMapInfo {
            size: size,
            width: width,
            height: height,
            planes: planes,
            bit_count: bit_count,
            compression: compression,
            image_size: image_size,
            x_pels: x_pels,
            y_pels: y_pels,
            number_of_colors_uesd: number_of_colors_uesd,
            important_colors: important_colors,
            colors_used: color_used,
            image: image,
        }
    }
}

impl BitMapInfoIsh for BitMapInfo {
    fn size(&self) -> i32 {
        self.size
    }

    fn set_size(&mut self, size: i32) {
        self.size = size;
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    fn planes(&self) -> i16 {
        self.planes
    }

    fn set_planes(&mut self, planes: i16) {
        self.planes = planes;
    }

    fn bit_count(&self) -> i16 {
        self.bit_count
    }

    fn set_bit_count(&mut self, bit_count: i16) {
        self.bit_count = bit_count;
    }

    fn compression(&self) -> u32 {
        self.compression
    }

    fn set_compression(&mut self, compression: u32) {
        self.compression = compression;
    }

    fn image_size(&self) -> u32 {
        self.image_size
    }

    fn set_image_szie(&mut self, image_size: u32) {
        self.image_size = image_size;
    }

    fn x_pels(&self) -> u32 {
        self.x_pels
    }

    fn set_x_pels(&mut self, x_pels: u32) {
        self.x_pels = x_pels;
    }

    fn y_pels(&self) -> u32 {
        self.y_pels
    }

    fn set_y_pels(&mut self, y_pels: u32) {
        self.y_pels = y_pels;
    }

    fn number_of_colors_used(&self) -> u32 {
        self.number_of_colors_uesd
    }

    fn set_number_of_colors_used(&mut self, num_of_colors: u32) {
        self.number_of_colors_uesd = num_of_colors;
    }

    fn important_colors(&self) -> u32 {
        self.important_colors
    }

    fn set_important_colors(&mut self, colors: u32) {
        self.important_colors = colors;
    }

    fn colors_used(&self) -> &Vec<u32> {
        &self.colors_used
    }

    fn set_colors_used(&mut self, color_used: Vec<u32>) {
        self.colors_used = color_used;
    }

    fn image(&self) -> &Vec<u8> {
        &self.image
    }

    fn set_image(&mut self, image: Vec<u8>) {
        self.image = image;
    }

    fn to_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut Serializer::convert_to_bytes(&self.size));
        buffer.append(&mut Serializer::convert_to_bytes(&self.width));
        buffer.append(&mut Serializer::convert_to_bytes(&self.height));
        buffer.append(&mut Serializer::convert_to_bytes(&self.planes));
        buffer.append(&mut Serializer::convert_to_bytes(&self.bit_count));
        buffer.append(&mut Serializer::convert_to_bytes(&self.compression));
        buffer.append(&mut Serializer::convert_to_bytes(&self.image_size));
        buffer.append(&mut Serializer::convert_to_bytes(&self.x_pels));
        buffer.append(&mut Serializer::convert_to_bytes(&self.y_pels));
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.number_of_colors_uesd,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(&self.important_colors));
        buffer.append(&mut Serializer::convert_to_bytes(&self.colors_used));
        buffer.append(&mut Serializer::convert_to_bytes(&self.image));
    }
}

pub struct PlayerDataTwo {
    unknown: [PrefixString<u16>; 32],
    ai_names: [PrefixString<u16>; 16],
    ai_files: [Box<dyn AIFileIsh>; 16],
    ai_types: [u8; 16],
    separator: u32,
    resources: [Box<dyn ResourcesIsh>; 16],
}

impl PlayerDataTwo {
    pub fn from_buffer(generator: &mut IncrementalGenerator) -> Self {
        PlayerDataTwo {
            unknown: generator.get_str_16_repeat_32(),
            ai_names: generator.get_str_16_repeat_16(),
            ai_files: AIFile::from_buffer_repeat_16(generator),
            ai_types: generator.get_unsigned_int_8_repeat_16(),
            separator: generator.get_unsigned_int_32(),
            resources: Resources::from_buffer_repeat_16(generator),
        }
    }
}

impl PlayerDataTwoIsh for PlayerDataTwo {
    fn unknown(&self) -> &[PrefixString<u16>; 32] {
        &self.unknown
    }

    fn unknown_mut(&mut self) -> &mut [PrefixString<u16>; 32] {
        &mut self.unknown
    }

    fn ai_names(&self) -> &[PrefixString<u16>; 16] {
        &self.ai_names
    }

    fn ai_names_mut(&mut self) -> &mut [PrefixString<u16>; 16] {
        &mut self.ai_names
    }

    fn ai_files(&self) -> &[Box<dyn AIFileIsh>; 16] {
        &self.ai_files
    }

    fn ai_files_mut(&mut self) -> &mut [Box<dyn AIFileIsh>; 16] {
        &mut self.ai_files
    }

    fn ai_types(&self) -> &[u8; 16] {
        &self.ai_types
    }

    fn ai_types_mut(&mut self) -> &mut [u8; 16] {
        &mut self.ai_types
    }

    fn separator(&self) -> u32 {
        self.separator
    }

    fn set_separator(&mut self, separator: u32) {
        self.separator = separator;
    }

    fn resources(&self) -> &[Box<dyn ResourcesIsh>; 16] {
        &self.resources
    }

    fn resources_mut(&mut self) -> &mut [Box<dyn ResourcesIsh>; 16] {
        &mut self.resources
    }

    fn to_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut Serializer::convert_to_bytes(&self.unknown));
        buffer.append(&mut Serializer::convert_to_bytes(&self.ai_names));
        for file in &self.ai_files {
            file.to_buffer(buffer);
        }
        buffer.append(&mut Serializer::convert_to_bytes(&self.ai_types));
        buffer.append(&mut Serializer::convert_to_bytes(&self.separator));
        for resource in &self.resources {
            resource.to_buffer(buffer);
        }
    }
}

pub struct AIFile {
    unknown_1: u32,
    unknown_2: u32,
    ai_per_file_text: PrefixString<u32>,
}

impl AIFile {
    pub fn from_buffer(generator: &mut IncrementalGenerator) -> Box<dyn AIFileIsh> {
        let unknown_1 = generator.get_unsigned_int_32();
        let unknown_2 = generator.get_unsigned_int_32();
        let ai_per_file_text = generator.get_str_32();
        Box::new(AIFile {
            unknown_1: unknown_1,
            unknown_2: unknown_2,
            ai_per_file_text: ai_per_file_text,
        })
    }

    pub fn from_buffer_repeat_16(generator: &mut IncrementalGenerator) -> [Box<dyn AIFileIsh>; 16] {
        [
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
        ]
    }
}

impl AIFileIsh for AIFile {
    fn unknown_1(&self) -> u32 {
        self.unknown_1
    }

    fn set_unknown_1(&mut self, unknown: u32) {
        self.unknown_1 = unknown;
    }

    fn unknown_2(&self) -> u32 {
        self.unknown_2
    }

    fn set_unknown_2(&mut self, unknown: u32) {
        self.unknown_2 = unknown;
    }

    fn ai_per_file_text(&self) -> &PrefixString<u32> {
        &self.ai_per_file_text
    }

    fn set_ai_per_file_text(&mut self, text: &str) {
        self.ai_per_file_text.raw = text.to_string();
    }

    fn to_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut Serializer::convert_to_bytes(&self.unknown_1));
        buffer.append(&mut Serializer::convert_to_bytes(&self.unknown_2));
        buffer.append(&mut Serializer::convert_to_bytes(&self.ai_per_file_text));
    }
}

pub struct Resources {
    gold: u32,
    wood: u32,
    food: u32,
    stone: u32,
    ore_x: u32,
    trade_goods: u32,
    player_color: u32,
}

impl Resources {
    pub fn from_buffer(generator: &mut IncrementalGenerator) -> Box<dyn ResourcesIsh> {
        Box::new(Resources {
            gold: generator.get_unsigned_int_32(),
            wood: generator.get_unsigned_int_32(),
            food: generator.get_unsigned_int_32(),
            stone: generator.get_unsigned_int_32(),
            ore_x: generator.get_unsigned_int_32(),
            trade_goods: generator.get_unsigned_int_32(),
            player_color: generator.get_unsigned_int_32(),
        })
    }

    pub fn from_buffer_repeat_16(
        generator: &mut IncrementalGenerator,
    ) -> [Box<dyn ResourcesIsh>; 16] {
        [
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
        ]
    }
}

impl ResourcesIsh for Resources {
    fn gold(&self) -> u32 {
        self.gold
    }

    fn set_good(&mut self, gold: u32) {
        self.gold = gold;
    }

    fn wood(&self) -> u32 {
        self.wood
    }

    fn set_wood(&mut self, wood: u32) {
        self.wood = wood;
    }

    fn food(&self) -> u32 {
        self.food
    }

    fn set_food(&mut self, food: u32) {
        self.food = food;
    }

    fn stone(&self) -> u32 {
        self.stone
    }

    fn set_stone(&mut self, stone: u32) {
        self.stone = stone;
    }

    fn ore_x(&self) -> u32 {
        self.ore_x
    }

    fn set_ore_x(&mut self, ore_x: u32) {
        self.ore_x = ore_x;
    }

    fn trade_goods(&self) -> u32 {
        self.trade_goods
    }

    fn set_trade_goods(&mut self, trade_goods: u32) {
        self.trade_goods = trade_goods;
    }

    fn player_color(&self) -> u32 {
        self.player_color
    }

    fn set_player_color(&mut self, player_color: u32) {
        self.player_color = player_color;
    }

    fn to_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut Serializer::convert_to_bytes(&self.gold));
        buffer.append(&mut Serializer::convert_to_bytes(&self.wood));
        buffer.append(&mut Serializer::convert_to_bytes(&self.food));
        buffer.append(&mut Serializer::convert_to_bytes(&self.stone));
        buffer.append(&mut Serializer::convert_to_bytes(&self.ore_x));
        buffer.append(&mut Serializer::convert_to_bytes(&self.trade_goods));
        buffer.append(&mut Serializer::convert_to_bytes(&self.player_color));
    }
}

pub struct GlobalVictory {
    separator: u32,
    conquest_required: u32,
    ruins: u32,
    artifacts: u32,
    discovery: u32,
    explored_percent_of_map_required: u32,
    gold_requred: u32,
    all_custom_conditions_required: u32,
    mode: u32,
    required_score_for_score_victory: u32,
    time_for_timed_gamed_in_10ths_of_a_year: u32,
}

impl GlobalVictory {
    pub fn from_buffer(generator: &mut IncrementalGenerator) -> Self {
        GlobalVictory {
            separator: generator.get_unsigned_int_32(),
            conquest_required: generator.get_unsigned_int_32(),
            ruins: generator.get_unsigned_int_32(),
            artifacts: generator.get_unsigned_int_32(),
            discovery: generator.get_unsigned_int_32(),
            explored_percent_of_map_required: generator.get_unsigned_int_32(),
            gold_requred: generator.get_unsigned_int_32(),
            all_custom_conditions_required: generator.get_unsigned_int_32(),
            mode: generator.get_unsigned_int_32(),
            required_score_for_score_victory: generator.get_unsigned_int_32(),
            time_for_timed_gamed_in_10ths_of_a_year: generator.get_unsigned_int_32(),
        }
    }
}

impl GlobalVictoryIsh for GlobalVictory {
    fn separator(&self) -> u32 {
        self.separator
    }

    fn set_separator(&mut self, separator: u32) {
        self.separator = separator;
    }

    fn conquest_required(&self) -> u32 {
        self.conquest_required
    }

    fn set_conquest_required(&mut self, conquest: u32) {
        self.conquest_required = conquest;
    }

    fn ruins(&self) -> u32 {
        self.ruins
    }

    fn set_ruins(&mut self, ruins: u32) {
        self.ruins = ruins;
    }

    fn artifacts(&self) -> u32 {
        self.artifacts
    }

    fn set_artifacts(&mut self, artifacts: u32) {
        self.artifacts = artifacts;
    }

    fn discovery(&self) -> u32 {
        self.discovery
    }

    fn set_discovery(&mut self, discovery: u32) {
        self.discovery = discovery;
    }

    fn explored_percent_of_map_required(&self) -> u32 {
        self.explored_percent_of_map_required
    }

    fn set_explored_percent_of_map_required(&mut self, percent: u32) {
        self.explored_percent_of_map_required = percent;
    }

    fn gold_requred(&self) -> u32 {
        self.gold_requred
    }

    fn set_gold_required(&mut self, gold: u32) {
        self.gold_requred = gold;
    }

    fn all_custom_conditions_required(&self) -> u32 {
        self.all_custom_conditions_required
    }

    fn set_all_custom_conditions_required(&mut self, conditions: u32) {
        self.all_custom_conditions_required = conditions;
    }

    fn mode(&self) -> u32 {
        self.mode
    }

    fn set_mode(&mut self, mode: u32) {
        self.mode = mode;
    }

    fn required_score_for_score_victory(&self) -> u32 {
        self.required_score_for_score_victory
    }

    fn set_required_score_for_score_victory(&mut self, score: u32) {
        self.required_score_for_score_victory = score;
    }

    fn time_for_timed_gamed_in_10ths_of_a_year(&self) -> u32 {
        self.time_for_timed_gamed_in_10ths_of_a_year
    }

    fn set_time_for_timed_gamed_in_10(&mut self, time: u32) {
        self.time_for_timed_gamed_in_10ths_of_a_year = time;
    }

    fn to_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut Serializer::convert_to_bytes(&self.separator));
        buffer.append(&mut Serializer::convert_to_bytes(&self.conquest_required));
        buffer.append(&mut Serializer::convert_to_bytes(&self.ruins));
        buffer.append(&mut Serializer::convert_to_bytes(&self.artifacts));
        buffer.append(&mut Serializer::convert_to_bytes(&self.discovery));
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.explored_percent_of_map_required,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(&self.gold_requred));
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.all_custom_conditions_required,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(&self.mode));
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.required_score_for_score_victory,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.time_for_timed_gamed_in_10ths_of_a_year,
        ));
    }
}

pub struct Diplomacy {
    diplomacies: [Box<dyn PlayerDiplomacyIsh>; 16],
    individual_victories: Vec<u8>,
    seperator: u32,
    per_player_allied_victory: [u32; 16],
    lock_teams: u8,
    allow_players_choose_teams: u8,
    random_start_points: u8,
    max_number_of_teams: u8,
}

impl Diplomacy {
    pub fn from_buffer(generator: &mut IncrementalGenerator) -> Self {
        Diplomacy {
            diplomacies: PlayerDiplomacy::from_buffer_repeat_16(generator),
            individual_victories: generator.skip_bytes(60 * 192),
            seperator: generator.get_unsigned_int_32(),
            per_player_allied_victory: generator.get_unsigned_int_32_repeat_16(),
            lock_teams: generator.get_unsigned_int_8(),
            allow_players_choose_teams: generator.get_unsigned_int_8(),
            random_start_points: generator.get_unsigned_int_8(),
            max_number_of_teams: generator.get_unsigned_int_8(),
        }
    }
}

impl DiplomacyIsh for Diplomacy {
    fn diplomacies(&self) -> &[Box<dyn PlayerDiplomacyIsh>; 16] {
        &self.diplomacies
    }

    fn diplomacies_mut(&mut self) -> &mut [Box<dyn PlayerDiplomacyIsh>; 16] {
        &mut self.diplomacies
    }

    fn individual_victories(&self) -> &Vec<u8> {
        &self.individual_victories
    }

    fn individual_victories_mut(&mut self) -> &mut Vec<u8> {
        &mut self.individual_victories
    }

    fn seperator(&self) -> u32 {
        self.seperator
    }

    fn set_separator(&mut self, seperator: u32) {
        self.seperator = seperator;
    }

    fn per_player_allied_victory(&self) -> &[u32; 16] {
        &self.per_player_allied_victory
    }

    fn per_player_allied_victory_mut(&mut self) -> &mut [u32; 16] {
        &mut self.per_player_allied_victory
    }

    fn lock_teams(&self) -> u8 {
        self.lock_teams
    }

    fn set_lock_teams(&mut self, lock_team: u8) {
        self.lock_teams = lock_team;
    }

    fn allow_players_choose_teams(&self) -> u8 {
        self.allow_players_choose_teams
    }

    fn set_allow_players_choose_teams(&mut self, choose_team: u8) {
        self.allow_players_choose_teams = choose_team;
    }

    fn random_start_points(&self) -> u8 {
        self.random_start_points
    }

    fn set_random_start_points(&mut self, random_start_points: u8) {
        self.random_start_points = random_start_points;
    }

    fn max_number_of_teams(&self) -> u8 {
        self.max_number_of_teams
    }

    fn set_max_number_of_teams(&mut self, max_of_teams: u8) {
        self.max_number_of_teams = max_of_teams;
    }

    fn to_buffer(&self, buffer: &mut Vec<u8>) {
        for diplomacy in &self.diplomacies {
            diplomacy.to_buffer(buffer);
        }
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.individual_victories,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(&self.seperator));
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.per_player_allied_victory,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(&self.lock_teams));
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.allow_players_choose_teams,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(&self.random_start_points));
        buffer.append(&mut Serializer::convert_to_bytes(&self.max_number_of_teams));
    }
}

pub struct PlayerDiplomacy {
    stance: [u32; 16],
}

impl PlayerDiplomacy {
    pub fn from_buffer(generator: &mut IncrementalGenerator) -> Box<dyn PlayerDiplomacyIsh> {
        Box::new(PlayerDiplomacy {
            stance: generator.get_unsigned_int_32_repeat_16(),
        })
    }

    pub fn from_buffer_repeat_16(
        generator: &mut IncrementalGenerator,
    ) -> [Box<dyn PlayerDiplomacyIsh>; 16] {
        [
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
            Self::from_buffer(generator),
        ]
    }
}

impl PlayerDiplomacyIsh for PlayerDiplomacy {
    fn stance(&self) -> &[u32; 16] {
        &self.stance
    }

    fn stance_mut(&mut self) -> &mut [u32; 16] {
        &mut self.stance
    }

    fn to_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut Serializer::convert_to_bytes(&self.stance));
    }
}

pub struct Options {
    per_player_number_of_disabled_techs: [u32; 16],
    disabled_tech_ids_player: [Vec<u32>; 16],
    per_player_number_of_disabled_units: [u32; 16],
    disabled_unit_ids_player: [Vec<u32>; 16],
    per_player_number_of_disabled_buildings: [u32; 16],
    disabled_building_ids_player: [Vec<u32>; 16],
    combat_mode: u32,
    naval_mode: u32,
    all_techs: u32,
    per_player_starting_age: [u32; 16],
    unknown_1: Vec<u8>,
    per_player_base_priority: [u8; 8],
    unknown_2: Vec<u8>,
    number_of_triggers: u32,
}

impl Options {
    pub fn from_buffer(generator: &mut IncrementalGenerator) -> Self {
        let per_player_number_of_disabled_techs = generator.get_unsigned_int_32_repeat_16();
        let disabled_tech_ids_player = per_player_number_of_disabled_techs
            .map(|number| generator.get_vec_u32_with_len(number as usize));

        let per_player_number_of_disabled_units = generator.get_unsigned_int_32_repeat_16();
        let disabled_unit_ids_player = per_player_number_of_disabled_units
            .map(|number| generator.get_vec_u32_with_len(number as usize));

        let per_player_number_of_disabled_buildings = generator.get_unsigned_int_32_repeat_16();
        let disabled_building_ids_player = per_player_number_of_disabled_buildings
            .map(|number| generator.get_vec_u32_with_len(number as usize));

        Options {
            per_player_number_of_disabled_techs: per_player_number_of_disabled_techs,
            disabled_tech_ids_player: disabled_tech_ids_player,
            per_player_number_of_disabled_units: per_player_number_of_disabled_units,
            disabled_unit_ids_player: disabled_unit_ids_player,
            per_player_number_of_disabled_buildings: per_player_number_of_disabled_buildings,
            disabled_building_ids_player: disabled_building_ids_player,
            combat_mode: generator.get_unsigned_int_32(),
            naval_mode: generator.get_unsigned_int_32(),
            all_techs: generator.get_unsigned_int_32(),
            per_player_starting_age: generator.get_unsigned_int_32_repeat_16(),
            unknown_1: generator.skip_bytes(17),
            per_player_base_priority: generator.get_unsigned_int_8_repeat_8(),
            unknown_2: generator.skip_bytes(7),
            number_of_triggers: generator.get_unsigned_int_32(),
        }
    }
}

impl OptionsIsh for Options {
    fn per_player_number_of_disabled_techs(&self) -> &[u32; 16] {
        &self.per_player_number_of_disabled_techs
    }

    fn per_player_number_of_disabled_techs_mut(&mut self) -> &mut [u32; 16] {
        &mut self.per_player_number_of_disabled_techs
    }

    fn disabled_tech_ids_player(&self) -> &[Vec<u32>; 16] {
        &self.disabled_tech_ids_player
    }

    fn disabled_tech_ids_player_mut(&mut self) -> &mut [Vec<u32>; 16] {
        &mut self.disabled_tech_ids_player
    }

    fn per_player_number_of_disabled_units(&self) -> &[u32; 16] {
        &self.per_player_number_of_disabled_units
    }

    fn per_player_number_of_disabled_units_mut(&mut self) -> &mut [u32; 16] {
        &mut self.per_player_number_of_disabled_units
    }

    fn disabled_unit_ids_player(&self) -> &[Vec<u32>; 16] {
        &self.disabled_unit_ids_player
    }

    fn disabled_unit_ids_player_mut(&mut self) -> &mut [Vec<u32>; 16] {
        &mut self.disabled_unit_ids_player
    }

    fn per_player_number_of_disabled_buildings(&self) -> &[u32; 16] {
        &self.per_player_number_of_disabled_buildings
    }

    fn per_player_number_of_disabled_buildings_mut(&mut self) -> &mut [u32; 16] {
        &mut self.per_player_number_of_disabled_buildings
    }

    fn disabled_building_ids_player(&self) -> &[Vec<u32>; 16] {
        &self.disabled_building_ids_player
    }

    fn disabled_building_ids_player_mut(&mut self) -> &mut [Vec<u32>; 16] {
        &mut self.disabled_building_ids_player
    }

    fn combat_mode(&self) -> u32 {
        self.combat_mode
    }

    fn set_combat_mode(&mut self, combat_mode: u32) {
        self.combat_mode = combat_mode;
    }

    fn naval_mode(&self) -> u32 {
        self.naval_mode
    }

    fn set_naval_mode(&mut self, naval_mode: u32) {
        self.naval_mode = naval_mode;
    }

    fn all_techs(&self) -> u32 {
        self.all_techs
    }

    fn set_all_techs(&mut self, all_techs: u32) {
        self.all_techs = all_techs;
    }

    fn per_player_starting_age(&self) -> &[u32; 16] {
        &self.per_player_starting_age
    }

    fn per_player_starting_age_mut(&mut self) -> &mut [u32; 16] {
        &mut self.per_player_starting_age
    }

    fn unknown_1(&self) -> &Vec<u8> {
        &self.unknown_1
    }

    fn unknown_1_mut(&mut self) -> &mut Vec<u8> {
        &mut self.unknown_1
    }

    fn per_player_base_priority(&self) -> &[u8; 8] {
        &self.per_player_base_priority
    }

    fn per_player_base_priority_mut(&mut self) -> &mut [u8; 8] {
        &mut self.per_player_base_priority
    }

    fn unknown_2(&self) -> &Vec<u8> {
        &self.unknown_2
    }

    fn unknown_2_mut(&mut self) -> &mut Vec<u8> {
        &mut self.unknown_2
    }

    fn number_of_triggers(&self) -> u32 {
        self.number_of_triggers
    }

    fn set_number_of_triggers(&mut self, number: u32) {
        self.number_of_triggers = number;
    }

    fn to_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.per_player_number_of_disabled_techs,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.disabled_tech_ids_player,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.per_player_number_of_disabled_units,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.disabled_unit_ids_player,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.per_player_number_of_disabled_buildings,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.disabled_building_ids_player,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(&self.combat_mode));
        buffer.append(&mut Serializer::convert_to_bytes(&self.naval_mode));
        buffer.append(&mut Serializer::convert_to_bytes(&self.all_techs));
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.per_player_starting_age,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(&self.unknown_1));
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.per_player_base_priority,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(&self.unknown_2));
        buffer.append(&mut Serializer::convert_to_bytes(&self.number_of_triggers));
    }
}

pub struct Map {
    string_starter_1: Vec<u8>,
    water_definition: PrefixString<u16>,
    string_starter_2: Vec<u8>,
    map_color_mood: PrefixString<u16>,
    string_starter_3: Vec<u8>,
    script_name: PrefixString<u16>,
    collide_and_correct: u8,
    villager_force_drop: u8,
    unknown: Vec<u8>,
    lock_coop_alliances: u8,
    ai_map_type: i32,
    per_player_population_cap: [u32; 16],
    secondary_game_mode: u32,
    unknown_2: Vec<u8>,
    unknown_3: Vec<u8>,
    no_waves_on_shore: i8,
    map_width: i32,
    map_height: i32,
    terrain: Vec<Box<dyn TerrainTileIsh>>,
}

impl Map {
    pub fn from_buffer(generator: &mut IncrementalGenerator) -> Self {
        let string_starter_1 = generator.skip_bytes(2);
        let water_definition = generator.get_str_16();
        let string_starter_2 = generator.skip_bytes(2);
        let map_color_mood = generator.get_str_16();
        let string_starter_3 = generator.skip_bytes(2);
        let script_name = generator.get_str_16();
        let collide_and_correct = generator.get_unsigned_int_8();
        let villager_force_drop = generator.get_unsigned_int_8();
        let unknown = generator.skip_bytes(128);
        let lock_coop_alliances = generator.get_unsigned_int_8();
        let ai_map_type = generator.get_signed_int_32();
        let per_player_population_cap = generator.get_unsigned_int_32_repeat_16();
        let secondary_game_mode = generator.get_unsigned_int_32();
        let unknown_2 = generator.skip_bytes(4);
        let unknown_3 = generator.skip_bytes(4);
        let no_waves_on_shore = generator.get_signed_int_8();
        let map_width = generator.get_signed_int_32();
        let map_height = generator.get_signed_int_32();

        let mut terrain = Vec::<Box<dyn TerrainTileIsh>>::new();
        for _ in 0..(map_width * map_height) {
            terrain.push(Box::new(TerrainTile::from_buffer(generator)))
        }

        Map {
            string_starter_1: string_starter_1,
            water_definition: water_definition,
            string_starter_2: string_starter_2,
            map_color_mood: map_color_mood,
            string_starter_3: string_starter_3,
            script_name: script_name,
            collide_and_correct: collide_and_correct,
            villager_force_drop: villager_force_drop,
            unknown: unknown,
            lock_coop_alliances: lock_coop_alliances,
            ai_map_type: ai_map_type,
            per_player_population_cap: per_player_population_cap,
            secondary_game_mode: secondary_game_mode,
            unknown_2: unknown_2,
            unknown_3: unknown_3,
            no_waves_on_shore: no_waves_on_shore,
            map_width: map_width,
            map_height: map_height,
            terrain: terrain,
        }
    }
}

impl MapIsh for Map {
    fn string_starter_1(&self) -> &Vec<u8> {
        &self.string_starter_1
    }

    fn string_starter_1_mut(&mut self) -> &mut Vec<u8> {
        &mut self.string_starter_1
    }

    fn water_definition(&self) -> &PrefixString<u16> {
        &self.water_definition
    }

    fn set_water_definition(&mut self, water_definition: &str) {
        self.water_definition.raw = water_definition.to_string();
    }

    fn string_starter_2(&self) -> &Vec<u8> {
        &self.string_starter_2
    }

    fn string_starter_2_mut(&mut self) -> &mut Vec<u8> {
        &mut self.string_starter_2
    }

    fn map_color_mood(&self) -> &PrefixString<u16> {
        &self.map_color_mood
    }

    fn set_map_color_mood(&mut self, map_color_mood: &str) {
        self.map_color_mood.raw = map_color_mood.to_string();
    }

    fn string_starter_3(&self) -> &Vec<u8> {
        &self.string_starter_3
    }

    fn string_starter_3_mut(&mut self) -> &mut Vec<u8> {
        &mut self.string_starter_3
    }

    fn script_name(&self) -> &PrefixString<u16> {
        &self.script_name
    }

    fn set_script_name(&mut self, script_name: &str) {
        self.script_name.raw = script_name.to_string();
    }

    fn collide_and_correct(&self) -> u8 {
        self.collide_and_correct
    }

    fn set_collide_and_correct(&mut self, collide_and_correct: u8) {
        self.collide_and_correct = collide_and_correct;
    }

    fn villager_force_drop(&self) -> u8 {
        self.villager_force_drop
    }

    fn set_villager_force_drop(&mut self, villager_force_drop: u8) {
        self.villager_force_drop = villager_force_drop;
    }

    fn unknown(&self) -> &Vec<u8> {
        &self.unknown
    }

    fn unknown_mut(&mut self) -> &mut Vec<u8> {
        &mut self.unknown
    }

    fn lock_coop_alliances(&self) -> u8 {
        self.lock_coop_alliances
    }

    fn set_lock_coop_alliances(&mut self, lock_coop_alliances: u8) {
        self.lock_coop_alliances = lock_coop_alliances;
    }

    fn ai_map_type(&self) -> i32 {
        self.ai_map_type
    }

    fn set_ai_map_type(&mut self, ai_map_type: i32) {
        self.ai_map_type = ai_map_type;
    }

    fn per_player_population_cap(&self) -> &[u32; 16] {
        &self.per_player_population_cap
    }

    fn per_player_population_cap_mut(&mut self) -> &mut [u32; 16] {
        &mut self.per_player_population_cap
    }

    fn secondary_game_mode(&self) -> u32 {
        self.secondary_game_mode
    }

    fn set_secondary_game_mode(&mut self, seconday_game_mode: u32) {
        self.secondary_game_mode = seconday_game_mode;
    }

    fn unknown_2(&self) -> &Vec<u8> {
        &self.unknown_2
    }

    fn unknown_2_mut(&mut self) -> &mut Vec<u8> {
        &mut self.unknown_2
    }

    fn unknown_3(&self) -> &Vec<u8> {
        &self.unknown_3
    }

    fn unknown_3_mut(&mut self) -> &mut Vec<u8> {
        &mut self.unknown_3
    }

    fn no_waves_on_shore(&self) -> i8 {
        self.no_waves_on_shore
    }

    fn set_no_wave_on_shore(&mut self, flag: i8) {
        self.no_waves_on_shore = flag;
    }

    fn map_width(&self) -> i32 {
        self.map_width
    }

    fn set_map_width(&mut self, width: i32) {
        self.map_width = width;
    }

    fn map_height(&self) -> i32 {
        self.map_height
    }

    fn set_map_height(&mut self, height: i32) {
        self.map_height = height;
    }

    fn terrain(&self) -> &Vec<Box<dyn TerrainTileIsh>> {
        &self.terrain
    }

    fn terrain_mut(&mut self) -> &mut Vec<Box<dyn TerrainTileIsh>> {
        &mut self.terrain
    }

    fn to_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut Serializer::convert_to_bytes(&self.string_starter_1));
        buffer.append(&mut Serializer::convert_to_bytes(&self.water_definition));
        buffer.append(&mut Serializer::convert_to_bytes(&self.string_starter_2));
        buffer.append(&mut Serializer::convert_to_bytes(&self.map_color_mood));
        buffer.append(&mut Serializer::convert_to_bytes(&self.string_starter_3));
        buffer.append(&mut Serializer::convert_to_bytes(&self.script_name));
        buffer.append(&mut Serializer::convert_to_bytes(&self.collide_and_correct));
        buffer.append(&mut Serializer::convert_to_bytes(&self.villager_force_drop));
        buffer.append(&mut Serializer::convert_to_bytes(&self.unknown));
        buffer.append(&mut Serializer::convert_to_bytes(&self.lock_coop_alliances));
        buffer.append(&mut Serializer::convert_to_bytes(&self.ai_map_type));
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.per_player_population_cap,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(&self.secondary_game_mode));
        buffer.append(&mut Serializer::convert_to_bytes(&self.unknown_2));
        buffer.append(&mut Serializer::convert_to_bytes(&self.unknown_3));
        buffer.append(&mut Serializer::convert_to_bytes(&self.no_waves_on_shore));
        buffer.append(&mut Serializer::convert_to_bytes(&self.map_width));
        buffer.append(&mut Serializer::convert_to_bytes(&self.map_height));

        for terrain_tile in &self.terrain {
            terrain_tile.to_buffer(buffer)
        }
    }
}

pub struct TerrainTile {
    id: u8,
    elevation: u8,
    unused: Vec<u8>,
    layer: i16,
}

impl TerrainTile {
    pub fn from_buffer(generator: &mut IncrementalGenerator) -> Self {
        TerrainTile {
            id: generator.get_unsigned_int_8(),
            elevation: generator.get_unsigned_int_8(),
            unused: generator.skip_bytes(3),
            layer: generator.get_signed_int_16(),
        }
    }
}

impl TerrainTileIsh for TerrainTile {
    fn id(&self) -> u8 {
        self.id
    }

    fn set_id(&mut self, id: u8) {
        self.id = id;
    }

    fn elevation(&self) -> u8 {
        self.elevation
    }

    fn set_elevation(&mut self, elevation: u8) {
        self.elevation = elevation;
    }

    fn unused(&self) -> &Vec<u8> {
        &self.unused
    }

    fn unused_mut(&mut self) -> &mut Vec<u8> {
        &mut self.unused
    }

    fn layer(&self) -> i16 {
        self.layer
    }

    fn set_layer(&mut self, layer: i16) {
        self.layer = layer;
    }

    fn to_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut Serializer::convert_to_bytes(&self.id));
        buffer.append(&mut Serializer::convert_to_bytes(&self.elevation));
        buffer.append(&mut Serializer::convert_to_bytes(&self.unused));
        buffer.append(&mut Serializer::convert_to_bytes(&self.layer));
    }
}

pub struct Units {
    number_of_unit_sections: u32,
    player_data_three: [Box<dyn PlayerDataThreeIsh>; 8],
    number_of_players: u32,
    player_data_four: [Box<dyn PlayerDataFourIsh>; 8],
    player_units: Vec<Box<dyn PlayerUnitsIsh>>,
}

impl Units {
    pub fn from_buffer(generator: &mut IncrementalGenerator) -> Self {
        let number_of_unit_sections = generator.get_unsigned_int_32();
        let player_data_three: [Box<dyn PlayerDataThreeIsh>; 8] = [
            Box::new(PlayerDataThree::from_buffer(generator)),
            Box::new(PlayerDataThree::from_buffer(generator)),
            Box::new(PlayerDataThree::from_buffer(generator)),
            Box::new(PlayerDataThree::from_buffer(generator)),
            Box::new(PlayerDataThree::from_buffer(generator)),
            Box::new(PlayerDataThree::from_buffer(generator)),
            Box::new(PlayerDataThree::from_buffer(generator)),
            Box::new(PlayerDataThree::from_buffer(generator)),
        ];
        let number_of_players = generator.get_unsigned_int_32();
        let player_data_four: [Box<dyn PlayerDataFourIsh>; 8] = [
            Box::new(PlayerDataFour::from_buffer(generator)),
            Box::new(PlayerDataFour::from_buffer(generator)),
            Box::new(PlayerDataFour::from_buffer(generator)),
            Box::new(PlayerDataFour::from_buffer(generator)),
            Box::new(PlayerDataFour::from_buffer(generator)),
            Box::new(PlayerDataFour::from_buffer(generator)),
            Box::new(PlayerDataFour::from_buffer(generator)),
            Box::new(PlayerDataFour::from_buffer(generator)),
        ];

        let mut player_units = Vec::<Box<dyn PlayerUnitsIsh>>::new();

        for _ in 0..number_of_unit_sections {
            player_units.push(Box::new(PlayerUnits::from_buffer(generator)));
        }

        Units {
            number_of_unit_sections: number_of_unit_sections,
            player_data_three: player_data_three,
            number_of_players: number_of_players,
            player_data_four: player_data_four,
            player_units: player_units,
        }
    }
}

impl UnitsIsh for Units {
    fn number_of_unit_sections(&self) -> u32 {
        self.number_of_unit_sections
    }

    fn set_number_of_unit_sections(&mut self, number: u32) {
        self.number_of_unit_sections = number;
    }

    fn player_data_three(&self) -> &[Box<dyn PlayerDataThreeIsh>; 8] {
        &self.player_data_three
    }

    fn player_data_three_mut(&mut self) -> &mut [Box<dyn PlayerDataThreeIsh>; 8] {
        &mut self.player_data_three
    }

    fn number_of_players(&self) -> u32 {
        self.number_of_players
    }

    fn set_number_of_players(&mut self, number: u32) {
        self.number_of_players = number;
    }

    fn player_data_four(&self) -> &[Box<dyn PlayerDataFourIsh>; 8] {
        &self.player_data_four
    }

    fn player_data_four_mut(&mut self) -> &mut [Box<dyn PlayerDataFourIsh>; 8] {
        &mut self.player_data_four
    }

    fn player_units(&self) -> &Vec<Box<dyn PlayerUnitsIsh>> {
        &self.player_units
    }

    fn player_units_mut(&mut self) -> &mut Vec<Box<dyn PlayerUnitsIsh>> {
        &mut self.player_units
    }

    fn to_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.number_of_unit_sections,
        ));

        for player_data in &self.player_data_three {
            player_data.to_buffer(buffer);
        }
        buffer.append(&mut Serializer::convert_to_bytes(&self.number_of_players));

        for player_data in &self.player_data_four {
            player_data.to_buffer(buffer);
        }
        for unit in &self.player_units {
            unit.to_buffer(buffer);
        }
    }
}

pub struct PlayerDataThree {
    food_duplicate: f32,
    wood_duplicate: f32,
    gold_duplicate: f32,
    stone_duplicate: f32,
    ore_x_duplicate: f32,
    trade_goods_duplicate: f32,
    population_limit: f32,
}

impl PlayerDataThree {
    fn from_buffer(generator: &mut IncrementalGenerator) -> Self {
        PlayerDataThree {
            food_duplicate: generator.get_float_32(),
            wood_duplicate: generator.get_float_32(),
            gold_duplicate: generator.get_float_32(),
            stone_duplicate: generator.get_float_32(),
            ore_x_duplicate: generator.get_float_32(),
            trade_goods_duplicate: generator.get_float_32(),
            population_limit: generator.get_float_32(),
        }
    }
}

impl PlayerDataThreeIsh for PlayerDataThree {
    fn food_duplicate(&self) -> f32 {
        self.food_duplicate
    }

    fn set_food_duplicate(&mut self, food_duplicate: f32) {
        self.food_duplicate = food_duplicate;
    }

    fn wood_duplicate(&self) -> f32 {
        self.wood_duplicate
    }

    fn set_wood_duplicate(&mut self, wood_duplicate: f32) {
        self.wood_duplicate = wood_duplicate;
    }

    fn gold_duplicate(&self) -> f32 {
        self.gold_duplicate
    }

    fn set_gold_duplicate(&mut self, gold_duplicate: f32) {
        self.gold_duplicate = gold_duplicate;
    }

    fn stone_duplicate(&self) -> f32 {
        self.stone_duplicate
    }

    fn set_stone_duplicate(&mut self, stone_duplicate: f32) {
        self.stone_duplicate = stone_duplicate;
    }

    fn ore_x_duplicate(&self) -> f32 {
        self.ore_x_duplicate
    }

    fn set_ore_x_duplicate(&mut self, ore_x_duplicate: f32) {
        self.ore_x_duplicate = ore_x_duplicate;
    }

    fn trade_goods_duplicate(&self) -> f32 {
        self.trade_goods_duplicate
    }

    fn set_trade_goods_deplicate(&mut self, trade_good_duplicate: f32) {
        self.trade_goods_duplicate = trade_good_duplicate;
    }

    fn population_limit(&self) -> f32 {
        self.population_limit
    }

    fn set_population_limit(&mut self, population: f32) {
        self.population_limit = population
    }

    fn to_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut Serializer::convert_to_bytes(&self.food_duplicate));
        buffer.append(&mut Serializer::convert_to_bytes(&self.wood_duplicate));
        buffer.append(&mut Serializer::convert_to_bytes(&self.gold_duplicate));
        buffer.append(&mut Serializer::convert_to_bytes(&self.stone_duplicate));
        buffer.append(&mut Serializer::convert_to_bytes(&self.ore_x_duplicate));
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.trade_goods_duplicate,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(&self.population_limit));
    }
}

pub struct PlayerDataFour {
    constant_name: PrefixString<u16>,
    editor_camera_x: f32,
    editor_camera_y: f32,
    initial_camera_x: i16,
    initial_camera_y: i16,
    aok_allied_victory: u8,
    player_count_for_diplomacy: u16,
    diplomacy_for_interaction: Vec<u8>,
    diplomacy_for_ai_system: [u32; 9],
    color: u32,
    victory_version: f32,
    unknown: u16,
    unknown_2: Vec<u8>,
    unknown_structure_grand_theft_empires: Vec<u8>,
    unknown_3: Vec<u8>,
    unknown_4: [u8; 7],
    unknown_structure_ww_campaign_2: Vec<Vec<u8>>,
    unknown_5: i32,
}

impl PlayerDataFour {
    fn from_buffer(generator: &mut IncrementalGenerator) -> Self {
        let constant_name = generator.get_str_16();
        let editor_camera_x = generator.get_float_32();
        let editor_camera_y = generator.get_float_32();
        let initial_camera_x = generator.get_signed_int_16();
        let initial_camera_y = generator.get_signed_int_16();
        let aok_allied_victory = generator.get_unsigned_int_8();
        let player_count_for_diplomacy = generator.get_unsigned_int_16();
        let mut diplomacy_for_interaction = Vec::<u8>::new();
        for _ in 0..player_count_for_diplomacy {
            diplomacy_for_interaction.push(generator.get_unsigned_int_8());
        }

        let mut diplomacy_for_ai_system: [u32; 9] = Default::default();
        for i in 0..9 {
            diplomacy_for_ai_system[i] = generator.get_unsigned_int_32();
        }

        let color = generator.get_unsigned_int_32();
        let victory_version = generator.get_float_32();
        let unknown = generator.get_unsigned_int_16();
        let mut unknown_2 = Vec::<u8>::new();
        if victory_version == 2.0 {
            for _ in 0..7 {
                unknown_2.push(generator.get_unsigned_int_8());
            }
        }

        let unknown_structure_grand_theft_empires = generator.skip_bytes((44 * unknown).into());
        let mut unknown_3 = Vec::<u8>::new();
        if victory_version == 2.0 {
            unknown_3.push(generator.get_unsigned_int_8());
        }

        let mut unknown_4: [u8; 7] = Default::default();
        for i in 0..7 {
            unknown_4[i] = generator.get_unsigned_int_8();
        }

        let mut unknown_structure_ww_campaign_2 = Vec::<Vec<u8>>::new();
        match unknown_3.get(0) {
            Some(x) => unknown_structure_ww_campaign_2.push(generator.skip_bytes((32 * x).into())),
            None => (),
        }

        let unknown_5 = generator.get_signed_int_32();

        PlayerDataFour {
            constant_name: constant_name,
            editor_camera_x: editor_camera_x,
            editor_camera_y: editor_camera_y,
            initial_camera_x: initial_camera_x,
            initial_camera_y: initial_camera_y,
            aok_allied_victory: aok_allied_victory,
            player_count_for_diplomacy: player_count_for_diplomacy,
            diplomacy_for_interaction: diplomacy_for_interaction,
            diplomacy_for_ai_system: diplomacy_for_ai_system,
            color: color,
            victory_version: victory_version,
            unknown: unknown,
            unknown_2: unknown_2,
            unknown_structure_grand_theft_empires: unknown_structure_grand_theft_empires,
            unknown_3: unknown_3,
            unknown_4: unknown_4,
            unknown_structure_ww_campaign_2: unknown_structure_ww_campaign_2,
            unknown_5: unknown_5,
        }
    }
}

impl PlayerDataFourIsh for PlayerDataFour {
    fn constant_name(&self) -> &PrefixString<u16> {
        &self.constant_name
    }

    fn set_constant_name(&mut self, name: &str) {
        self.constant_name.raw = name.to_string();
    }

    fn editor_camera_x(&self) -> f32 {
        self.editor_camera_x
    }

    fn set_editor_camera_x(&mut self, x: f32) {
        self.editor_camera_x = x;
    }

    fn editor_camera_y(&self) -> f32 {
        self.editor_camera_y
    }

    fn set_editor_camera_y(&mut self, y: f32) {
        self.editor_camera_y = y;
    }

    fn initial_camera_x(&self) -> i16 {
        self.initial_camera_x
    }

    fn set_initial_camera_x(&mut self, x: i16) {
        self.initial_camera_x = x;
    }

    fn initial_camera_y(&self) -> i16 {
        self.initial_camera_y
    }

    fn aok_allied_victory(&self) -> u8 {
        self.aok_allied_victory
    }

    fn set_aok_allied_victory(&mut self, victory: u8) {
        self.aok_allied_victory = victory;
    }

    fn player_count_for_diplomacy(&self) -> u16 {
        self.player_count_for_diplomacy
    }

    fn set_player_count_for_diplomacy(&mut self, count: u16) {
        self.player_count_for_diplomacy = count;
    }

    fn diplomacy_for_interaction(&self) -> &Vec<u8> {
        &self.diplomacy_for_interaction
    }

    fn diplomacy_for_interaction_mut(&mut self) -> &mut Vec<u8> {
        &mut self.diplomacy_for_interaction
    }

    fn diplomacy_for_ai_system(&self) -> &[u32; 9] {
        &self.diplomacy_for_ai_system
    }

    fn diplomacy_for_ai_system_mut(&mut self) -> &mut [u32; 9] {
        &mut self.diplomacy_for_ai_system
    }

    fn color(&self) -> u32 {
        self.color
    }

    fn set_color(&mut self, color: u32) {
        self.color = color;
    }

    fn victory_version(&self) -> f32 {
        self.victory_version
    }

    fn set_victory_version(&mut self, version: f32) {
        self.victory_version = version;
    }

    fn unknown(&self) -> u16 {
        self.unknown
    }

    fn set_unknown(&mut self, unknown: u16) {
        self.unknown = unknown;
    }

    fn unknown_2(&self) -> &Vec<u8> {
        &self.unknown_2
    }

    fn unknown_2_mut(&mut self) -> &mut Vec<u8> {
        &mut self.unknown_2
    }

    fn unknown_structure_grand_theft_empires(&self) -> &Vec<u8> {
        &self.unknown_structure_grand_theft_empires
    }

    fn unknown_structure_grand_theft_empires_mut(&mut self) -> &mut Vec<u8> {
        &mut self.unknown_structure_grand_theft_empires
    }

    fn unknown_3(&self) -> &Vec<u8> {
        &self.unknown_3
    }

    fn unknown_3_mut(&mut self) -> &mut Vec<u8> {
        &mut self.unknown_3
    }

    fn unknown_4(&self) -> &[u8; 7] {
        &self.unknown_4
    }

    fn unknown_4_mut(&mut self) -> &mut [u8; 7] {
        &mut self.unknown_4
    }

    fn unknown_structure_ww_campaign_2(&self) -> &Vec<Vec<u8>> {
        &self.unknown_structure_ww_campaign_2
    }

    fn unknown_structure_ww_campaign_2_mut(&mut self) -> &mut Vec<Vec<u8>> {
        &mut self.unknown_structure_ww_campaign_2
    }

    fn unknown_5(&self) -> i32 {
        self.unknown_5
    }

    fn set_unknown_5(&mut self, unknown: i32) {
        self.unknown_5 = unknown;
    }

    fn to_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut Serializer::convert_to_bytes(&self.constant_name));
        buffer.append(&mut Serializer::convert_to_bytes(&self.editor_camera_x));
        buffer.append(&mut Serializer::convert_to_bytes(&self.editor_camera_y));
        buffer.append(&mut Serializer::convert_to_bytes(&self.initial_camera_x));
        buffer.append(&mut Serializer::convert_to_bytes(&self.initial_camera_y));
        buffer.append(&mut Serializer::convert_to_bytes(&self.aok_allied_victory));
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.player_count_for_diplomacy,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.diplomacy_for_interaction,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.diplomacy_for_ai_system,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(&self.color));
        buffer.append(&mut Serializer::convert_to_bytes(&self.victory_version));
        buffer.append(&mut Serializer::convert_to_bytes(&self.unknown));
        buffer.append(&mut Serializer::convert_to_bytes(&self.unknown_2));
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.unknown_structure_grand_theft_empires,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(&self.unknown_3));
        buffer.append(&mut Serializer::convert_to_bytes(&self.unknown_4));
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.unknown_structure_ww_campaign_2,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(&self.unknown_5));
    }
}

pub struct PlayerUnits {
    unit_count: u32,
    units: Vec<Box<dyn UnitIsh>>,
}

impl PlayerUnits {
    pub fn from_buffer(generator: &mut IncrementalGenerator) -> Self {
        let count = generator.get_unsigned_int_32();
        let mut units = Vec::<Box<dyn UnitIsh>>::new();
        for _ in 0..count {
            units.push(Box::new(Unit::from_buffer(generator)))
        }
        PlayerUnits {
            unit_count: count,
            units: units,
        }
    }
}

impl PlayerUnitsIsh for PlayerUnits {
    fn unit_count(&self) -> u32 {
        self.unit_count
    }

    fn set_unit_count(&mut self, count: u32) {
        self.unit_count = count;
    }

    fn units(&self) -> &Vec<Box<dyn UnitIsh>> {
        &self.units
    }

    fn units_mut(&mut self) -> &mut Vec<Box<dyn UnitIsh>> {
        &mut self.units
    }

    fn to_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut Serializer::convert_to_bytes(&self.unit_count));
        for unit in &self.units {
            unit.to_buffer(buffer);
        }
    }
}

pub struct Unit {
    x: f32,
    y: f32,
    z: f32,
    id: u32,
    constant: u16,
    status: u8,
    rotation: f32,
    initial_frame: u16,
    garrisoned_in_id: i32,
}

impl Unit {
    fn from_buffer(generator: &mut IncrementalGenerator) -> Self {
        Unit {
            x: generator.get_float_32(),
            y: generator.get_float_32(),
            z: generator.get_float_32(),
            id: generator.get_unsigned_int_32(),
            constant: generator.get_unsigned_int_16(),
            status: generator.get_unsigned_int_8(),
            rotation: generator.get_float_32(),
            initial_frame: generator.get_unsigned_int_16(),
            garrisoned_in_id: generator.get_signed_int_32(),
        }
    }
}

impl UnitIsh for Unit {
    fn x(&self) -> f32 {
        self.x
    }

    fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    fn y(&self) -> f32 {
        self.y
    }

    fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    fn z(&self) -> f32 {
        self.z
    }

    fn set_z(&mut self, z: f32) {
        self.z = z;
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    fn constant(&self) -> u16 {
        self.constant
    }

    fn set_constant(&mut self, constant: u16) {
        self.constant = constant;
    }

    fn status(&self) -> u8 {
        self.status
    }

    fn set_status(&mut self, status: u8) {
        self.status = status;
    }

    fn rotation(&self) -> f32 {
        self.rotation
    }

    fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    fn initial_frame(&self) -> u16 {
        self.initial_frame
    }

    fn set_initial_frame(&mut self, initial_frame: u16) {
        self.initial_frame = initial_frame;
    }

    fn garrisoned_in_id(&self) -> i32 {
        self.garrisoned_in_id
    }

    fn set_garrisoned_in_id(&mut self, id: i32) {
        self.garrisoned_in_id = id;
    }

    fn to_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut Serializer::convert_to_bytes(&self.x));
        buffer.append(&mut Serializer::convert_to_bytes(&self.y));
        buffer.append(&mut Serializer::convert_to_bytes(&self.z));
        buffer.append(&mut Serializer::convert_to_bytes(&self.id));
        buffer.append(&mut Serializer::convert_to_bytes(&self.constant));
        buffer.append(&mut Serializer::convert_to_bytes(&self.status));
        buffer.append(&mut Serializer::convert_to_bytes(&self.rotation));
        buffer.append(&mut Serializer::convert_to_bytes(&self.initial_frame));
        buffer.append(&mut Serializer::convert_to_bytes(&self.garrisoned_in_id));
    }
}

pub struct Triggers {
    trigger_version: f64,
    trigger_instuction_start: i8,
    number_of_triggers: i32,
    trigger_data: Vec<Box<dyn TriggerDataIsh>>,
    trigger_display_order_array: Vec<u32>,
    unknown_bytes: Vec<u8>,
    number_of_variables: u32,
    variable_data: Vec<Box<dyn VariableIsh>>,
    useless_trigger_data: Vec<u8>,
}

impl Triggers {
    pub fn from_buffer(generator: &mut IncrementalGenerator) -> Self {
        let version = generator.get_float_64();
        let instruction_start = generator.get_signed_int_8();
        let number_of_triggers = generator.get_signed_int_32();
        let mut trigger_data = Vec::<Box<dyn TriggerDataIsh>>::new();
        for _ in 0..number_of_triggers {
            trigger_data.push(Box::new(TriggerData::from_buffer(generator)));
        }

        let mut trigger_display_order_array = Vec::<u32>::new();
        for _ in 0..number_of_triggers {
            trigger_display_order_array.push(generator.get_unsigned_int_32());
        }

        let unknown_bytes = generator.skip_bytes(1028);
        let number_of_variables = generator.get_unsigned_int_32();

        let mut variable_data = Vec::<Box<dyn VariableIsh>>::new();
        for _ in 0..number_of_variables {
            variable_data.push(Box::new(Variable::from_buffer(generator)));
        }
        let useless_trigger_data = generator.skip_bytes(9);

        Triggers {
            trigger_version: version,
            trigger_instuction_start: instruction_start,
            number_of_triggers: number_of_triggers,
            trigger_data: trigger_data,
            trigger_display_order_array: trigger_display_order_array,
            unknown_bytes: unknown_bytes,
            number_of_variables: number_of_variables,
            variable_data: variable_data,
            useless_trigger_data: useless_trigger_data,
        }
    }
}

impl TriggersIsh for Triggers {
    fn version(&self) -> f64 {
        self.trigger_version
    }

    fn set_version(&mut self, version: f64) {
        self.trigger_version = version;
    }

    fn instruction_start(&self) -> i8 {
        self.trigger_instuction_start
    }

    fn set_instruction_start(&mut self, start: i8) {
        self.trigger_instuction_start = start;
    }

    fn number_of_triggers(&self) -> i32 {
        self.number_of_triggers
    }

    fn set_number_of_triggers(&mut self, number: i32) {
        self.number_of_triggers = number;
    }

    fn trigger_data(&self) -> &Vec<Box<dyn TriggerDataIsh>> {
        &self.trigger_data
    }

    fn trigger_data_mut(&mut self) -> &mut Vec<Box<dyn TriggerDataIsh>> {
        &mut self.trigger_data
    }

    fn trigger_display_order_array(&self) -> &Vec<u32> {
        &self.trigger_display_order_array
    }

    fn trigger_display_order_array_mut(&mut self) -> &mut Vec<u32> {
        &mut self.trigger_display_order_array
    }

    fn unknown_bytes(&self) -> &Vec<u8> {
        &self.unknown_bytes
    }

    fn unknown_bytes_mut(&mut self) -> &mut Vec<u8> {
        &mut self.unknown_bytes
    }

    fn number_of_variables(&self) -> u32 {
        self.number_of_variables
    }

    fn set_number_of_variables(&mut self, number_of_variables: u32) {
        self.number_of_variables = number_of_variables;
    }

    fn variable_data(&self) -> &Vec<Box<dyn VariableIsh>> {
        &self.variable_data
    }

    fn variable_data_mut(&mut self) -> &mut Vec<Box<dyn VariableIsh>> {
        &mut self.variable_data
    }

    fn useless_trigger_data(&self) -> &Vec<u8> {
        &self.useless_trigger_data
    }

    fn useless_trigger_data_mut(&mut self) -> &mut Vec<u8> {
        &mut self.useless_trigger_data
    }

    fn to_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut Serializer::convert_to_bytes(&self.trigger_version));
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.trigger_instuction_start,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(&self.number_of_triggers));
        for data in &self.trigger_data {
            data.to_buffer(buffer);
        }
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.trigger_display_order_array,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(&self.unknown_bytes));
        buffer.append(&mut Serializer::convert_to_bytes(&self.number_of_variables));
        for data in &self.variable_data {
            data.to_buffer(buffer);
        }
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.useless_trigger_data,
        ));
    }
}

pub struct TriggerData {
    enabled: u32,
    looping: i8,
    description_string_table_id: i32,
    display_as_objective: u8,
    objective_description_order: u32,
    make_header: u8,
    short_description_string_table_id: i32,
    display_on_screen: u8,
    unknown: Vec<u8>,
    mute_objectives: u8,
    trigger_description: PrefixString<u32>,
    trigger_name: PrefixString<u32>,
    short_description: PrefixString<u32>,
    number_of_effects: i32,
    effect_data: Vec<Box<dyn EffectDataIsh>>,
    effect_display_order_array: Vec<i32>,
    number_of_conditions: i32,
    condition_data: Vec<Box<dyn ConditionDataIsh>>,
    condition_display_order: Vec<i32>,
}

impl TriggerData {
    pub fn from_buffer(generator: &mut IncrementalGenerator) -> Self {
        let enabled = generator.get_unsigned_int_32();
        let looping = generator.get_signed_int_8();
        let description_string_table_id = generator.get_signed_int_32();
        let display_as_objective = generator.get_unsigned_int_8();
        let objective_description_order = generator.get_unsigned_int_32();
        let make_header = generator.get_unsigned_int_8();
        let short_description_string_table_id = generator.get_signed_int_32();
        let display_on_screen = generator.get_unsigned_int_8();
        let unknown = generator.skip_bytes(5);
        let mute_objectives = generator.get_unsigned_int_8();
        let trigger_description = generator.get_str_32();
        let trigger_name = generator.get_str_32();
        let short_description = generator.get_str_32();
        let number_of_effects = generator.get_signed_int_32();

        let mut effect_data = Vec::<Box<dyn EffectDataIsh>>::new();
        let mut effect_display_order_array = Vec::<i32>::new();
        for _ in 0..number_of_effects {
            effect_data.push(Box::new(EffectData::from_buffer(generator)));
        }

        for _ in 0..number_of_effects {
            effect_display_order_array.push(generator.get_signed_int_32());
        }

        let number_of_conditions = generator.get_signed_int_32();

        let mut condition_data = Vec::<Box<dyn ConditionDataIsh>>::new();
        for _ in 0..number_of_conditions {
            condition_data.push(Box::new(ConditionData::from_buffer(generator)));
        }

        let mut condition_display_order = Vec::<i32>::new();
        for _ in 0..number_of_conditions {
            condition_display_order.push(generator.get_signed_int_32());
        }

        TriggerData {
            enabled: enabled,
            looping: looping,
            description_string_table_id: description_string_table_id,
            display_as_objective: display_as_objective,
            objective_description_order: objective_description_order,
            make_header: make_header,
            short_description_string_table_id: short_description_string_table_id,
            display_on_screen: display_on_screen,
            unknown: unknown,
            mute_objectives: mute_objectives,
            trigger_description: trigger_description,
            trigger_name: trigger_name,
            short_description: short_description,
            number_of_effects: number_of_effects,
            effect_data: effect_data,
            effect_display_order_array: effect_display_order_array,
            number_of_conditions: number_of_conditions,
            condition_data: condition_data,
            condition_display_order: condition_display_order,
        }
    }
}

impl TriggerDataIsh for TriggerData {
    fn enabled(&self) -> u32 {
        self.enabled
    }

    fn set_enabled(&mut self, enable: u32) {
        self.enabled = enable;
    }

    fn looping(&self) -> i8 {
        self.looping
    }

    fn set_looping(&mut self, looping: i8) {
        self.looping = looping;
    }

    fn description_string_table_id(&self) -> i32 {
        self.description_string_table_id
    }

    fn set_description_string_table_id(&mut self, description_string_table_id: i32) {
        self.description_string_table_id = description_string_table_id;
    }

    fn display_as_objective(&self) -> u8 {
        self.display_as_objective
    }

    fn set_display_as_objective(&mut self, display_as_objective: u8) {
        self.display_as_objective = display_as_objective;
    }

    fn objective_description_order(&self) -> u32 {
        self.objective_description_order
    }

    fn set_objective_description_order(&mut self, objective_description_order: u32) {
        self.objective_description_order = objective_description_order;
    }

    fn make_header(&self) -> u8 {
        self.make_header
    }

    fn set_make_header(&mut self, make_header: u8) {
        self.make_header = make_header;
    }

    fn short_description_string_table_id(&self) -> i32 {
        self.description_string_table_id
    }

    fn set_short_description_string_table_id(&mut self, short_description_string_table_id: i32) {
        self.short_description_string_table_id = short_description_string_table_id;
    }

    fn display_on_screen(&self) -> u8 {
        self.display_on_screen
    }

    fn set_display_on_screen(&mut self, display_on_screen: u8) {
        self.display_on_screen = display_on_screen;
    }

    fn unknown(&self) -> &Vec<u8> {
        &self.unknown
    }

    fn set_unknown(&mut self, unknown: Vec<u8>) {
        self.unknown = unknown;
    }

    fn mute_objectives(&self) -> u8 {
        self.mute_objectives
    }

    fn set_mute_objectives(&mut self, mute_objectives: u8) {
        self.mute_objectives = mute_objectives;
    }

    fn trigger_description(&self) -> &PrefixString<u32> {
        &self.trigger_description
    }

    fn set_trigger_description(&mut self, trigger_description: &str) {
        self.trigger_description.raw = trigger_description.to_string();
    }

    fn trigger_name(&self) -> &PrefixString<u32> {
        &self.trigger_name
    }

    fn set_trigger_name(&mut self, trigger_name: &str) {
        self.trigger_name.raw = trigger_name.to_string();
    }

    fn short_description(&self) -> &PrefixString<u32> {
        &self.short_description
    }

    fn set_short_description(&mut self, short_description: &str) {
        self.short_description.raw = short_description.to_string();
    }

    fn number_of_effects(&self) -> i32 {
        self.number_of_effects
    }

    fn set_number_of_effects(&mut self, number_of_effects: i32) {
        self.number_of_effects = number_of_effects;
    }

    fn effect_data(&self) -> &Vec<Box<dyn EffectDataIsh>> {
        &self.effect_data
    }

    fn effect_data_mut(&mut self) -> &mut Vec<Box<dyn EffectDataIsh>> {
        &mut self.effect_data
    }

    fn effect_display_order_array(&self) -> &Vec<i32> {
        &self.effect_display_order_array
    }

    fn effect_display_order_array_mut(&mut self) -> &mut Vec<i32> {
        &mut self.effect_display_order_array
    }

    fn number_of_conditions(&self) -> i32 {
        self.number_of_conditions
    }

    fn set_number_of_conditions(&mut self, number_of_conditions: i32) {
        self.number_of_conditions = number_of_conditions;
    }

    fn condition_data(&self) -> &Vec<Box<dyn ConditionDataIsh>> {
        &self.condition_data
    }

    fn condition_data_mut(&mut self) -> &mut Vec<Box<dyn ConditionDataIsh>> {
        &mut self.condition_data
    }

    fn condition_display_order(&self) -> &Vec<i32> {
        &self.condition_display_order
    }

    fn condition_display_order_mut(&mut self) -> &mut Vec<i32> {
        &mut self.condition_display_order
    }

    fn to_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut Serializer::convert_to_bytes(&self.enabled));
        buffer.append(&mut Serializer::convert_to_bytes(&self.looping));
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.description_string_table_id,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.display_as_objective,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.objective_description_order,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(&self.make_header));
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.short_description_string_table_id,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(&self.display_on_screen));
        buffer.append(&mut Serializer::convert_to_bytes(&self.unknown));
        buffer.append(&mut Serializer::convert_to_bytes(&self.mute_objectives));
        buffer.append(&mut Serializer::convert_to_bytes(&self.trigger_description));
        buffer.append(&mut Serializer::convert_to_bytes(&self.trigger_name));
        buffer.append(&mut Serializer::convert_to_bytes(&self.short_description));
        buffer.append(&mut Serializer::convert_to_bytes(&self.number_of_effects));
        for data in &self.effect_data {
            data.to_buffer(buffer);
        }
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.effect_display_order_array,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.number_of_conditions,
        ));
        for data in &self.condition_data {
            data.to_buffer(buffer);
        }
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.condition_display_order,
        ));
    }
}

pub struct EffectData {
    effect_type: i32,
    static_value_46: i32,
    ai_script_goal: i32,
    quatity: i32,
    tribue_list: i32,
    diplomacy: i32,
    number_of_units_selected: i32,
    legacy_location_object_reference: i32,
    object_list_unit_id: i32,
    source_player: i32,
    target_player: i32,
    technology: i32,
    string_id: i32,
    unknown_2: i32,
    display_time: i32,
    trigger_id: i32,
    location_x: i32,
    location_y: i32,
    area_x1: i32,
    area_y1: i32,
    area_x2: i32,
    area_y2: i32,
    object_group: i32,
    object_type: i32,
    instruction_panel_position: i32,
    attack_stance: i32,
    time_unit: i32,
    enabled: i32,
    food: i32,
    wood: i32,
    stone: i32,
    gold: i32,
    item_id: i32,
    flash_object: i32,
    force_research_technology: i32,
    visibility_state: i32,
    scroll: i32,
    operation: i32,
    object_list_unit_id_2: i32,
    button_location: i32,
    ai_signal_value: i32,
    unknown_3: i32,
    object_attributes: i32,
    variable: i32,
    timer: i32,
    facet: i32,
    location_object_reference: i32,
    play_sound: i32,
    player_color: i32,
    unknown_4: i32,
    color_mood: i32,
    reset_time: i32,
    object_state: i32,
    action_type: i32,
    message: PrefixString<u32>,
    sound_name: PrefixString<u32>,
    selected_object_ids: Vec<i32>,
}

impl EffectData {
    pub fn from_buffer(generator: &mut IncrementalGenerator) -> Self {
        let effect_type = generator.get_signed_int_32();
        let static_value_46 = generator.get_signed_int_32();
        let ai_script_goal = generator.get_signed_int_32();
        let quatity = generator.get_signed_int_32();
        let tribue_list = generator.get_signed_int_32();
        let diplomacy = generator.get_signed_int_32();
        let number_of_units_selected = generator.get_signed_int_32();
        let legacy_location_object_reference = generator.get_signed_int_32();
        let object_list_unit_id = generator.get_signed_int_32();
        let source_player = generator.get_signed_int_32();
        let target_player = generator.get_signed_int_32();
        let technology = generator.get_signed_int_32();
        let string_id = generator.get_signed_int_32();
        let unknown_2 = generator.get_signed_int_32();
        let display_time = generator.get_signed_int_32();
        let trigger_id = generator.get_signed_int_32();
        let location_x = generator.get_signed_int_32();
        let location_y = generator.get_signed_int_32();
        let area_x1 = generator.get_signed_int_32();
        let area_y1 = generator.get_signed_int_32();
        let area_x2 = generator.get_signed_int_32();
        let area_y2 = generator.get_signed_int_32();
        let object_group = generator.get_signed_int_32();
        let object_type = generator.get_signed_int_32();
        let instruction_panel_position = generator.get_signed_int_32();
        let attack_stance = generator.get_signed_int_32();
        let time_unit = generator.get_signed_int_32();
        let enabled = generator.get_signed_int_32();
        let food = generator.get_signed_int_32();
        let wood = generator.get_signed_int_32();
        let stone = generator.get_signed_int_32();
        let gold = generator.get_signed_int_32();
        let item_id = generator.get_signed_int_32();
        let flash_object = generator.get_signed_int_32();
        let force_research_technology = generator.get_signed_int_32();
        let visibility_state = generator.get_signed_int_32();
        let scroll = generator.get_signed_int_32();
        let operation = generator.get_signed_int_32();
        let object_list_unit_id_2 = generator.get_signed_int_32();
        let button_location = generator.get_signed_int_32();
        let ai_signal_value = generator.get_signed_int_32();
        let unknown_3 = generator.get_signed_int_32();
        let object_attributes = generator.get_signed_int_32();
        let variable = generator.get_signed_int_32();
        let timer = generator.get_signed_int_32();
        let facet = generator.get_signed_int_32();
        let location_object_reference = generator.get_signed_int_32();
        let play_sound = generator.get_signed_int_32();
        let player_color = generator.get_signed_int_32();
        let unknown_4 = generator.get_signed_int_32();
        let color_mood = generator.get_signed_int_32();
        let reset_time = generator.get_signed_int_32();
        let object_state = generator.get_signed_int_32();
        let action_type = generator.get_signed_int_32();
        let message = generator.get_str_32();
        let sound_name = generator.get_str_32();

        let mut selected_object_ids = Vec::<i32>::new();
        for _ in 0..number_of_units_selected {
            selected_object_ids.push(generator.get_signed_int_32());
        }

        EffectData {
            effect_type: effect_type,
            static_value_46: static_value_46,
            ai_script_goal: ai_script_goal,
            quatity: quatity,
            tribue_list: tribue_list,
            diplomacy: diplomacy,
            number_of_units_selected: number_of_units_selected,
            legacy_location_object_reference: legacy_location_object_reference,
            object_list_unit_id: object_list_unit_id,
            source_player: source_player,
            target_player: target_player,
            technology: technology,
            string_id: string_id,
            unknown_2: unknown_2,
            display_time: display_time,
            trigger_id: trigger_id,
            location_x: location_x,
            location_y: location_y,
            area_x1: area_x1,
            area_y1: area_y1,
            area_x2: area_x2,
            area_y2: area_y2,
            object_group: object_group,
            object_type: object_type,
            instruction_panel_position: instruction_panel_position,
            attack_stance: attack_stance,
            time_unit: time_unit,
            enabled: enabled,
            food: food,
            wood: wood,
            stone: stone,
            gold: gold,
            item_id: item_id,
            flash_object: flash_object,
            force_research_technology: force_research_technology,
            visibility_state: visibility_state,
            scroll: scroll,
            operation: operation,
            object_list_unit_id_2: object_list_unit_id_2,
            button_location: button_location,
            ai_signal_value: ai_signal_value,
            unknown_3: unknown_3,
            object_attributes: object_attributes,
            variable: variable,
            timer: timer,
            facet: facet,
            location_object_reference: location_object_reference,
            play_sound: play_sound,
            player_color: player_color,
            unknown_4: unknown_4,
            color_mood: color_mood,
            reset_time: reset_time,
            object_state: object_state,
            action_type: action_type,
            message: message,
            sound_name: sound_name,
            selected_object_ids: selected_object_ids,
        }
    }
}

impl EffectDataIsh for EffectData {
    fn effect_type(&self) -> i32 {
        self.effect_type
    }

    fn set_effect_type(&mut self, effect_type: i32) {
        self.effect_type = effect_type;
    }

    fn static_value_46(&self) -> i32 {
        self.static_value_46
    }

    fn set_static_value_46(&mut self, static_value: i32) {
        self.static_value_46 = static_value;
    }

    fn ai_script_goal(&self) -> i32 {
        self.ai_script_goal
    }

    fn set_ai_script_goal(&mut self, goal: i32) {
        self.ai_script_goal = goal;
    }

    fn quatity(&self) -> i32 {
        self.quatity
    }

    fn set_quatity(&mut self, quatity: i32) {
        self.quatity = quatity;
    }

    fn tribue_list(&self) -> i32 {
        self.tribue_list
    }

    fn set_tribue_list(&mut self, list: i32) {
        self.tribue_list = list;
    }

    fn diplomacy(&self) -> i32 {
        self.diplomacy
    }

    fn set_diplomacy(&mut self, diplomacy: i32) {
        self.diplomacy = diplomacy;
    }

    fn number_of_units_selected(&self) -> i32 {
        self.number_of_units_selected
    }

    fn set_number_of_units_selected(&mut self, number: i32) {
        self.number_of_units_selected = number;
    }

    fn legacy_location_object_reference(&self) -> i32 {
        self.legacy_location_object_reference
    }

    fn set_legacy_location_object_reference(&mut self, reference: i32) {
        self.legacy_location_object_reference = reference;
    }

    fn object_list_unit_id(&self) -> i32 {
        self.object_list_unit_id
    }

    fn set_object_list_unit_id(&mut self, id: i32) {
        self.object_list_unit_id = id;
    }

    fn source_player(&self) -> i32 {
        self.source_player
    }

    fn set_source_player(&mut self, source_player: i32) {
        self.source_player = source_player;
    }

    fn target_player(&self) -> i32 {
        self.target_player
    }

    fn set_target_player(&mut self, target_player: i32) {
        self.target_player = target_player;
    }

    fn technology(&self) -> i32 {
        self.technology
    }

    fn set_technology(&mut self, tech: i32) {
        self.technology = tech;
    }

    fn string_id(&self) -> i32 {
        self.string_id
    }

    fn set_string_id(&mut self, string_id: i32) {
        self.string_id = string_id;
    }

    fn unknown_2(&self) -> i32 {
        self.unknown_2
    }

    fn set_unknown_2(&mut self, unknown: i32) {
        self.unknown_2 = unknown;
    }

    fn display_time(&self) -> i32 {
        self.display_time
    }

    fn set_display_time(&mut self, display_time: i32) {
        self.display_time = display_time;
    }

    fn trigger_id(&self) -> i32 {
        self.trigger_id
    }

    fn set_trigger_id(&mut self, trigger_id: i32) {
        self.trigger_id = trigger_id;
    }

    fn location_x(&self) -> i32 {
        self.location_x
    }

    fn set_location_x(&mut self, location: i32) {
        self.location_x = location;
    }

    fn location_y(&self) -> i32 {
        self.location_y
    }

    fn set_location_y(&mut self, location: i32) {
        self.location_y = location;
    }

    fn area_x1(&self) -> i32 {
        self.area_x1
    }

    fn set_area_x1(&mut self, area_x1: i32) {
        self.area_x1 = area_x1;
    }

    fn area_y1(&self) -> i32 {
        self.area_y1
    }

    fn set_area_y1(&mut self, area_y1: i32) {
        self.area_y1 = area_y1;
    }

    fn area_x2(&self) -> i32 {
        self.area_x2
    }

    fn set_area_x2(&mut self, area_x2: i32) {
        self.area_x2 = area_x2;
    }

    fn area_y2(&self) -> i32 {
        self.area_y2
    }

    fn set_area_y2(&mut self, area_y2: i32) {
        self.area_y2 = area_y2;
    }

    fn object_group(&self) -> i32 {
        self.object_group
    }

    fn set_object_group(&mut self, object_group: i32) {
        self.object_group = object_group;
    }

    fn object_type(&self) -> i32 {
        self.object_type
    }

    fn set_object_type(&mut self, object_type: i32) {
        self.object_type = object_type;
    }

    fn instruction_panel_position(&self) -> i32 {
        self.instruction_panel_position
    }

    fn set_instruction_panel_position(&mut self, position: i32) {
        self.instruction_panel_position = position;
    }

    fn attack_stance(&self) -> i32 {
        self.attack_stance
    }

    fn set_attack_stance(&mut self, attack_stance: i32) {
        self.attack_stance = attack_stance;
    }

    fn time_unit(&self) -> i32 {
        self.time_unit
    }

    fn set_time_unit(&mut self, time_unit: i32) {
        self.time_unit = time_unit;
    }

    fn enabled(&self) -> i32 {
        self.enabled
    }

    fn set_enabled(&mut self, enable: i32) {
        self.enabled = enable;
    }

    fn food(&self) -> i32 {
        self.food
    }

    fn set_food(&mut self, food: i32) {
        self.food = food;
    }

    fn wood(&self) -> i32 {
        self.wood
    }

    fn set_wood(&mut self, wood: i32) {
        self.wood = wood;
    }

    fn stone(&self) -> i32 {
        self.stone
    }

    fn set_stone(&mut self, stone: i32) {
        self.stone = stone;
    }

    fn gold(&self) -> i32 {
        self.gold
    }

    fn set_gold(&mut self, gold: i32) {
        self.gold = gold;
    }

    fn item_id(&self) -> i32 {
        self.item_id
    }

    fn set_item_id(&mut self, item_id: i32) {
        self.item_id = item_id;
    }

    fn flash_object(&self) -> i32 {
        self.flash_object
    }

    fn set_flash_object(&mut self, flash_object: i32) {
        self.flash_object = flash_object;
    }

    fn force_research_technology(&self) -> i32 {
        self.force_research_technology
    }

    fn set_force_research_technology(&mut self, force_research_force: i32) {
        self.force_research_technology = force_research_force;
    }

    fn visibility_state(&self) -> i32 {
        self.visibility_state
    }

    fn set_visibility_state(&mut self, visibility_state: i32) {
        self.visibility_state = visibility_state;
    }

    fn scroll(&self) -> i32 {
        self.scroll
    }

    fn set_scroll(&mut self, scroll: i32) {
        self.scroll = scroll;
    }

    fn operation(&self) -> i32 {
        self.operation
    }

    fn set_operation(&mut self, operation: i32) {
        self.operation = operation;
    }

    fn object_list_unit_id_2(&self) -> i32 {
        self.object_list_unit_id_2
    }

    fn set_object_list_unit_id_2(&mut self, object_list_unit_id_2: i32) {
        self.object_list_unit_id_2 = object_list_unit_id_2;
    }

    fn button_location(&self) -> i32 {
        self.button_location
    }

    fn set_button_location(&mut self, button_location: i32) {
        self.button_location = button_location;
    }

    fn ai_signal_value(&self) -> i32 {
        self.ai_signal_value
    }

    fn set_ai_signal_value(&mut self, ai_signal_value: i32) {
        self.ai_signal_value = ai_signal_value;
    }

    fn unknown_3(&self) -> i32 {
        self.unknown_3
    }

    fn set_unknown_3(&mut self, unknown: i32) {
        self.unknown_3 = unknown;
    }

    fn object_attributes(&self) -> i32 {
        self.object_attributes
    }

    fn set_object_attributes(&mut self, object_attributes: i32) {
        self.object_attributes = object_attributes;
    }

    fn variable(&self) -> i32 {
        self.variable
    }

    fn set_variable(&mut self, variable: i32) {
        self.variable = variable;
    }

    fn timer(&self) -> i32 {
        self.timer
    }

    fn set_timer(&mut self, timer: i32) {
        self.timer = timer;
    }

    fn facet(&self) -> i32 {
        self.facet
    }

    fn set_facet(&mut self, facet: i32) {
        self.facet = facet;
    }

    fn location_object_reference(&self) -> i32 {
        self.location_object_reference
    }

    fn set_location_object_reference(&mut self, reference: i32) {
        self.location_object_reference = reference;
    }

    fn play_sound(&self) -> i32 {
        self.play_sound
    }

    fn set_play_sound(&mut self, play_sound: i32) {
        self.play_sound = play_sound;
    }

    fn player_color(&self) -> i32 {
        self.player_color
    }

    fn set_player_color(&mut self, player_color: i32) {
        self.player_color = player_color;
    }

    fn unknown_4(&self) -> i32 {
        self.unknown_4
    }

    fn set_unknown_4(&mut self, unknown: i32) {
        self.unknown_4 = unknown;
    }

    fn color_mood(&self) -> i32 {
        self.color_mood
    }

    fn set_color_mood(&mut self, color_mood: i32) {
        self.color_mood = color_mood;
    }

    fn reset_time(&self) -> i32 {
        self.reset_time
    }

    fn set_reset_time(&mut self, reset_time: i32) {
        self.reset_time = reset_time;
    }

    fn object_state(&self) -> i32 {
        self.object_state
    }

    fn set_object_state(&mut self, object_state: i32) {
        self.object_state = object_state;
    }

    fn action_type(&self) -> i32 {
        self.action_type
    }

    fn set_action_type(&mut self, action_type: i32) {
        self.action_type = action_type;
    }

    fn message(&self) -> &PrefixString<u32> {
        &self.message
    }

    fn set_message(&mut self, message: &str) {
        self.message.raw = message.to_string();
    }

    fn sound_name(&self) -> &PrefixString<u32> {
        &self.sound_name
    }

    fn set_sound_name(&mut self, sound_name: &str) {
        self.sound_name.raw = sound_name.to_string();
    }

    fn selected_object_ids(&self) -> &Vec<i32> {
        &self.selected_object_ids
    }

    fn selected_object_ids_mut(&mut self) -> &mut Vec<i32> {
        &mut self.selected_object_ids
    }

    fn to_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut Serializer::convert_to_bytes(&self.effect_type));
        buffer.append(&mut Serializer::convert_to_bytes(&self.static_value_46));
        buffer.append(&mut Serializer::convert_to_bytes(&self.ai_script_goal));
        buffer.append(&mut Serializer::convert_to_bytes(&self.quatity));
        buffer.append(&mut Serializer::convert_to_bytes(&self.tribue_list));
        buffer.append(&mut Serializer::convert_to_bytes(&self.diplomacy));
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.number_of_units_selected,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.legacy_location_object_reference,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(&self.object_list_unit_id));
        buffer.append(&mut Serializer::convert_to_bytes(&self.source_player));
        buffer.append(&mut Serializer::convert_to_bytes(&self.target_player));
        buffer.append(&mut Serializer::convert_to_bytes(&self.technology));
        buffer.append(&mut Serializer::convert_to_bytes(&self.string_id));
        buffer.append(&mut Serializer::convert_to_bytes(&self.unknown_2));
        buffer.append(&mut Serializer::convert_to_bytes(&self.display_time));
        buffer.append(&mut Serializer::convert_to_bytes(&self.trigger_id));
        buffer.append(&mut Serializer::convert_to_bytes(&self.location_x));
        buffer.append(&mut Serializer::convert_to_bytes(&self.location_y));
        buffer.append(&mut Serializer::convert_to_bytes(&self.area_x1));
        buffer.append(&mut Serializer::convert_to_bytes(&self.area_y1));
        buffer.append(&mut Serializer::convert_to_bytes(&self.area_x2));
        buffer.append(&mut Serializer::convert_to_bytes(&self.area_y2));
        buffer.append(&mut Serializer::convert_to_bytes(&self.object_group));
        buffer.append(&mut Serializer::convert_to_bytes(&self.object_type));
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.instruction_panel_position,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(&self.attack_stance));
        buffer.append(&mut Serializer::convert_to_bytes(&self.time_unit));
        buffer.append(&mut Serializer::convert_to_bytes(&self.enabled));
        buffer.append(&mut Serializer::convert_to_bytes(&self.food));
        buffer.append(&mut Serializer::convert_to_bytes(&self.wood));
        buffer.append(&mut Serializer::convert_to_bytes(&self.stone));
        buffer.append(&mut Serializer::convert_to_bytes(&self.gold));
        buffer.append(&mut Serializer::convert_to_bytes(&self.item_id));
        buffer.append(&mut Serializer::convert_to_bytes(&self.flash_object));
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.force_research_technology,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(&self.visibility_state));
        buffer.append(&mut Serializer::convert_to_bytes(&self.scroll));
        buffer.append(&mut Serializer::convert_to_bytes(&self.operation));
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.object_list_unit_id_2,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(&self.button_location));
        buffer.append(&mut Serializer::convert_to_bytes(&self.ai_signal_value));
        buffer.append(&mut Serializer::convert_to_bytes(&self.unknown_3));
        buffer.append(&mut Serializer::convert_to_bytes(&self.object_attributes));
        buffer.append(&mut Serializer::convert_to_bytes(&self.variable));
        buffer.append(&mut Serializer::convert_to_bytes(&self.timer));
        buffer.append(&mut Serializer::convert_to_bytes(&self.facet));
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.location_object_reference,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(&self.play_sound));
        buffer.append(&mut Serializer::convert_to_bytes(&self.player_color));
        buffer.append(&mut Serializer::convert_to_bytes(&self.unknown_4));
        buffer.append(&mut Serializer::convert_to_bytes(&self.color_mood));
        buffer.append(&mut Serializer::convert_to_bytes(&self.reset_time));
        buffer.append(&mut Serializer::convert_to_bytes(&self.object_state));
        buffer.append(&mut Serializer::convert_to_bytes(&self.action_type));
        buffer.append(&mut Serializer::convert_to_bytes(&self.message));
        buffer.append(&mut Serializer::convert_to_bytes(&self.sound_name));
        buffer.append(&mut Serializer::convert_to_bytes(&self.selected_object_ids));
    }
}

pub struct ConditionData {
    condition_type: i32,
    static_value_21: i32,
    quantity: i32,
    attribute: i32,
    unit_object: i32,
    next_object: i32,
    object_list: i32,
    source_player: i32,
    technology: i32,
    timer: i32,
    unknown: i32,
    area_x1: i32,
    area_y1: i32,
    area_x2: i32,
    area_y2: i32,
    object_group: i32,
    object_type: i32,
    ai_signal: i32,
    inverted: i32,
    unknown_2: i32,
    variable: i32,
    comparison: i32,
    target_player: i32,
    unit_ai_action: i32,
    unknown_4: i32,
    object_state: i32,
    timer_id: i32,
    victory_timer_type: i32,
    include_changeable_weapon_objects: i32,
    xs_function: PrefixString<u32>,
}

impl ConditionData {
    pub fn from_buffer(generator: &mut IncrementalGenerator) -> Self {
        ConditionData {
            condition_type: generator.get_signed_int_32(),
            static_value_21: generator.get_signed_int_32(),
            quantity: generator.get_signed_int_32(),
            attribute: generator.get_signed_int_32(),
            unit_object: generator.get_signed_int_32(),
            next_object: generator.get_signed_int_32(),
            object_list: generator.get_signed_int_32(),
            source_player: generator.get_signed_int_32(),
            technology: generator.get_signed_int_32(),
            timer: generator.get_signed_int_32(),
            unknown: generator.get_signed_int_32(),
            area_x1: generator.get_signed_int_32(),
            area_y1: generator.get_signed_int_32(),
            area_x2: generator.get_signed_int_32(),
            area_y2: generator.get_signed_int_32(),
            object_group: generator.get_signed_int_32(),
            object_type: generator.get_signed_int_32(),
            ai_signal: generator.get_signed_int_32(),
            inverted: generator.get_signed_int_32(),
            unknown_2: generator.get_signed_int_32(),
            variable: generator.get_signed_int_32(),
            comparison: generator.get_signed_int_32(),
            target_player: generator.get_signed_int_32(),
            unit_ai_action: generator.get_signed_int_32(),
            unknown_4: generator.get_signed_int_32(),
            object_state: generator.get_signed_int_32(),
            timer_id: generator.get_signed_int_32(),
            victory_timer_type: generator.get_signed_int_32(),
            include_changeable_weapon_objects: generator.get_signed_int_32(),
            xs_function: generator.get_str_32(),
        }
    }
}

impl ConditionDataIsh for ConditionData {
    fn condition_type(&self) -> i32 {
        self.condition_type
    }

    fn set_condition_type(&mut self, condition_type: i32) {
        self.condition_type = condition_type;
    }

    fn static_value_21(&self) -> i32 {
        self.static_value_21
    }

    fn set_static_value_21(&mut self, static_value_21: i32) {
        self.static_value_21 = static_value_21
    }

    fn quantity(&self) -> i32 {
        self.quantity
    }

    fn set_quantity(&mut self, quantity: i32) {
        self.quantity = quantity;
    }

    fn attribute(&self) -> i32 {
        self.attribute
    }

    fn set_attribute(&mut self, attribute: i32) {
        self.attribute = attribute;
    }

    fn unit_object(&self) -> i32 {
        self.unit_object
    }

    fn set_unit_object(&mut self, unit_object: i32) {
        self.unit_object = unit_object;
    }

    fn next_object(&self) -> i32 {
        self.next_object
    }

    fn set_next_object(&mut self, next_object: i32) {
        self.next_object = next_object;
    }

    fn object_list(&self) -> i32 {
        self.object_list
    }

    fn set_object_list(&mut self, object_list: i32) {
        self.object_list = object_list;
    }

    fn source_player(&self) -> i32 {
        self.source_player
    }

    fn set_source_player(&mut self, source_player: i32) {
        self.source_player = source_player;
    }

    fn technology(&self) -> i32 {
        self.technology
    }

    fn set_technology(&mut self, technology: i32) {
        self.technology = technology;
    }

    fn timer(&self) -> i32 {
        self.timer
    }

    fn set_timer(&mut self, timer: i32) {
        self.timer = timer
    }

    fn unknown(&self) -> i32 {
        self.unknown
    }

    fn set_unknown(&mut self, unknown: i32) {
        self.unknown = unknown;
    }

    fn area_x1(&self) -> i32 {
        self.area_x1
    }

    fn set_area_x1(&mut self, area_x1: i32) {
        self.area_x1 = area_x1;
    }

    fn area_y1(&self) -> i32 {
        self.area_y1
    }

    fn set_area_y1(&mut self, area_y1: i32) {
        self.area_y1 = area_y1;
    }

    fn area_x2(&self) -> i32 {
        self.area_x2
    }

    fn set_area_x2(&mut self, area_x2: i32) {
        self.area_x2 = area_x2;
    }

    fn area_y2(&self) -> i32 {
        self.area_y2
    }

    fn set_area_y2(&mut self, area_y2: i32) {
        self.area_y2 = area_y2;
    }

    fn object_group(&self) -> i32 {
        self.object_group
    }

    fn set_object_group(&mut self, object_group: i32) {
        self.object_group = object_group;
    }

    fn object_type(&self) -> i32 {
        self.object_type
    }

    fn set_object_type(&mut self, object_type: i32) {
        self.object_type = object_type;
    }

    fn ai_signal(&self) -> i32 {
        self.ai_signal
    }

    fn set_ai_signal(&mut self, ai_signal: i32) {
        self.ai_signal = ai_signal;
    }

    fn inverted(&self) -> i32 {
        self.inverted
    }

    fn set_inverted(&mut self, inverted: i32) {
        self.inverted = inverted;
    }

    fn unknown_2(&self) -> i32 {
        self.unknown_2
    }

    fn set_unknown_2(&mut self, unknown_2: i32) {
        self.unknown_2 = unknown_2;
    }

    fn variable(&self) -> i32 {
        self.variable
    }

    fn set_variable(&mut self, variable: i32) {
        self.variable = variable;
    }

    fn comparison(&self) -> i32 {
        self.comparison
    }

    fn set_comparision(&mut self, comparison: i32) {
        self.comparison = comparison;
    }

    fn target_player(&self) -> i32 {
        self.target_player
    }

    fn set_target_player(&mut self, target_player: i32) {
        self.target_player = target_player;
    }

    fn unit_ai_action(&self) -> i32 {
        self.unit_ai_action
    }

    fn set_unit_ai_action(&mut self, unit_ai_action: i32) {
        self.unit_ai_action = unit_ai_action;
    }

    fn unknown_4(&self) -> i32 {
        self.unknown_4
    }

    fn set_unknown_4(&mut self, unknown_4: i32) {
        self.unknown_4 = unknown_4;
    }

    fn object_state(&self) -> i32 {
        self.object_state
    }

    fn set_object_state(&mut self, object_state: i32) {
        self.object_state = object_state;
    }

    fn timer_id(&self) -> i32 {
        self.timer_id
    }

    fn set_timer_id(&mut self, timer_id: i32) {
        self.timer_id = timer_id;
    }

    fn victory_timer_type(&self) -> i32 {
        self.victory_timer_type
    }

    fn set_victory_timer_type(&mut self, victory_timer_type: i32) {
        self.victory_timer_type = victory_timer_type;
    }

    fn include_changeable_weapon_objects(&self) -> i32 {
        self.include_changeable_weapon_objects
    }

    fn set_include_changeable_wapon_objects(&mut self, include_changeable_weapon_objects: i32) {
        self.include_changeable_weapon_objects = include_changeable_weapon_objects;
    }

    fn xs_function(&self) -> &PrefixString<u32> {
        &self.xs_function
    }

    fn set_xs_function(&mut self, xs_function: &str) {
        self.xs_function.raw = xs_function.to_string();
    }

    fn to_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut Serializer::convert_to_bytes(&self.condition_type));
        buffer.append(&mut Serializer::convert_to_bytes(&self.static_value_21));
        buffer.append(&mut Serializer::convert_to_bytes(&self.quantity));
        buffer.append(&mut Serializer::convert_to_bytes(&self.attribute));
        buffer.append(&mut Serializer::convert_to_bytes(&self.unit_object));
        buffer.append(&mut Serializer::convert_to_bytes(&self.next_object));
        buffer.append(&mut Serializer::convert_to_bytes(&self.object_list));
        buffer.append(&mut Serializer::convert_to_bytes(&self.source_player));
        buffer.append(&mut Serializer::convert_to_bytes(&self.technology));
        buffer.append(&mut Serializer::convert_to_bytes(&self.timer));
        buffer.append(&mut Serializer::convert_to_bytes(&self.unknown));
        buffer.append(&mut Serializer::convert_to_bytes(&self.area_x1));
        buffer.append(&mut Serializer::convert_to_bytes(&self.area_y1));
        buffer.append(&mut Serializer::convert_to_bytes(&self.area_x2));
        buffer.append(&mut Serializer::convert_to_bytes(&self.area_y2));
        buffer.append(&mut Serializer::convert_to_bytes(&self.object_group));
        buffer.append(&mut Serializer::convert_to_bytes(&self.object_type));
        buffer.append(&mut Serializer::convert_to_bytes(&self.ai_signal));
        buffer.append(&mut Serializer::convert_to_bytes(&self.inverted));
        buffer.append(&mut Serializer::convert_to_bytes(&self.unknown_2));
        buffer.append(&mut Serializer::convert_to_bytes(&self.variable));
        buffer.append(&mut Serializer::convert_to_bytes(&self.comparison));
        buffer.append(&mut Serializer::convert_to_bytes(&self.target_player));
        buffer.append(&mut Serializer::convert_to_bytes(&self.unit_ai_action));
        buffer.append(&mut Serializer::convert_to_bytes(&self.unknown_4));
        buffer.append(&mut Serializer::convert_to_bytes(&self.object_state));
        buffer.append(&mut Serializer::convert_to_bytes(&self.timer_id));
        buffer.append(&mut Serializer::convert_to_bytes(&self.victory_timer_type));
        buffer.append(&mut Serializer::convert_to_bytes(
            &self.include_changeable_weapon_objects,
        ));
        buffer.append(&mut Serializer::convert_to_bytes(&self.xs_function));
    }
}

pub struct Variable {
    variable_id: u32,
    variable_name: PrefixString<u32>,
}

impl Variable {
    pub fn from_buffer(generator: &mut IncrementalGenerator) -> Self {
        Variable {
            variable_id: generator.get_unsigned_int_32(),
            variable_name: generator.get_str_32(),
        }
    }
}

impl VariableIsh for Variable {
    fn variable_id(&self) -> u32 {
        self.variable_id
    }

    fn set_variable_id(&mut self, variable_id: u32) {
        self.variable_id = variable_id;
    }

    fn variable_name(&self) -> &PrefixString<u32> {
        &self.variable_name
    }

    fn set_variable_name(&mut self, variable_name: &str) {
        self.variable_name.raw = variable_name.to_string();
    }

    fn to_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut Serializer::convert_to_bytes(&self.variable_id));
        buffer.append(&mut Serializer::convert_to_bytes(&self.variable_name));
    }
}

pub struct Files {
    script_file_path: PrefixString<u16>,
    script_file_content: PrefixString<u32>,
    ai_files_present: u32,
    unknown: Vec<u8>,
    number_of_ai_files: Vec<u32>,
    ai_files: Vec<Box<dyn AI2Ish>>,
}

impl Files {
    pub fn from_buffer(generator: &mut IncrementalGenerator) -> Self {
        let script_file_path = generator.get_str_16();
        let script_file_content = generator.get_str_32();
        let ai_files_present = generator.get_unsigned_int_32();
        let unknown = generator.skip_bytes(4);

        let mut number_of_ai_files = Vec::<u32>::new();
        for _ in 0..ai_files_present {
            number_of_ai_files.push(generator.get_unsigned_int_32());
        }

        let mut ai_files = Vec::<Box<dyn AI2Ish>>::new();
        let count = if ai_files_present > 0 {
            number_of_ai_files[0]
        } else {
            0
        };

        for _ in 0..count {
            ai_files.push(Box::new(AI2::from_buffer(generator)))
        }

        Files {
            script_file_path: script_file_path,
            script_file_content: script_file_content,
            ai_files_present: ai_files_present,
            unknown: unknown,
            number_of_ai_files: number_of_ai_files,
            ai_files: ai_files,
        }
    }
}

impl FilesIsh for Files {
    fn script_file_path(&self) -> &PrefixString<u16> {
        &self.script_file_path
    }

    fn set_script_file_path(&mut self, script_file_path: &str) {
        self.script_file_path.raw = script_file_path.to_string();
    }

    fn script_file_content(&self) -> &PrefixString<u32> {
        &self.script_file_content
    }

    fn set_script_file_content(&mut self, script_file_content: &str) {
        self.script_file_content.raw = script_file_content.to_string();
    }

    fn ai_files_present(&self) -> u32 {
        self.ai_files_present
    }

    fn set_ai_files_present(&mut self, ai_files_present: u32) {
        self.ai_files_present = ai_files_present;
    }

    fn unknown(&self) -> &Vec<u8> {
        &self.unknown
    }

    fn unknown_mut(&mut self) -> &mut Vec<u8> {
        &mut self.unknown
    }

    fn number_of_ai_files(&self) -> &Vec<u32> {
        &self.number_of_ai_files
    }

    fn number_of_ai_files_mut(&mut self) -> &mut Vec<u32> {
        &mut self.number_of_ai_files
    }

    fn ai_files(&self) -> &Vec<Box<dyn AI2Ish>> {
        &self.ai_files
    }

    fn ai_file_mut(&mut self) -> &mut Vec<Box<dyn AI2Ish>> {
        &mut self.ai_files
    }

    fn to_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut Serializer::convert_to_bytes(&self.script_file_path));
        buffer.append(&mut Serializer::convert_to_bytes(&self.script_file_content));
        buffer.append(&mut Serializer::convert_to_bytes(&self.ai_files_present));
        buffer.append(&mut Serializer::convert_to_bytes(&self.unknown));
        buffer.append(&mut Serializer::convert_to_bytes(&self.number_of_ai_files));
        for file in &self.ai_files {
            file.to_buffer(buffer);
        }
    }
}

pub struct AI2 {
    ai_file_name: PrefixString<u32>,
    ai_file: PrefixString<u32>,
}

impl AI2 {
    pub fn from_buffer(generator: &mut IncrementalGenerator) -> Self {
        AI2 {
            ai_file_name: generator.get_str_32(),
            ai_file: generator.get_str_32(),
        }
    }
}

impl AI2Ish for AI2 {
    fn ai_file_name(&self) -> &PrefixString<u32> {
        &self.ai_file_name
    }

    fn set_ai_file_name(&mut self, ai_file_name: &str) {
        self.ai_file_name.raw = ai_file_name.to_string();
    }

    fn ai_file(&self) -> &PrefixString<u32> {
        &self.ai_file
    }

    fn set_ai_file(&mut self, ai_file: &str) {
        self.ai_file.raw = ai_file.to_string();
    }

    fn to_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut Serializer::convert_to_bytes(&self.ai_file_name));
        buffer.append(&mut Serializer::convert_to_bytes(&self.ai_file));
    }
}
