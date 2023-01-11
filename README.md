# Anime-Repository

功能需求：

1. 根据指定规则识别临时文件夹内的文件名以及集数
2. 使用 tmdb 获取每集的详细信息，并写入符合 kodi 规则的 nfo 文件内
3. 根据规则将文件移动到指定路径并规范重命名
4. 显示视频并且支持断点续播
5. 国际化支持

侧边菜单：

- 媒体库
- 监控规则
- 设置

技术栈：
tauri、vite、react-ts、sass、antd

参考文档：

- [KODI.nfo](https://kodi.wiki/view/NFO_files)
- [TMDB API](https://developers.themoviedb.org/3)
- [antd](https://ant.design/components/overview-cn)
- [tauri api](https://tauri.app/zh-cn/v1/api/config)
