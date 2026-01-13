use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse, dynamic::all::DynamicAllData};

impl BpiClient {
    /// 获取用户空间动态
    ///
    /// | 参数名          | 类型   | 内容   | 必要性 | 备注  |
    /// |-----------------|--------|--------|-----| - |
    /// | offset          | string | 分页偏移量 | 不必要 |     |
    /// | host_mid        | string | 被查询用户 UID (mid)  |必要| |
    /// | timezone_offset | number | 时区偏移| 不必要| 默认 `-480` |
    /// | platform        | string | 平台 | 不必要 | 如 `web` |
    /// | features        | string | 功能 | 不必要 | 留空为空, 默认为 `itemOpusStyle,listOnlyfans,opusBigCover,onlyfansVote,forwardListHidden,decorationCard,commentsNewVersion,onlyfansAssetsV2,ugcDelete,onlyfansQaCard`, 参见 [功能模块](../opus/features.md#features) |
    /// | web_location    | string | `333.1387` | 不必要 |  |
    pub async fn dynamic_of_user_space(
        &self,
        offset: Option<&str>,
        host_mid: &str,
        timezone_offset: Option<i32>,
        platform: Option<&str>,
        features: Option<&str>,
        web_location: Option<&str>,
    ) -> Result<BpiResponse<DynamicAllData>, BpiError> {
        let timezone_offset = timezone_offset.unwrap_or(-480).to_string();
        let params = vec![
            ("offset", offset.unwrap_or("")),
            ("host_mid", host_mid),
            ("timezone_offset", &timezone_offset),
            ("platform", platform.unwrap_or("web")),
            ("features", features.unwrap_or("itemOpusStyle,listOnlyfans,opusBigCover,onlyfansVote,decorationCard,onlyfansAssetsV2,forwardListHidden,ugcDelete")),
            ("web_location", web_location.unwrap_or("333.1387")),
            ("x-bili-device-req-json", r#"{"platform":"web","device":"pc","spmid":"333.1387"}"#)
        ];
        let signed_params = self.get_wbi_sign2(params).await?;
        self.get("https://api.bilibili.com/x/polymer/web-dynamic/v1/feed/space")
            .query(&signed_params)
            .send_bpi("获取用户空间动态")
            .await
    }
}
