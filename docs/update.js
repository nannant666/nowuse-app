// Nowuse AI 客户端远程配置
// 客户端启动时按顺序请求本文件；JSONP 形式，任何来源都能加载。
//
// min       最低可用版本。客户端低于此版本会弹出强制更新页，无法进入站点。
//           平时留空字符串表示不强制；需要强制时写目标版本号（如 "1.2.0"）。
// latest    当前最新版本，仅用于文案展示。
// entries   进入站点的线路，按顺序取第一条；换域名时改这里，无需重新发版。
// download  各平台安装包地址，强制更新时按钮指向这里。
// note      强制更新弹窗里的说明文字。
window.__NOWUSE_CONFIG__({
    min: '',
    latest: '1.0.0',
    note: '',
    entries: [
        'https://chat2.nowuse-ai.com/list/',
        'https://chat1.nowuse-ai.com/list/',
        'https://chat3.nowuse-ai.com/list/'
    ],
    download: {
        android: '',
        windows: ''
    },
    updatedAt: '2026-11-11T02:40:00+08:00'
});
