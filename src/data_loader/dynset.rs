#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Dynset {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Data {
    #[serde(rename = "TrendingSet")]
    pub trending_set: Option<TrendingSet>,
    #[serde(rename = "CuratedSet")]
    pub curated_set: Option<CuratedSet>,
    #[serde(rename = "PersonalizedCuratedSet")]
    pub personalized_curated_set: Option<PersonalizedCuratedSet>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct TrendingSet {
    #[serde(rename = "contentClass")]
    pub content_class: String,
    #[serde(rename = "experimentToken")]
    pub experiment_token: String,
    pub items: Vec<Item>,
    pub meta: Meta,
    #[serde(rename = "setId")]
    pub set_id: String,
    pub text: Text2,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Item {
    #[serde(rename = "contentId")]
    pub content_id: String,
    #[serde(rename = "callToAction")]
    pub call_to_action: ::serde_json::Value,
    #[serde(rename = "currentAvailability")]
    pub current_availability: CurrentAvailability,
    #[serde(rename = "encodedSeriesId")]
    pub encoded_series_id: Option<String>,
    pub image: Image,
    #[serde(rename = "seriesId")]
    pub series_id: Option<String>,
    pub text: Text,
    #[serde(rename = "textExperienceId")]
    pub text_experience_id: Option<String>,
    pub tags: Vec<Tag>,
    #[serde(rename = "mediaRights")]
    pub media_rights: MediaRights,
    pub ratings: Vec<Rating>,
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
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CurrentAvailability {
    pub region: String,
    #[serde(rename = "kidsMode")]
    pub kids_mode: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Image {
    pub background_details: Option<BackgroundDetails>,
    pub tile: Tile,
    pub hero_tile: HeroTile,
    pub background: Background,
    pub title_treatment: TitleTreatment,
    pub hero_collection: HeroCollection,
    pub title_treatment_layer: TitleTreatmentLayer,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BackgroundDetails {
    #[serde(rename = "1.78")]
    pub n178: N178,
    #[serde(rename = "1.33")]
    pub n133: N133,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N178 {
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
pub struct N133 {
    pub program: Option<Program2>,
    pub series: Option<Series2>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program2 {
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
pub struct Series2 {
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
pub struct Tile {
    #[serde(rename = "2.29")]
    pub n229: Option<N229>,
    #[serde(rename = "0.75")]
    pub n075: N075,
    #[serde(rename = "1.00")]
    pub n100: Option<N100>,
    #[serde(rename = "1.78")]
    pub n178: N1782,
    #[serde(rename = "1.33")]
    pub n133: Option<N1332>,
    #[serde(rename = "0.71")]
    pub n071: N071,
    #[serde(rename = "0.67")]
    pub n067: Option<N067>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N229 {
    pub program: Option<Program3>,
    pub series: Option<Series3>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program3 {
    pub default: Default5,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default5 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series3 {
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
pub struct N075 {
    pub series: Option<Series4>,
    pub program: Option<Program4>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series4 {
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
pub struct Program4 {
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
pub struct N100 {
    pub series: Option<Series5>,
    pub program: Option<Program5>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series5 {
    pub default: Default9,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default9 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program5 {
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
pub struct N1782 {
    pub series: Option<Series6>,
    pub program: Option<Program6>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series6 {
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
pub struct Program6 {
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
pub struct N1332 {
    pub program: Option<Program7>,
    pub series: Option<Series7>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program7 {
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
pub struct Series7 {
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
pub struct N071 {
    pub series: Option<Series8>,
    pub program: Option<Program8>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series8 {
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
pub struct Program8 {
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
pub struct N067 {
    pub program: Option<Program9>,
    pub series: Option<Series9>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program9 {
    pub default: Default17,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default17 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series9 {
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
pub struct HeroTile {
    #[serde(rename = "3.91")]
    pub n391: N391,
    #[serde(rename = "3.00")]
    pub n300: N300,
    #[serde(rename = "1.78")]
    pub n178: N1783,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N391 {
    pub series: Option<Series10>,
    pub program: Option<Program10>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series10 {
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
pub struct Program10 {
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
pub struct N300 {
    pub series: Option<Series11>,
    pub program: Option<Program11>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series11 {
    pub default: Default21,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default21 {
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
pub struct N1783 {
    pub series: Option<Series12>,
    pub program: Option<Program12>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series12 {
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
pub struct Program12 {
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
pub struct Background {
    #[serde(rename = "1.78")]
    pub n178: N1784,
    #[serde(rename = "2.89")]
    pub n289: Option<N289>,
    #[serde(rename = "1.33")]
    pub n133: Option<N1333>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N1784 {
    pub series: Option<Series13>,
    pub program: Option<Program13>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series13 {
    pub default: Default25,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default25 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program13 {
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
pub struct N289 {
    pub program: Option<Program14>,
    pub series: Option<Series14>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program14 {
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
pub struct Series14 {
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
pub struct N1333 {
    pub series: Option<Series15>,
    pub program: Option<Program15>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series15 {
    pub default: Default29,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default29 {
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
pub struct TitleTreatment {
    #[serde(rename = "3.32")]
    pub n332: N332,
    #[serde(rename = "1.78")]
    pub n178: N1785,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N332 {
    pub series: Option<Series16>,
    pub program: Option<Program16>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series16 {
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
pub struct Program16 {
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
pub struct N1785 {
    pub series: Option<Series17>,
    pub program: Option<Program17>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series17 {
    pub default: Default33,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default33 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program17 {
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
pub struct HeroCollection {
    #[serde(rename = "1.78")]
    pub n178: N1786,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N1786 {
    pub series: Option<Series18>,
    pub program: Option<Program18>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series18 {
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
pub struct Program18 {
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
pub struct TitleTreatmentLayer {
    #[serde(rename = "3.00")]
    pub n300: N3002,
    #[serde(rename = "3.91")]
    pub n391: N3912,
    #[serde(rename = "1.78")]
    pub n178: N1787,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N3002 {
    pub series: Option<Series19>,
    pub program: Option<Program19>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series19 {
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
pub struct Program19 {
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
pub struct N3912 {
    pub series: Option<Series20>,
    pub program: Option<Program20>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series20 {
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
pub struct Program20 {
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
pub struct N1787 {
    pub series: Option<Series21>,
    pub program: Option<Program21>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series21 {
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
pub struct Program21 {
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
pub struct Text {
    pub title: Title,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Title {
    pub slug: Slug,
    pub full: Full,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Slug {
    pub series: Option<Series22>,
    pub program: Option<Program22>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series22 {
    pub default: Default43,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default43 {
    pub content: String,
    pub language: String,
    #[serde(rename = "sourceEntity")]
    pub source_entity: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program22 {
    pub default: Default44,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default44 {
    pub content: String,
    pub language: String,
    #[serde(rename = "sourceEntity")]
    pub source_entity: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Full {
    pub series: Option<Series23>,
    pub program: Option<Program23>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series23 {
    pub default: Default45,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default45 {
    pub content: String,
    pub language: String,
    #[serde(rename = "sourceEntity")]
    pub source_entity: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program23 {
    pub default: Default46,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default46 {
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
    pub release_date: String,
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
    pub season_id: Option<::serde_json::Value>,
    #[serde(rename = "seriesId")]
    pub series_id: Option<::serde_json::Value>,
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
    pub set: Set,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Set {
    pub default: Default47,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default47 {
    pub content: String,
    pub language: String,
    #[serde(rename = "sourceEntity")]
    pub source_entity: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CuratedSet {
    #[serde(rename = "contentClass")]
    pub content_class: String,
    pub items: Vec<Item2>,
    pub meta: Meta2,
    #[serde(rename = "setId")]
    pub set_id: String,
    pub text: Text4,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Item2 {
    #[serde(rename = "contentId")]
    pub content_id: String,
    #[serde(rename = "callToAction")]
    pub call_to_action: ::serde_json::Value,
    #[serde(rename = "currentAvailability")]
    pub current_availability: CurrentAvailability2,
    #[serde(rename = "encodedSeriesId")]
    pub encoded_series_id: Option<String>,
    pub image: Image2,
    #[serde(rename = "seriesId")]
    pub series_id: Option<String>,
    pub text: Text3,
    #[serde(rename = "textExperienceId")]
    pub text_experience_id: Option<String>,
    pub tags: Vec<Tag2>,
    #[serde(rename = "mediaRights")]
    pub media_rights: MediaRights2,
    pub ratings: Vec<Rating2>,
    pub releases: Vec<Release2>,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "videoArt")]
    pub video_art: Vec<VideoArt2>,
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
    #[serde(rename = "episodeNumber")]
    pub episode_number: Option<::serde_json::Value>,
    #[serde(rename = "episodeSequenceNumber")]
    pub episode_sequence_number: Option<::serde_json::Value>,
    #[serde(rename = "episodeSeriesSequenceNumber")]
    pub episode_series_sequence_number: Option<::serde_json::Value>,
    pub family: Option<Family2>,
    #[serde(default)]
    pub groups: Vec<Group2>,
    #[serde(rename = "internalTitle")]
    pub internal_title: Option<String>,
    #[serde(rename = "mediaMetadata")]
    pub media_metadata: Option<MediaMetadata4>,
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
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CurrentAvailability2 {
    pub region: String,
    #[serde(rename = "kidsMode")]
    pub kids_mode: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Image2 {
    pub tile: Tile2,
    pub title_treatment_layer: TitleTreatmentLayer2,
    pub title_treatment: TitleTreatment2,
    pub background: Option<Background2>,
    pub background_details: Option<BackgroundDetails2>,
    pub hero_collection: HeroCollection2,
    pub hero_tile: HeroTile2,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Tile2 {
    #[serde(rename = "2.29")]
    pub n229: Option<N2292>,
    #[serde(rename = "1.78")]
    pub n178: N1788,
    #[serde(rename = "0.71")]
    pub n071: N0712,
    #[serde(rename = "0.75")]
    pub n075: Option<N0752>,
    #[serde(rename = "0.67")]
    pub n067: Option<N0672>,
    #[serde(rename = "1.00")]
    pub n100: Option<N1002>,
    #[serde(rename = "1.33")]
    pub n133: Option<N1334>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N2292 {
    pub program: Option<Program24>,
    pub series: Option<Series24>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program24 {
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
pub struct Series24 {
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
pub struct N1788 {
    pub series: Option<Series25>,
    pub program: Option<Program25>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series25 {
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
pub struct Program25 {
    pub default: Default51,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default51 {
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
    pub series: Option<Series26>,
    pub program: Option<Program26>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series26 {
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
pub struct Program26 {
    pub default: Default53,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default53 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N0752 {
    pub program: Option<Program27>,
    pub series: Option<Series27>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program27 {
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
pub struct Series27 {
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
pub struct N0672 {
    pub program: Option<Program28>,
    pub series: Option<Series28>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program28 {
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
pub struct Series28 {
    pub default: Default57,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default57 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N1002 {
    pub program: Option<Program29>,
    pub series: Option<Series29>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program29 {
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
pub struct Series29 {
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
pub struct N1334 {
    pub program: Option<Program30>,
    pub series: Option<Series30>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program30 {
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
pub struct Series30 {
    pub default: Default61,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default61 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct TitleTreatmentLayer2 {
    #[serde(rename = "3.00")]
    pub n300: N3003,
    #[serde(rename = "3.91")]
    pub n391: N3913,
    #[serde(rename = "1.78")]
    pub n178: N1789,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N3003 {
    pub series: Option<Series31>,
    pub program: Option<Program31>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series31 {
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
pub struct Program31 {
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
pub struct N3913 {
    pub series: Option<Series32>,
    pub program: Option<Program32>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series32 {
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
pub struct Program32 {
    pub default: Default65,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default65 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N1789 {
    pub series: Option<Series33>,
    pub program: Option<Program33>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series33 {
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
pub struct Program33 {
    pub default: Default67,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default67 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct TitleTreatment2 {
    #[serde(rename = "1.78")]
    pub n178: N17810,
    #[serde(rename = "3.32")]
    pub n332: Option<N3322>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N17810 {
    pub series: Option<Series34>,
    pub program: Option<Program34>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series34 {
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
pub struct Program34 {
    pub default: Default69,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default69 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N3322 {
    pub series: Option<Series35>,
    pub program: Option<Program35>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series35 {
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
pub struct Program35 {
    pub default: Default71,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default71 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Background2 {
    #[serde(rename = "1.78")]
    pub n178: N17811,
    #[serde(rename = "1.33")]
    pub n133: Option<N1335>,
    #[serde(rename = "2.89")]
    pub n289: Option<N2892>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N17811 {
    pub program: Option<Program36>,
    pub series: Option<Series36>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program36 {
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
pub struct Series36 {
    pub default: Default73,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default73 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N1335 {
    pub series: Option<Series37>,
    pub program: Option<Program37>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series37 {
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
pub struct Program37 {
    pub default: Default75,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default75 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N2892 {
    pub program: Option<Program38>,
    pub series: Option<Series38>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program38 {
    pub default: Default76,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default76 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series38 {
    pub default: Default77,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default77 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BackgroundDetails2 {
    #[serde(rename = "1.78")]
    pub n178: N17812,
    #[serde(rename = "1.33")]
    pub n133: N1336,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N17812 {
    pub program: Option<Program39>,
    pub series: Option<Series39>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program39 {
    pub default: Default78,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default78 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series39 {
    pub default: Default79,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default79 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N1336 {
    pub program: Option<Program40>,
    pub series: Option<Series40>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program40 {
    pub default: Default80,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default80 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series40 {
    pub default: Default81,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default81 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct HeroCollection2 {
    #[serde(rename = "1.78")]
    pub n178: N17813,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N17813 {
    pub series: Option<Series41>,
    pub program: Option<Program41>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series41 {
    pub default: Default82,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default82 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program41 {
    pub default: Default83,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default83 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct HeroTile2 {
    #[serde(rename = "3.00")]
    pub n300: N3004,
    #[serde(rename = "1.78")]
    pub n178: N17814,
    #[serde(rename = "3.91")]
    pub n391: N3914,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N3004 {
    pub series: Option<Series42>,
    pub program: Option<Program42>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series42 {
    pub default: Default84,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default84 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program42 {
    pub default: Default85,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default85 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N17814 {
    pub series: Option<Series43>,
    pub program: Option<Program43>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series43 {
    pub default: Default86,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default86 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program43 {
    pub default: Default87,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default87 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N3914 {
    pub series: Option<Series44>,
    pub program: Option<Program44>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series44 {
    pub default: Default88,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default88 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program44 {
    pub default: Default89,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default89 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Text3 {
    pub title: Title3,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Title3 {
    pub slug: Slug2,
    pub full: Full3,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Slug2 {
    pub series: Option<Series45>,
    pub program: Option<Program45>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series45 {
    pub default: Default90,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default90 {
    pub content: String,
    pub language: String,
    #[serde(rename = "sourceEntity")]
    pub source_entity: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program45 {
    pub default: Default91,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default91 {
    pub content: String,
    pub language: String,
    #[serde(rename = "sourceEntity")]
    pub source_entity: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Full3 {
    pub series: Option<Series46>,
    pub program: Option<Program46>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series46 {
    pub default: Default92,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default92 {
    pub content: String,
    pub language: String,
    #[serde(rename = "sourceEntity")]
    pub source_entity: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program46 {
    pub default: Default93,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default93 {
    pub content: String,
    pub language: String,
    #[serde(rename = "sourceEntity")]
    pub source_entity: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Tag2 {
    #[serde(rename = "displayName")]
    pub display_name: ::serde_json::Value,
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct MediaRights2 {
    #[serde(rename = "downloadBlocked")]
    pub download_blocked: bool,
    #[serde(rename = "pconBlocked")]
    pub pcon_blocked: bool,
    #[serde(default)]
    pub violations: Vec<::serde_json::Value>,
    pub rewind: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Rating2 {
    pub advisories: Vec<::serde_json::Value>,
    pub description: Option<String>,
    pub system: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Release2 {
    #[serde(rename = "releaseDate")]
    pub release_date: Option<String>,
    #[serde(rename = "releaseType")]
    pub release_type: String,
    #[serde(rename = "releaseYear")]
    pub release_year: i64,
    pub territory: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct VideoArt2 {
    #[serde(rename = "mediaMetadata")]
    pub media_metadata: MediaMetadata3,
    pub purpose: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct MediaMetadata3 {
    pub urls: Vec<Url2>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Url2 {
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Family2 {
    #[serde(rename = "encodedFamilyId")]
    pub encoded_family_id: String,
    #[serde(rename = "familyId")]
    pub family_id: String,
    pub parent: bool,
    #[serde(rename = "parentRef")]
    pub parent_ref: ParentRef2,
    #[serde(rename = "sequenceNumber")]
    pub sequence_number: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ParentRef2 {
    #[serde(rename = "encodedSeriesId")]
    pub encoded_series_id: ::serde_json::Value,
    #[serde(rename = "programId")]
    pub program_id: String,
    #[serde(rename = "seasonId")]
    pub season_id: Option<::serde_json::Value>,
    #[serde(rename = "seriesId")]
    pub series_id: Option<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Group2 {
    pub name: String,
    #[serde(rename = "partnerGroupId")]
    pub partner_group_id: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct MediaMetadata4 {
    pub format: String,
    #[serde(rename = "mediaId")]
    pub media_id: String,
    pub phase: String,
    #[serde(rename = "playbackUrls")]
    pub playback_urls: Vec<PlaybackUrl2>,
    #[serde(rename = "productType")]
    pub product_type: String,
    #[serde(rename = "runtimeMillis")]
    pub runtime_millis: i64,
    pub state: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct PlaybackUrl2 {
    pub rel: String,
    pub href: String,
    pub templated: bool,
    pub params: Vec<Param2>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Param2 {
    pub name: String,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Meta2 {
    pub hits: i64,
    pub offset: i64,
    pub page_size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Text4 {
    pub title: Title4,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Title4 {
    pub full: Full4,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Full4 {
    pub set: Set2,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Set2 {
    pub default: Default94,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default94 {
    pub content: String,
    pub language: String,
    #[serde(rename = "sourceEntity")]
    pub source_entity: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct PersonalizedCuratedSet {
    #[serde(rename = "setId")]
    pub set_id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "contentClass")]
    pub content_class: String,
    pub items: Vec<Item3>,
    pub meta: Meta3,
    #[serde(rename = "experimentToken")]
    pub experiment_token: String,
    pub text: Text6,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Item3 {
    #[serde(rename = "contentId")]
    pub content_id: String,
    #[serde(rename = "currentAvailability")]
    pub current_availability: CurrentAvailability3,
    #[serde(rename = "encodedSeriesId")]
    pub encoded_series_id: Option<String>,
    pub image: Image3,
    pub ratings: Vec<Rating3>,
    pub releases: Vec<Release3>,
    #[serde(rename = "seriesId")]
    pub series_id: Option<String>,
    pub tags: Vec<Tag3>,
    #[serde(rename = "textExperienceId")]
    pub text_experience_id: Option<String>,
    pub text: Text5,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "videoArt")]
    pub video_art: Vec<VideoArt3>,
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
    #[serde(rename = "episodeNumber")]
    pub episode_number: Option<::serde_json::Value>,
    #[serde(rename = "episodeSequenceNumber")]
    pub episode_sequence_number: Option<::serde_json::Value>,
    #[serde(rename = "episodeSeriesSequenceNumber")]
    pub episode_series_sequence_number: Option<::serde_json::Value>,
    pub family: Option<Family3>,
    #[serde(default)]
    pub groups: Vec<Group3>,
    #[serde(rename = "internalTitle")]
    pub internal_title: Option<String>,
    #[serde(rename = "mediaMetadata")]
    pub media_metadata: Option<MediaMetadata6>,
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
    #[serde(rename = "videoId")]
    pub video_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CurrentAvailability3 {
    pub region: String,
    #[serde(rename = "kidsMode")]
    pub kids_mode: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Image3 {
    pub tile: Tile3,
    pub title_treatment_layer: TitleTreatmentLayer3,
    pub hero_tile: HeroTile3,
    pub title_treatment: TitleTreatment3,
    pub hero_collection: HeroCollection3,
    pub background: Background3,
    pub background_details: Option<BackgroundDetails3>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Tile3 {
    #[serde(rename = "1.33")]
    pub n133: N1337,
    #[serde(rename = "1.0")]
    pub n10: Option<N10>,
    #[serde(rename = "0.71")]
    pub n071: N0713,
    #[serde(rename = "1.78")]
    pub n178: N17815,
    #[serde(rename = "2.29")]
    pub n229: Option<N2293>,
    #[serde(rename = "0.67")]
    pub n067: N0673,
    #[serde(rename = "0.75")]
    pub n075: N0753,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N1337 {
    pub series: Option<Series47>,
    pub program: Option<Program47>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series47 {
    pub default: Default95,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default95 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program47 {
    pub default: Default96,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default96 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N10 {
    pub series: Series48,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series48 {
    pub default: Default97,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default97 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N0713 {
    pub series: Option<Series49>,
    pub program: Option<Program48>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series49 {
    pub default: Default98,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default98 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program48 {
    pub default: Default99,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default99 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N17815 {
    pub series: Option<Series50>,
    pub program: Option<Program49>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series50 {
    pub default: Default100,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default100 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program49 {
    pub default: Default101,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default101 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N2293 {
    pub program: Option<Program50>,
    pub series: Option<Series51>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program50 {
    pub default: Default102,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default102 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series51 {
    pub default: Default103,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default103 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N0673 {
    pub series: Option<Series52>,
    pub program: Option<Program51>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series52 {
    pub default: Default104,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default104 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program51 {
    pub default: Default105,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default105 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N0753 {
    pub series: Option<Series53>,
    pub program: Option<Program52>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series53 {
    pub default: Default106,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default106 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program52 {
    pub default: Default107,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default107 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct TitleTreatmentLayer3 {
    #[serde(rename = "3.0")]
    pub n30: N30,
    #[serde(rename = "1.78")]
    pub n178: N17816,
    #[serde(rename = "3.91")]
    pub n391: N3915,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N30 {
    pub series: Option<Series54>,
    pub program: Option<Program53>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series54 {
    pub default: Default108,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default108 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program53 {
    pub default: Default109,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default109 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N17816 {
    pub series: Option<Series55>,
    pub program: Option<Program54>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series55 {
    pub default: Default110,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default110 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program54 {
    pub default: Default111,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default111 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N3915 {
    pub series: Option<Series56>,
    pub program: Option<Program55>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series56 {
    pub default: Default112,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default112 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program55 {
    pub default: Default113,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default113 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct HeroTile3 {
    #[serde(rename = "3.0")]
    pub n30: N302,
    #[serde(rename = "1.78")]
    pub n178: N17817,
    #[serde(rename = "3.91")]
    pub n391: N3916,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N302 {
    pub series: Option<Series57>,
    pub program: Option<Program56>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series57 {
    pub default: Default114,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default114 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program56 {
    pub default: Default115,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default115 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N17817 {
    pub series: Option<Series58>,
    pub program: Option<Program57>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series58 {
    pub default: Default116,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default116 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program57 {
    pub default: Default117,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default117 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N3916 {
    pub series: Option<Series59>,
    pub program: Option<Program58>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series59 {
    pub default: Default118,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default118 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program58 {
    pub default: Default119,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default119 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct TitleTreatment3 {
    #[serde(rename = "1.78")]
    pub n178: N17818,
    #[serde(rename = "3.32")]
    pub n332: N3323,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N17818 {
    pub series: Option<Series60>,
    pub program: Option<Program59>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series60 {
    pub default: Default120,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default120 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program59 {
    pub default: Default121,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default121 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N3323 {
    pub series: Option<Series61>,
    pub program: Option<Program60>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series61 {
    pub default: Default122,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default122 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program60 {
    pub default: Default123,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default123 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct HeroCollection3 {
    #[serde(rename = "1.78")]
    pub n178: N17819,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N17819 {
    pub series: Option<Series62>,
    pub program: Option<Program61>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series62 {
    pub default: Default124,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default124 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program61 {
    pub default: Default125,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default125 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Background3 {
    #[serde(rename = "2.89")]
    pub n289: Option<N2893>,
    #[serde(rename = "1.33")]
    pub n133: Option<N1338>,
    #[serde(rename = "1.78")]
    pub n178: N17820,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N2893 {
    pub program: Option<Program62>,
    pub series: Option<Series63>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program62 {
    pub default: Default126,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default126 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series63 {
    pub default: Default127,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default127 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N1338 {
    pub series: Option<Series64>,
    pub program: Option<Program63>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series64 {
    pub default: Default128,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default128 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program63 {
    pub default: Default129,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default129 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N17820 {
    pub series: Option<Series65>,
    pub program: Option<Program64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series65 {
    pub default: Default130,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default130 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program64 {
    pub default: Default131,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default131 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BackgroundDetails3 {
    #[serde(rename = "1.78")]
    pub n178: N17821,
    #[serde(rename = "1.33")]
    pub n133: N1339,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N17821 {
    pub program: Program65,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program65 {
    pub default: Default132,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default132 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct N1339 {
    pub program: Program66,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program66 {
    pub default: Default133,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default133 {
    #[serde(rename = "masterId")]
    pub master_id: String,
    #[serde(rename = "masterHeight")]
    pub master_height: i64,
    #[serde(rename = "masterWidth")]
    pub master_width: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Rating3 {
    pub advisories: Vec<::serde_json::Value>,
    pub description: Option<String>,
    pub system: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Release3 {
    #[serde(rename = "releaseDate")]
    pub release_date: String,
    #[serde(rename = "releaseType")]
    pub release_type: String,
    #[serde(rename = "releaseYear")]
    pub release_year: i64,
    pub territory: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Tag3 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: String,
    pub entity: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Text5 {
    pub title: Title5,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Title5 {
    pub full: Full5,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Full5 {
    pub series: Option<Series66>,
    pub program: Option<Program67>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series66 {
    pub default: Default134,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default134 {
    pub content: String,
    pub language: String,
    #[serde(rename = "sourceEntity")]
    pub source_entity: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Program67 {
    pub default: Default135,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default135 {
    pub content: String,
    pub language: String,
    #[serde(rename = "sourceEntity")]
    pub source_entity: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct VideoArt3 {
    #[serde(rename = "mediaMetadata")]
    pub media_metadata: MediaMetadata5,
    pub purpose: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct MediaMetadata5 {
    pub urls: Vec<Url3>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Url3 {
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Family3 {
    #[serde(rename = "encodedFamilyId")]
    pub encoded_family_id: String,
    #[serde(rename = "familyId")]
    pub family_id: String,
    pub parent: bool,
    #[serde(rename = "parentRef")]
    pub parent_ref: ParentRef3,
    #[serde(rename = "sequenceNumber")]
    pub sequence_number: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ParentRef3 {
    #[serde(rename = "encodedSeriesId")]
    pub encoded_series_id: ::serde_json::Value,
    #[serde(rename = "programId")]
    pub program_id: String,
    #[serde(rename = "seasonId")]
    pub season_id: Option<::serde_json::Value>,
    #[serde(rename = "seriesId")]
    pub series_id: Option<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Group3 {
    pub name: String,
    #[serde(rename = "partnerGroupId")]
    pub partner_group_id: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct MediaMetadata6 {
    pub format: String,
    #[serde(rename = "mediaId")]
    pub media_id: String,
    pub phase: String,
    #[serde(rename = "playbackUrls")]
    pub playback_urls: Vec<PlaybackUrl3>,
    #[serde(rename = "productType")]
    pub product_type: String,
    #[serde(rename = "runtimeMillis")]
    pub runtime_millis: i64,
    pub state: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct PlaybackUrl3 {
    pub href: String,
    pub params: Vec<Param3>,
    pub rel: String,
    pub templated: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Param3 {
    pub description: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Meta3 {
    pub hits: i64,
    pub page_size: i64,
    pub offset: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Text6 {
    pub title: Title6,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Title6 {
    pub full: Full6,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Full6 {
    pub set: Set3,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Set3 {
    pub default: Default136,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Default136 {
    pub content: String,
    pub language: String,
    #[serde(rename = "sourceEntity")]
    pub source_entity: String,
}
