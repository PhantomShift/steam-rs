use steam_rs::{
    published_file_service::query_files::{
        PublishedFileInfoMatchingFileType, PublishedFileQueryType,
    },
    Steam,
};

mod common;

const EXAMPLE_APP_ID: u32 = 440; // Team Fortress 2
const EXAMPLE_ITEM_IDS: &[u64] = &[3345507915, 3476304156]; // 2Fort Classic REDUX and Broojlynn workshop items

#[tokio::test]
pub async fn query_files() {
    let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
    let query = steam
        .query_files(
            PublishedFileQueryType::RankedByVote,
            0,
            "*",
            Some(5),
            EXAMPLE_APP_ID,
            EXAMPLE_APP_ID,
            "",
            "",
            None,
            "",
            "",
            "",
            PublishedFileInfoMatchingFileType::Items,
            0,
            7,
            false,
            None,
            None,
            "",
            false,
            false,
            true,
            true,
            true,
            true,
            true,
            true,
            true,
            Some(true),
            10,
        )
        .await
        .unwrap();
    println!("{:?}", query);
}

#[tokio::test]
pub async fn get_details() {
    let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
    let details = steam
        .get_details(
            Vec::from(EXAMPLE_ITEM_IDS),
            true,
            true,
            true,
            true,
            true,
            false,
            false,
            true,
            None,
            10,
            EXAMPLE_APP_ID,
            false,
            None,
            Some(true),
            false,
        )
        .await
        .unwrap();
    assert_eq!(EXAMPLE_ITEM_IDS.len(), details.published_file_details.len());
    println!("{:?}", details);
}
