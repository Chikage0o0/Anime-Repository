# 动漫仓库

一个轻量级的爬虫程序，旨在后台自动抓取指定目录中的文件。

我不太擅长绘画，如果您愿意为它设计一个 logo，请联系我，非常感谢。

这是我用 Rust 和 Typescript 编写的第一个程序，也是我的毕业设计。

# 特点

- 可定制多个规则匹配文件
- 兼容 BT 种子下载
- 可使用 OpenAI 进行自动智能匹配

## 用户界面

![UI](https://raw.githubusercontent.com/Chikage0o0/Anime-Repository/master/img/ui.png)

### 关于订阅规则

**订阅规则的优先级高于 OpenAI 识别**

例如，文件路径为：
```
Downloads/AnimeRepository/[Lilith-Raws] Isekai Nonbiri Nouka - 03 [Baha][WEB-DL][1080p][AVC AAC][CHT][MP4].mp4
```
电视剧的正则表达式可以是 `[Lilith-Raws] Isekai Nonbiri Nouka`，只需部分匹配文件路径即可。

在大多数情况下，剧集的正则表达式将保持默认值，只需更改位置和偏移量。

位置是 Regex 匹配的文件名中剧集的位置。

例如：
```
// 剧集正则表达式为 \d+，表示只匹配数字

[Lilith-Raws] Isekai Nonbiri Nouka - 03 [Baha][WEB-DL][1080p][AVC AAC][CHT][MP4].mp4

// 位置 1 = 03
// 位置 2 = 1080
// 位置 3 = 4
```
对于文件名中的剧集与 TMDB 不匹配的罕见情况，可以使用偏移量进行校正。

这是一个规则示例

![Rule](https://raw.githubusercontent.com/Chikage0o0/Anime-Repository/master/img/rule.png)

### 关于 OpenAI
**OpenAI 不会匹配子文件夹中的文件。**

你所需要做的就是填写 OpenAI API KEY 并打开开关，其余全部自动完成。

### 关于创建软链接
如果需要BT文件在移动后继续做种，需要允许程序创建软链接，这在Windows平台需要额外的操作。

在 Windows 中，创建软链接需要管理员权限。您可以通过以下方法绕过管理员权限。

- 启用开发者模式（仅适用于本地目录）。
- 编辑组策略
  1. 打开组策略
  2. 转到 计算机配置->Windows 设置->安全设置->本地策略->用户权限分配
  3. 编辑**创建符号链接**
  4. 添加您的用户名
  5. 重启

## 开发

阅读 [https://tauri.app/v1/guides/getting-started/prerequisites/](https://tauri.app/v1/guides/getting-started/prerequisites/)

```bash
git clone https://github.com/Chikage0o0/Anime-Repository
cd Anime-Repository
npm install
code .
//TMDB_KEY is v4
export TMDB_KEY="xxxxxxxxx" or $Env:TMDB_KEY="xxxxxxxxx"
yarn tauri dev 
```