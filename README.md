# Nowuse AI 客户端

把站点 https://chat2.nowuse-ai.com 包装成 Windows 桌面端与 Android App 的工程目录。
客户端只是一层外壳：启动页做版本检查与线路选择，然后进入线上站点，因此网页更新后
App 无需重新发版。

## 目录

| 路径 | 用途 |
| --- | --- |
| `shell/index.html` | 启动页：版本检查、强制更新、线路选择。**改这个文件即可调整启动体验** |
| `shell/logo.png` | 启动页图标 |
| `docs/update.js` | 远程配置（JSONP）。强制更新、换域名都改这里，见下 |
| `desktop/` | Windows 端（Tauri 2） |
| `.github/workflows/windows.yml` | Windows 云端打包 |

Android 端在独立仓库 `nannant666/nowuse-android`（基于 PakePlus-Android），启动页副本在
`scripts/www/index.html`，改启动页时两边同步。

## 远程配置怎么用

`docs/update.js` 会被客户端在启动时拉取。三个开关：

| 字段 | 作用 |
| --- | --- |
| `min` | 最低可用版本。填 `"1.2.0"` 后，所有低于 1.2.0 的客户端会卡在强制更新页，无法进入站点 |
| `entries` | 进站线路，按顺序取第一条。换域名只改这里，老客户端自动跟着走 |
| `download.android` / `download.windows` | 强制更新时按钮指向的安装包地址 |

改完保存即生效（客户端每次启动都会拉）。

## 发新版本

1. 改 `desktop/src-tauri/tauri.conf.json` 的 `version`（Android 端改 `scripts/ppconfig.json`）。
2. 推送触发 `windows` 工作流，安装包会挂到 Release。
3. 新的 Android 包上传到文件服务，把地址填进 `docs/update.js` 的 `download`。
4. 需要强制时，把 `min` 改成新版本号。
