//! Implements the `GetDetails` endpoint

use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    errors::{ErrorHandle, PublishedFileServiceError},
    macros::{do_http, optional_argument},
    Steam, BASE,
};

use super::query_files::File;
use super::INTERFACE;

const ENDPOINT: &str = "GetDetails";
const VERSION: &str = "1";

/// Represents an invalid file (i.e. file with given ID doesn't exist).
#[derive(Serialize, Deserialize, Debug)]
pub struct InvalidFile {
    /// Result status of the file.
    pub result: u64,
    /// The published file ID.
    #[serde(rename = "publishedfileid")]
    pub published_file_id: String,
    /// Language of the file.
    pub language: u32,
}

/// Represents whether the file details of its corresponding ID were valid.
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)] // it is expected that most files will be valid
pub enum FileDetailResult {
    /// A valid file.
    Valid(File),
    /// An invalid file.
    Invalid(InvalidFile),
}

/// Represents published file information with specified IDs.
#[derive(Serialize, Deserialize, Debug)]
pub struct FileDetails {
    /// Details of published files with specified IDs.
    #[serde(rename = "publishedfiledetails")]
    #[serde(default)]
    pub published_file_details: Vec<FileDetailResult>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    response: FileDetails,
}

fn url_encode<T: Display>(name: &str, list: &[T]) -> String {
    list.iter()
        .enumerate()
        // .map(|(i, t)| format!("{name}%5B{i}%5D={t}"))
        .map(|(i, t)| format!("{name}[{i}]={t}"))
        .collect::<Vec<String>>()
        .join("&")
}

impl Steam {
    /// Retrieves information about a set of published files.
    ///
    /// # Arguments
    ///
    /// * `published_file_ids` - Set of published file Ids to retrieve details for.
    /// * `include_tags` - If true, return tag information in the returned details.
    /// * `include_additional_previews` - If true, return preview information in the returned details.
    /// * `include_children` - If true, return children in the returned details.
    /// * `include_kv_tags` - If true, return key value tags in the returned details.
    /// * `include_votes` - If true, return vote data in the returned details.
    /// * `short_description` - If true, return a short description instead of the full description.
    /// * `include_for_sale_data` - If true, return pricing data, if applicable.
    /// * `include_metadata` - If true, populate the metadata field.
    /// * `language` - Specifies the localized text to return. Defaults to English.
    /// * `return_playtime_stats` - Return playtime stats for the specified number of days before today.
    /// * `app_id` - App that consumes the files.
    /// * `strip_description_bbcode` - Strips BBCode from descriptions.
    /// * `desired_revision` - Returns the data for the specified revision.
    /// * `include_reactions` - If true, then reactions to items will be returned.
    /// * `admin_query` - Admin tool is doing a query, return hidden items.
    pub async fn get_details(
        &self,
        published_file_ids: impl AsRef<[u64]>,
        include_tags: bool,
        include_additional_previews: bool,
        include_children: bool,
        include_kv_tags: bool,
        include_votes: bool,
        short_description: bool,
        include_for_sale_data: bool,
        include_metadata: bool,
        language: Option<i32>,
        return_playtime_stats: u32,
        app_id: u32,
        strip_description_bbcode: bool,
        desired_revision: Option<u32>,
        include_reactions: Option<bool>,
        admin_query: bool,
    ) -> Result<FileDetails, PublishedFileServiceError> {
        let query = vec![
            format!("?key={}", &self.api_key),
            format!(
                "&{}",
                url_encode("publishedfileids", published_file_ids.as_ref())
            ),
            format!("&includetags={}", include_tags),
            format!("&includeadditionalpreviews={}", include_additional_previews),
            format!("&includechildren={}", include_children),
            format!("&includekvtags={}", include_kv_tags),
            format!("&include_votes={}", include_votes),
            format!("&short_description={}", short_description),
            format!("&includeforsaledata={}", include_for_sale_data),
            format!("&includemetadata={}", include_metadata),
            format!("&return_playtime_stats={}", return_playtime_stats),
            format!("&appid={}", app_id),
            format!("&strip_description_bbcode={}", strip_description_bbcode),
            format!("&admin_query={}", admin_query),
            optional_argument!(language),
            optional_argument!(desired_revision),
            optional_argument!(include_reactions, "includereactions"),
        ];

        let url = format!(
            "{}/{}/{}/v{}/{}",
            BASE,
            INTERFACE,
            ENDPOINT,
            VERSION,
            query.concat()
        );

        let response = do_http!(
            url,
            Response,
            ErrorHandle,
            PublishedFileServiceError::GetDetails
        );

        Ok(response.response)
    }
}
