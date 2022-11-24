use thiserror::Error;

#[derive(Error, Debug)]
pub enum CgtoolError {
    #[error("获取接口：{url:?} 数据失败！")]
    GetDataError {
        url: String,
        error: reqwest::Error,
    },
    #[error("参数: {param:?}获取失败！")]
    ParamError {
        param: String,
    },
    #[error("json解析失败！")]
    JsonParseError,

    #[error("获取响应失败！")]
    ResponseTextGetFailed,
}