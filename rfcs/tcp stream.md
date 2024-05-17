# 功能

- tcp stream
- 2024-5-17
- zuiyu1998

## 概述

位于 tcp socket 上的高级抽象，使用于原生客户端和 connect 服务的通信。

## 指南级别的解释

tcp 连接是连续的字节流，这里最需要处理的是粘包。采用最常见也最简单的方案传输每个包的长度。

## 参考级别解释

使用 Tokio 来处理网络数据，可以方便的复用 Tokio 的生态，这里可以使用 tokio_utils 这个库简化处理。新增一个 LengthCodec。LengthCodec 会先获取负载的长度，然后获取整个有效的负载。它需要实现 Decoder 和 Encoder 这两个 trait。
