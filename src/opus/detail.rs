//! 图文详细信息
//!
//! [图文详细信息](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/opus/detail.md)

use serde::{Deserialize, Serialize};

use crate::{BilibiliRequest, BpiClient, BpiResponse};

/// 图文详细信息响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DetailData {
    #[serde(rename = "item")]
    Item(DetailItem),
    #[serde(rename = "fallback")]
    Fallback(Fallback),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DetailItem {
    /// 基本信息
    pub basic: ItemBasic,
    /// 动态 id
    pub id_str: String,
    /// 模块信息（参见 [功能模块](features.md)）
    // pub modules: todo,
    /// 类型
    pub r#type: i32,
    // 根据现在的测试，这个字段可能移动到了上一个层级，接口文档过时
    // /// 回滚信息（请检查请求参数 `features`）
    // pub fallback: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Fallback {
    id: String,
    r#type: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemBasic {
    /// 评论对象 id 字符串
    pub comment_id_str: String,
    /// 评论区类型
    pub comment_type: i32,
    /// 点赞图标?
    pub like_icon: BasicLikeIcon,
    /// 关联 id 字符串
    pub rid_str: String,
    /// 图文标题  
    pub title: String,
    /// 作者 mid (UID)
    pub uid: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BasicLikeIcon {
    pub action_url: String,
    pub end_url: String,
    pub id: u64,
    pub start_url: String,
}

impl BpiClient {
    /// 获取图文详细信息
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/opus)
    ///
    /// # 参数
    ///
    /// | 参数名          | 类型   | 内容     | 必要性 | 备注 |
    /// | --------------- | ------ | -------- | ------ | ---- |
    /// | id              | string | 动态 id  | 必要   | 数字 |
    /// | timezone_offset | number | 时区偏移 | 非必要 | 如 `-480` |
    /// | features        | string | 功能     | 非必要 | `onlyfansVote,onlyfansAssetsV2,decorationCard,htmlNewStyle,ugcDelete,editable,opusPrivateVisible,tribeeEdit,avatarAutoTheme,avatarTypeOpus` |
    pub async fn opus_detail(
        &self,
        id: u64,
        timezone_offset: Option<i64>,
        features: Option<&str>,
    ) -> Result<BpiResponse<DetailData>, crate::BpiError> {
        let query = vec![
            ("id", id.to_string()),
            (
                "timezone_offset",
                timezone_offset.map_or("".to_string(), |t| t.to_string()),
            ),
            ("features", features.unwrap_or("").to_string()),
        ];
        self.get("https://api.bilibili.com/x/polymer/web-dynamic/v1/opus/detail")
            .query(&query)
            .send_bpi("获取图文详细信息")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[tokio::test]
    async fn test_opus_detail() {
        let bpi = BpiClient::new();
        let resp = bpi.opus_detail(933099353259638816, None, None).await;
        assert!(resp.is_ok());
        if let Ok(r) = resp {
            info!("图文详细信息返回: {:?}", r);
        }
    }
}
