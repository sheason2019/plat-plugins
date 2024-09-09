# 使用 Wit-Deps 管理 WIT 文件。

https://github.com/bytecodealliance/wit-deps

# 通过符号链接复用源代码

目前 WitBindgen 对类型复用的支持还不完善，因此通过符号链接的方式实现源代码的复用。

WitBindgen 相关 Issue：https://github.com/bytecodealliance/cargo-component/issues/76。

Windows 下使用 PowerShell 管理符号链接：

```powershell
# 例子
New-Item -ItemType SymbolicLink -Path "./wit" -Target "F:\WorkSpace\plat_plugins\web_tool\wit"
```

Target 必须使用绝对路径才可以正确创建指向目录的软连接，这个可能是 Powershell 的一个 Bug。

# PlatX

PlatX CLI 提供了插件开发能力。

打包和解包命令：

```sh
# 打包
platx tar ./path/to/plugin.json -o foo.plat
# 解包
platx untar foo.plat -o ./path/to/plugin_dir
```

拉起即时服务：

```sh
platx serve ./path/to/plugin.json -d http://daemon_address.dev -r http://vite_server.dev -p port
```
