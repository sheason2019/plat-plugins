# Message 插件设计

Message 是 Plat 的消息插件，插件允许用户通过 X25519 + Chacha20-ploy1305 传输端到端加密信息，同时通过 CRDT 实现消息的多端同步。

# 多端同步

## Operation Index 计算方式

上一次的 Operation Index 的 Ref SHA256 + 当前 Operation 的 Object SHA256。

Object SHA256 用来索引当前 Operation 的内容，Ref SHA256 用来帮助寻找两个设备间 Operation 分支的公共祖先。
