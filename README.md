# free chatgpt

无需账号即可免费无限次数使用的chatgpt api.
- 优点: 免费, 无需账号, 支持多个模型。
- 缺点: 不是很快, 不太稳定。

## 原理
Vercel提供了无需登录免费使用的[AI Playground](https://play.vercel.ai), 限制了使用次数。
其验证的是浏览器的user-agent, 所以只要一直更换请求头则可以一直免费使用。

## 例子

- url: `https://api.classmatelin.top/api`
- curl体验: 
```
curl --silent --location --request POST 'https://api.classmatelin.top/api' \
--header 'Content-Type: application/json' \
--data-raw '{
 
        "model": "openai:gpt-3.5-turbo",
        "prompt": "您是一个Rust语言专家,我有问题需要问你。\n\n请问如何写一个hello world程序?"
}'
```


- ![usage](./images/usage.png)

- vercel单个api仅支持256个token,通过多次请求合并上下文支持超过1024个token。
![usage2](./images/usage2.png)
## 安装

![images](./images/images.png)

- `docker run -itd -p 8080:8080 --name=free-chatgpt classmatelin/free-chatgpt:latest`.

- docker-compose, 见[lazy-docker](https://github.com/ClassmateLin/lazy-docker)

如需使用proxy, 则添加环境变量:
```
SOCKS_PROXY: "socks5:x.x.x.x:x"
HTTP_PROXY: "https://x.x.x.x:x"
HTTPS_PROXY: "https://x.x.x.x:x"
```
例如: `docker run -itd -p 8080:8080 -e ALL_PROXY="socks5://192.168.123.88:1080" --name=free-chatgpt classmatelin/free-chatgpt:latest`.
## 使用

```
curl --silent --location --request POST 'http://127.0.0.1:8080/api' \
--header 'Content-Type: application/json' \
--data-raw '{
 
        "model": "openai:gpt-3.5-turbo",
        "prompt": "您是一个Rust语言专家,我有问题需要问你。\n\n请问如何写一个hello world程序?"
}'
```

## 参数说明

|参数|必填|描述|
|--|--|---|
|model|N|默认: openai:gpt-3.5-turbo.|
|temperature|N|默认:1|
|topP|N|默认:1|
|frequencyPenalty|N|默认：0|
|presence_penalty|N|默认:0|
|stop_sequences|N|默认:[]|

## 支持模型

- anthropic:claude-instant-v1
- anthropic:claude-v1
- replicate:replicate/alpaca-7b
- replicate:stability-ai/stablelm-tuned-alpha-7b
- huggingface:bigscience/bloomz
- huggingface:google/flan-t5-xxl
- huggingface:google/flan-ul2
- cohere:command-medium-nightly
- cohere:command-xlarge-nightly
- openai:gpt-3.5-turbo
- openai:text-ada-001
- openai:text-babbage-001
- openai:text-curie-001
- openai:text-davinci-002
- openai:text-davinci-003


**默认使用openai:gpt-3.5-turbo**


## 声明

此项目仅用于测试和学习研究，请勿用于商业用途, 不能保证其合法性，准确性，完整性和有效性。