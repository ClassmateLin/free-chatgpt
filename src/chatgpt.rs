use crate::random_user_agent;
use anyhow::{anyhow, Result};
use regex::Regex;
use reqwest::{
    self,
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChatgptParams {
    pub prompt: String,
    pub model: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u16>,
    pub top_p: Option<u8>,
    pub frequency_penalty: Option<u8>,
    pub presence_penalty: Option<u8>,
    pub stop_sequences: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct Chatgpt {
    client: Client,
    base_url: &'static str,
}

fn is_complete(str: &str) -> bool {
    str.ends_with("```") || Regex::new(r"[。？！.!?]$").unwrap().is_match(str)
}

/// 核心功能实现
/// 请求一次https://play.vercel.ai/api/oepnai.jpg获取custom-enconding, 再请求https://play.vercel.ai/api/generate.
/// user-agent生成不够完善, 需要记录生成过的user-agent, 避免重复生成。
impl Chatgpt {
    pub fn new() -> Result<Self> {
        let base_url = "https://play.vercel.ai/";
        let mut headers = HeaderMap::new();
        headers.append("origin", HeaderValue::from_str(base_url)?);
        headers.append("referer", HeaderValue::from_str(base_url)?);
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .http2_prior_knowledge()
            .build()?;

        Ok(Self { client, base_url })
    }

    // 
    //```
    //let response=await fetch("/openai.jpeg");
    // data=JSON.parse(atob(await response.text())); # base64解码后解析为json
    // ret=eval("(".concat(data.c,")(data.a)")); # 用data.a作为参数调用函数data.c
    // botPreventionToken=btoa(JSON.stringify({r:ret,t:data.t})) # json转字符串后进行base64编码
    //```
    // 
    pub async fn get_encoding(&self, user_agent: &str) -> Result<String> {
        let url = format!("{}openai.jpeg", self.base_url);
        let response = self
            .client
            .get(url)
            .header("user-agent", HeaderValue::from_str(user_agent)?)
            .send()
            .await?
            .text()
            .await?;
        println!("{}", response);
        Ok(response + ".")
    }

    pub async fn request(self: Arc<Self>, chatgpt_params: &ChatgptParams) -> Result<String> {
        let params = serde_json::json!(chatgpt_params);
        let url = format!("{}api/generate", self.base_url);
        let ua = random_user_agent();
        let encoding = self.get_encoding(&ua).await;
        if encoding.is_err() {
            return Err(anyhow!("Fail to get custom-encoding!"));
        }
        let response = self
            .client
            .post(url)
            .json(&params)
            .header("user-agent", HeaderValue::from_str(&ua)?)
            .header(
                "custom-encoding",
                HeaderValue::from_str(&encoding.unwrap())?,
            )
            .send()
            .await?
            .text()
            .await?;
        if response == "Not authorized" {
            return Err(anyhow!("请尝试更换IP地址..."));
        }
        if response == "Internal Server Error" {
            return Err(anyhow!("API 可能已失效!"));
        }
        let str = response
            .split('\n') // 使用换行符分割字符串
            .map(|c| match c.len() > 2 { // 去除token左右两边双引号
                true => &c[1..c.len() - 1],
                false => "",
            })
            .filter(|c| !c.is_empty()) // 去除空字符串
            .collect::<Vec<_>>()
            .join("") // 拼接完整字符串
            .replace("\\n", "\n")  
            .replace("\\\"", "\""); 
        Ok(str)
    }

    pub async fn ask(self: Arc<Self>, chatgpt_params: ChatgptParams) -> Result<String> {
        let mut res = self.clone().request(&chatgpt_params).await?;
        for _ in 0..10 {
            if res.contains("limit exceede") {// 超出限制, 说明user-agent达到使用上限
                return Err(anyhow!("Rate limit exceeded!"));
            }
            if is_complete(&res) {
                break;
            }
            let mut params = chatgpt_params.clone();
            params.prompt = params.prompt + &res;
            let that = self.clone();
            let tmp = that.request(&params).await?;
            res += &tmp;
        }
        Ok(res)
    }
}
