# Skynet URI 協定

Skynet 協定使用 `skynet://` 作為統一資源識別碼（URI）協定，用於引用 Skynet 網路中的各類資源。

## 協定格式

### 基本結構

```
skynet://[subnet_id]/[resource_type]/[resource_id][?query][#fragment]
```

### 元件說明

| 元件 | 說明 | 必需 |
|-----|------|------|
| **skynet://** | 協定頭 | 是 |
| **subnet_id** | 子網 ID | 是 |
| **resource_type** | 資源類型 | 否 |
| **resource_id** | 資源 ID | 否 |
| **query** | 查詢參數 | 否 |
| **fragment** | 片段識別碼 | 否 |

## 資源類型

### 支援的資源類型

| 資源類型 | 說明 | 路徑格式 |
|---------|------|---------|
| **subnet** | 子網 | `skynet://{subnet_id}` |
| **user** | 使用者 | `skynet://{subnet_id}/user/{user_id}` |
| **channel** | 頻道 | `skynet://{subnet_id}/channel/{channel_id}` |
| **message** | 訊息 | `skynet://{subnet_id}/message/{message_id}` |
| **resource** | 資源（檔案等） | `skynet://{subnet_id}/resource/{resource_id}` |
| **thread** | 訊息討論串 | `skynet://{subnet_id}/thread/{thread_id}` |

## 範例

### 1. 引用子網

```
skynet://5f4dcc3b5aa765d61d8327deb882cf99
```

### 2. 引用使用者

```
skynet://5f4dcc3b5aa765d61d8327deb882cf99/user/user_123
```

### 3. 引用頻道

```
skynet://5f4dcc3b5aa765d61d8327deb882cf99/channel/channel_456
```

### 4. 引用訊息

```
skynet://5f4dcc3b5aa765d61d8327deb882cf99/message/msg_789
```

### 5. 引用資源

```
skynet://5f4dcc3b5aa765d61d8327deb882cf99/resource/res_abc
```

### 6. 引用討論串

```
skynet://5f4dcc3b5aa765d61d8327deb882cf99/thread/msg_789
```

### 7. 帶查詢參數

```
skynet://5f4dcc3b5aa765d61d8327deb882cf99/message/msg_789?highlight=true&scroll_to_bottom=true
```

## 編碼規範

- **subnet_id**：使用 Blake3 雜湊值的十六進位表示（小寫）
- **user_id**、**channel_id**、**message_id**、**resource_id**、**thread_id**：使用 URL 安全的 Base64 編碼或十六進位
- **特殊字元**：路徑元件中的特殊字元必須進行 URL 編碼

## 子網隔離原則

Skynet URI 遵循子網隔離原則：
- 所有 URI 都必須包含 `subnet_id`
- 資源不跨子網共用，如需跨子網使用必須複製
- URI 中的資源僅在指定子網內可存取

## 客戶端處理

客戶端在解析 `skynet://` URI 時應：
1. 驗證 URI 格式
2. 檢查目標子網是否已加入
3. 根據資源類型執行相應操作（如跳轉、開啟資源等）
4. 對於未加入的子網，可提供加入引導
