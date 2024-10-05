/**
 * @file    bcut.rs
 * @brief   Defines data structures to deserialize BCut JSON data.
 * @author  GitHub@Akiko97 <mud.miscue_0l@icloud.com>
 *
 * Copyright(C) 2024, GitHub@Akiko97. All Right Reserved
 */

use serde::{Deserialize};

#[derive(Deserialize)]
pub struct BCutData {
    pub tracks: Vec<BCutTrack>,
}

#[derive(Deserialize)]
pub struct BCutTrack {
    pub clips: Vec<BCutClip>,
}

#[derive(Deserialize)]
pub struct BCutClip {
    #[serde(rename = "30021")]
    pub start: u64,
    #[serde(rename = "30012")]
    pub duration: u64,
    #[serde(rename = "AssetInfo")]
    pub asset_info: BCutAssetInfo,
}

#[derive(Deserialize)]
pub struct BCutAssetInfo {
    pub content: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
}
