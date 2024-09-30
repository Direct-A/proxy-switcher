# Proxy Switcher

Proxy Switcher 是一个简单易用的代理切换工具，可以帮助你根据当前的网络环境快速切换Windows系统代理服务器。

## TODO

- [ ] 首次启动自动创建 config.toml，并提供示例 config.toml
- [ ] 更加详细的网络检查，检查网络名称，网卡设备名，IP 等等
- [ ] 更加全面的网络代理，不需要单独再为 ssh 进行设置

## 特性

- 系统级代理设置，支持多种代理协议：HTTP、HTTPS 和 SOCKS5
- 静默运行，仅有托盘图标
- 快速切换代理服务器

## 安装

你可以从 GitHub 仓库下载最新版本的 Proxy Switcher:

```
https://github.com/Direct-A/proxy-switcher/releases
```

## 使用

1. 在程序运行目录下书写配置文件`config.toml`。配置文件示例如下：
```toml
proxy_network = "以太网"
proxy_host = "socks://172.20.10.1" # 对于socks5代理可以直接使用 socks://ip的形式，http代理直接填写ip
proxy_port = 9988
check_interval = 15 # 查询网络状态的间隔
log_level = "info"
```
2. 启动 Proxy Switcher 应用程序。
3. 如有需要手动停止代理，右击托盘，菜单中选择`Disable Proxy`。
   1. 此时如需启动代理，右击托盘，菜单中选择`Enable Proxy`。
4. 如果需要为`ssh`进行代理，还需进行详细配置具体参考
   1. [Ncat 使用方法整理](https://www.jianshu.com/p/340b3897e725)
   2. [How To Install Netcat on Windows 10/11](https://medium.com/@bonguides25/how-to-install-netcat-on-windows-10-11-f5be1a185611)

## 贡献

如果你发现了 Proxy Switcher 的任何问题或有新的功能建议,欢迎提交 issue 或 pull request。我会尽快处理并回复。
