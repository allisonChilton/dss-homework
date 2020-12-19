#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct HomeData {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Data {
    #[serde(rename = "StandardCollection")]
    pub standard_collection: StandardCollection,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct StandardCollection {
    #[serde(rename = "callToAction")]
    pub call_to_action: ::serde_json::Value,
    #[serde(rename = "collectionGroup")]
    pub collection_group: CollectionGroup,
    #[serde(rename = "collectionId")]
    pub collection_id: String,
    pub containers: Vec<Container>,
    pub image: Image2,
    pub text: Text3,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "videoArt")]
    pub video_art: Vec<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CollectionGroup {
    #[serde(rename = "collectionGroupId")]
    pub collection_group_id: String,
    #[serde(rename = "contentClass")]
    pub content_class: String,
    pub key: String,
    pub slugs: Vec<Slug>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Slug {
    pub language: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Container {
    pub set: Set,
    #[serde(rename = "type")]
    pub type_field: String,
    pub style: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Set {
    #[serde(rename = "contentClass")]
    pub content_class: String,
    #[serde(default)]
    pub items: Vec<Item>,
    pub meta: Option<Meta>,
    #[serde(rename = "setId")]
    pub set_id: Option<String>,
    pub text: Text2,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "refId")]
    pub ref_id: Option<String>,
    #[serde(rename = "refIdType")]
    pub ref_id_type: Option<String>,
    #[serde(rename = "refType")]
    pub ref_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Item {
    #[serde(rename = "contentId")]
    pub content_id: Option<String>,
    #[serde(rename = "callToAction")]
    pub call_to_action: ::serde_json::Value,
    #[serde(rename = "currentAvailability")]
    pub current_availability: Option<CurrentAvailability>,
    #[serde(rename = "encodedSeriesId")]
    pub encoded_series_id: Option<String>,
    pub image: Image,
    #[serde(rename = "seriesId")]
    pub series_id: Option<String>,
    pub text: Text,
    #[serde(rename = "textExperienceId")]
    pub text_experience_id: Option<String>,
    #[serde(default)]
    pub tags: Vec<Tag>,
    #[serde(rename = "mediaRights")]
    pub media_rights: Option<MediaRights>,
    #[serde(default)]
    pub ratings: Vec<Rating>,
    #[serde(default)]
    pub releases: Vec<Release>,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "videoArt")]
    pub video_art: Vec<VideoArt>,
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
    #[serde(rename = "episodeNumber")]
    pub episode_number: Option<::serde_json::Value>,
    #[serde(rename = "episodeSequenceNumber")]
    pub episode_sequence_number: Option<::serde_json::Value>,
    #[serde(rename = "episodeSeriesSequenceNumber")]
    pub episode_series_sequence_number: Option<::serde_json::Value>,
    pub family: Option<Family>,
    #[serde(default)]
    pub groups: Vec<Group>,
    #[serde(rename = "internalTitle")]
    pub internal_title: Option<String>,
    #[serde(rename = "mediaMetadata")]
    pub media_metadata: Option<MediaMetadata2>,
    #[serde(rename = "originalLanguage")]
    pub original_language: Option<String>,
    #[serde(rename = "programId")]
    pub program_id: Option<String>,
    #[serde(rename = "programType")]
    pub program_type: Option<String>,
    #[serde(rename = "seasonId")]
    pub season_id: Option<::serde_json::Value>,
    #[serde(rename = "seasonSequenceNumber")]
    pub season_sequence_number: Option<::serde_json::Value>,
    #[serde(rename = "targetLanguage")]
    pub target_language: Option<String>,
    #[serde(rename = "videoId")]
    pub video_id: Option<String>,
    #[serde(rename = "collectionGroup")]
    pub collection_group: Option<CollectionGroup2>,
    #[serde(rename = "collectionId")]
    pub collection_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CurrentAvailability {
    pub region: String,
    #[serde(rename = "kidsMode")]
    pub kids_mode: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Image {
    pub tile: Tile,
    pub title_treatment_layer: Option<TitleTreatmentLayer>,
    pub title_treatment: Option<TitleTreatment>,
    pub background: Option<Background>,
    pub background_details: Option<BackgroundDetails>,
    pub hero_collection: HeroCollection,
    pub hero_tile: Option<HeroTile>,
    pub logo: Option<Logo>,
    pub logo_layer: Option<LogoLayer>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Tile {
    #[serde(rename = "2.29")]
    pub n229: Option<N229>,
    #[serde(rename = "1.78")]
    pub n178: N178,
    #[serde(rename = "0.71")]
    pub n071: N071,
    #[serde(rename = "0.75")]
    pub n075: Option<N075>,
    #[serde(rename = "0.67")]
    pub n067: Option<N067>,
    #[serde(rename = "1.00")]
    pub n100: Option<N100>,
    #[serde(rename = "1.33")]
    pub n133: Option<N133>,
    #[serde(rename = "1.20")]
    pub n120: Option<N120>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N229 {
    pub program: Option<Program>,
    pub series: Option<Series>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program {
    pub default: Default,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series {
    pub default: Default2,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default2 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N178 {
    pub series: Option<Series2>,
    pub program: Option<Program2>,
    pub default: Option<Default5>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series2 {
    pub default: Default3,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default3 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program2 {
    pub default: Default4,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default4 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default5 {
    pub default: Default6,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default6 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N071 {
    pub series: Option<Series3>,
    pub program: Option<Program3>,
    pub default: Option<Default9>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series3 {
    pub default: Default7,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default7 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program3 {
    pub default: Default8,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default8 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default9 {
    pub default: Default10,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default10 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N075 {
    pub program: Option<Program4>,
    pub series: Option<Series4>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program4 {
    pub default: Default11,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default11 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series4 {
    pub default: Default12,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default12 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N067 {
    pub program: Option<Program5>,
    pub series: Option<Series5>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program5 {
    pub default: Default13,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default13 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series5 {
    pub default: Default14,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default14 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N100 {
    pub program: Option<Program6>,
    pub series: Option<Series6>,
    pub default: Option<Default17>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program6 {
    pub default: Default15,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default15 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series6 {
    pub default: Default16,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default16 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default17 {
    pub default: Default18,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default18 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N133 {
    pub program: Option<Program7>,
    pub series: Option<Series7>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program7 {
    pub default: Default19,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default19 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series7 {
    pub default: Default20,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default20 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N120 {
    pub default: Default21,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default21 {
    pub default: Default22,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default22 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct TitleTreatmentLayer {
    #[serde(rename = "1.78")]
    pub n178: N1782,
    #[serde(rename = "3.91")]
    pub n391: N391,
    #[serde(rename = "3.00")]
    pub n300: N300,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N1782 {
    pub series: Option<Series8>,
    pub program: Option<Program8>,
    pub default: Option<Default25>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series8 {
    pub default: Default23,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default23 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program8 {
    pub default: Default24,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default24 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default25 {
    pub default: Default26,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default26 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N391 {
    pub series: Option<Series9>,
    pub program: Option<Program9>,
    pub default: Option<Default29>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series9 {
    pub default: Default27,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default27 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program9 {
    pub default: Default28,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default28 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default29 {
    pub default: Default30,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default30 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N300 {
    pub series: Option<Series10>,
    pub program: Option<Program10>,
    pub default: Option<Default33>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series10 {
    pub default: Default31,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default31 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program10 {
    pub default: Default32,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default32 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default33 {
    pub default: Default34,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default34 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct TitleTreatment {
    #[serde(rename = "3.32")]
    pub n332: N332,
    #[serde(rename = "1.78")]
    pub n178: N1783,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N332 {
    pub series: Option<Series11>,
    pub program: Option<Program11>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series11 {
    pub default: Default35,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default35 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program11 {
    pub default: Default36,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default36 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N1783 {
    pub series: Option<Series12>,
    pub program: Option<Program12>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series12 {
    pub default: Default37,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default37 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program12 {
    pub default: Default38,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default38 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Background {
    #[serde(rename = "2.89")]
    pub n289: Option<N289>,
    #[serde(rename = "1.78")]
    pub n178: N1784,
    #[serde(rename = "1.33")]
    pub n133: Option<N1332>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N289 {
    pub program: Option<Program13>,
    pub series: Option<Series13>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program13 {
    pub default: Default39,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default39 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series13 {
    pub default: Default40,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default40 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N1784 {
    pub program: Option<Program14>,
    pub series: Option<Series14>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program14 {
    pub default: Default41,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default41 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series14 {
    pub default: Default42,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default42 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N1332 {
    pub series: Option<Series15>,
    pub program: Option<Program15>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series15 {
    pub default: Default43,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default43 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program15 {
    pub default: Default44,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default44 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BackgroundDetails {
    #[serde(rename = "1.33")]
    pub n133: N1333,
    #[serde(rename = "1.78")]
    pub n178: N1785,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N1333 {
    pub program: Option<Program16>,
    pub series: Option<Series16>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program16 {
    pub default: Default45,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default45 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series16 {
    pub default: Default46,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default46 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N1785 {
    pub program: Option<Program17>,
    pub series: Option<Series17>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program17 {
    pub default: Default47,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default47 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series17 {
    pub default: Default48,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default48 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct HeroCollection {
    #[serde(rename = "1.78")]
    pub n178: N1786,
    #[serde(rename = "0.71")]
    pub n071: Option<N0712>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N1786 {
    pub series: Option<Series18>,
    pub program: Option<Program18>,
    pub default: Option<Default51>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series18 {
    pub default: Default49,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default49 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program18 {
    pub default: Default50,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default50 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default51 {
    pub default: Default52,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default52 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N0712 {
    pub default: Default53,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default53 {
    pub default: Default54,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default54 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct HeroTile {
    #[serde(rename = "3.91")]
    pub n391: N3912,
    #[serde(rename = "3.00")]
    pub n300: N3002,
    #[serde(rename = "1.78")]
    pub n178: N1787,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N3912 {
    pub series: Option<Series19>,
    pub program: Option<Program19>,
    pub default: Option<Default57>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series19 {
    pub default: Default55,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default55 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program19 {
    pub default: Default56,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default56 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default57 {
    pub default: Default58,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default58 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N3002 {
    pub series: Option<Series20>,
    pub program: Option<Program20>,
    pub default: Option<Default61>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series20 {
    pub default: Default59,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default59 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program20 {
    pub default: Default60,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default60 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default61 {
    pub default: Default62,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default62 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N1787 {
    pub series: Option<Series21>,
    pub program: Option<Program21>,
    pub default: Option<Default65>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series21 {
    pub default: Default63,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default63 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program21 {
    pub default: Default64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default64 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default65 {
    pub default: Default66,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default66 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Logo {
    #[serde(rename = "2.00")]
    pub n200: N200,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N200 {
    pub default: Default67,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default67 {
    pub default: Default68,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default68 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct LogoLayer {
    #[serde(rename = "1.78")]
    pub n178: N1788,
    #[serde(rename = "3.00")]
    pub n300: N3003,
    #[serde(rename = "3.91")]
    pub n391: N3913,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N1788 {
    pub default: Default69,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default69 {
    pub default: Default70,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default70 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N3003 {
    pub default: Default71,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default71 {
    pub default: Default72,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default72 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N3913 {
    pub default: Default73,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default73 {
    pub default: Default74,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default74 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Text {
    pub title: Title,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Title {
    pub slug: Option<Slug2>,
    pub full: Full,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Slug2 {
    pub series: Option<Series22>,
    pub program: Option<Program22>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series22 {
    pub default: Default75,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default75 {
    pub content: String,
    pub language: String,
    #[serde(rename = "sourceEntity")]
    pub source_entity: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program22 {
    pub default: Default76,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default76 {
    pub content: String,
    pub language: String,
    #[serde(rename = "sourceEntity")]
    pub source_entity: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Full {
    pub series: Option<Series23>,
    pub program: Option<Program23>,
    pub collection: Option<Collection>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series23 {
    pub default: Default77,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default77 {
    pub content: String,
    pub language: String,
    #[serde(rename = "sourceEntity")]
    pub source_entity: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program23 {
    pub default: Default78,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default78 {
    pub content: String,
    pub language: String,
    #[serde(rename = "sourceEntity")]
    pub source_entity: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Collection {
    pub default: Default79,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default79 {
    pub content: String,
    pub language: String,
    #[serde(rename = "sourceEntity")]
    pub source_entity: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Tag {
    #[serde(rename = "displayName")]
    pub display_name: ::serde_json::Value,
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct MediaRights {
    #[serde(rename = "downloadBlocked")]
    pub download_blocked: bool,
    #[serde(rename = "pconBlocked")]
    pub pcon_blocked: bool,
    #[serde(default)]
    pub violations: Vec<::serde_json::Value>,
    pub rewind: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Rating {
    pub advisories: Vec<::serde_json::Value>,
    pub description: Option<String>,
    pub system: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Release {
    #[serde(rename = "releaseDate")]
    pub release_date: Option<String>,
    #[serde(rename = "releaseType")]
    pub release_type: String,
    #[serde(rename = "releaseYear")]
    pub release_year: i64,
    pub territory: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct VideoArt {
    #[serde(rename = "mediaMetadata")]
    pub media_metadata: MediaMetadata,
    pub purpose: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct MediaMetadata {
    pub urls: Vec<Url>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Url {
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Family {
    #[serde(rename = "encodedFamilyId")]
    pub encoded_family_id: String,
    #[serde(rename = "familyId")]
    pub family_id: String,
    pub parent: bool,
    #[serde(rename = "parentRef")]
    pub parent_ref: ParentRef,
    #[serde(rename = "sequenceNumber")]
    pub sequence_number: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ParentRef {
    #[serde(rename = "encodedSeriesId")]
    pub encoded_series_id: ::serde_json::Value,
    #[serde(rename = "programId")]
    pub program_id: String,
    #[serde(rename = "seasonId")]
    pub season_id: ::serde_json::Value,
    #[serde(rename = "seriesId")]
    pub series_id: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Group {
    pub name: String,
    #[serde(rename = "partnerGroupId")]
    pub partner_group_id: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct MediaMetadata2 {
    pub format: String,
    #[serde(rename = "mediaId")]
    pub media_id: String,
    pub phase: String,
    #[serde(rename = "playbackUrls")]
    pub playback_urls: Vec<PlaybackUrl>,
    #[serde(rename = "productType")]
    pub product_type: String,
    #[serde(rename = "runtimeMillis")]
    pub runtime_millis: i64,
    pub state: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct PlaybackUrl {
    pub rel: String,
    pub href: String,
    pub templated: bool,
    pub params: Vec<Param>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Param {
    pub name: String,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CollectionGroup2 {
    #[serde(rename = "collectionGroupId")]
    pub collection_group_id: String,
    #[serde(rename = "contentClass")]
    pub content_class: String,
    pub key: String,
    pub slugs: Vec<Slug3>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Slug3 {
    pub language: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Meta {
    pub hits: i64,
    pub offset: i64,
    pub page_size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Text2 {
    pub title: Title2,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Title2 {
    pub full: Full2,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Full2 {
    pub set: Set2,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Set2 {
    pub default: Default80,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default80 {
    pub content: String,
    pub language: String,
    #[serde(rename = "sourceEntity")]
    pub source_entity: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Image2 {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Text3 {
    pub title: Title3,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Title3 {
    pub full: Full3,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Full3 {
    pub collection: Collection2,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Collection2 {
    pub default: Default81,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default81 {
    pub content: String,
    pub language: String,
    #[serde(rename = "sourceEntity")]
    pub source_entity: String,
}
