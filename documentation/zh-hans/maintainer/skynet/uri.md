# Skynet URI 协议

Skynet 协议使用 `skynet://` 作为统一资源标识符（URI）协议，用于引用 Skynet 网络中的各类资源。

## 协议格式

### 基本结构

```
skynet://[subnet_id]/[resource_type]/[resource_id][?query][#fragment]
```

### 组件说明

| 组件 | 说明 | 必需 |
|-----|------|------|
| **skynet://** | 协议头 | 是 |
| **subnet_id** | 子网 ID | 是 |
| **resource_type** | 资源类型 | 否 |
| **resource_id** | 资源 ID | 否 |
| **query** | 查询参数 | 否 |
| **fragment** | 片段标识符 | 否 |

## 资源类型

### 支持的资源类型

| 资源类型 | 说明 | 路径格式 |
|---------|------|---------|
| **subnet** | 子网 | `skynet://{subnet_id}` |
| **user** | 用户 | `skynet://{subnet_id}/user/{user_id}` |
| **channel** | 频道 | `skynet://{subnet_id}/channel/{channel_id}` |
| **message** | 消息 | `skynet://{subnet_id}/message/{message_id}` |
| **resource** | 资源（文件等） | `skynet://{subnet_id}/resource/{resource_id}` |
| **thread** | 消息线程 | `skynet://{subnet_id}/thread/{thread_id}` |

## 示例

### 1. 引用子网

```
skynet://5f4dcc3b5aa765d61d8327deb882cf99
```

### 2. 引用用户

```
skynet://5f4dcc3b5aa765d61d8327deb882cf99/user/user_123
```

### 3. 引用频道

```
skynet://5f4dcc3b5aa765d61d8327deb882cf99/channel/channel_456
```

### 4. 引用消息

```
skynet://5f4dcc3b5aa765d61d8327deb882cf99/message/msg_789
```

### 5. 引用资源

```
skynet://5f4dcc3b5aa765d61d8327deb882cf99/resource/res_abc
```

### 6. 引用线程

```
skynet://5f4dcc3b5aa765d61d8327deb882cf99/thread/msg_789
```

### 7. 带查询参数

```
skynet://5f4dcc3b5aa765d61d8327deb882cf99/message/msg_789?highlight=true&scroll_to_bottom=true
```

## 编码规范

- **subnet_id**：使用 Blake3 哈希值的十六进制表示（小写）
- **user_id**、**channel_id**、**message_id**、**resource_id**、**thread_id**：使用 URL 安全的 Base64 编码或十六进制
- **特殊字符**：路径组件中的特殊字符必须进行 URL 编码

## 子网隔离原则

Skynet URI 遵循子网隔离原则：
- 所有 URI 都必须包含 `subnet_id`
- 资源不跨子网共享，如需跨子网使用必须复制
- URI 中的资源仅在指定子网内可访问

## 客户端处理

客户端在解析 `skynet://` URI 时应：
1. 验证 URI 格式
2. 检查目标子网是否已加入
3. 根据资源类型执行相应操作（如跳转、打开资源等）
4. 对于未加入的子网，可提供加入引导
