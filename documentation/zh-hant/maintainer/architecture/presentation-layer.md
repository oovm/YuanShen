# 表現層（應用端）

表現層是用戶直接互動的介面，包括各類應用程式和前端介面。本位於三層架構的最頂層，透過呼叫實現層的核心模組來提供服務。

## 整體架構

表現層由兩個主要部分組成：
- **前端應用** - 位於 `frontends/` 目錄，提供用戶介面
- **後端應用** - 位於 `backends/` 目錄，提供服務端支援

### 表現層元件關係圖

```
┌─────────────────────────────────────────────────────────────┐
│                        前端應用 (frontends/)                  │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │  ai-company  │  │  ai-empire   │  │  ai-planet   │    │
│  │  (主應用)     │  │  (帝國模組)   │  │  (星球模組)   │    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │  ai-waifu    │  │  client-h5    │  │  client-desktop│  │
│  │  (角色模組)   │  │  (H5客戶端)   │  │  (桌面客戶端)   │  │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
│  ┌──────────────┐  ┌──────────────┐                        │
│  │client-mobile │  │ client-shared │                        │
│  │(行動端)      │  │ (共享庫)      │                        │
│  └──────────────┘  └──────────────┘                        │
└─────────────────────────────────────────────────────────────┘
                            ↓ HTTP/WebSocket
┌─────────────────────────────────────────────────────────────┐
│                        後端應用 (backends/)                   │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │  ai-company  │  │  ai-empire   │  │  ai-planet   │    │
│  │  (主應用)     │  │  (帝國模組)   │  │  (星球模組)   │    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
│  ┌──────────────┐                                            │
│  │  ai-waifu    │                                            │
│  │  (角色模組)   │                                            │
│  └──────────────┘                                            │
└─────────────────────────────────────────────────────────────┘
                            ↓ 呼叫
┌─────────────────────────────────────────────────────────────┐
│              實現層（augur-* 核心模組）                       │
└─────────────────────────────────────────────────────────────┘
```

## 前端應用詳解

### 1. ai-company（主應用）

**目錄位置**: `frontends/ai-company/`

**技術棧**:
- Vue 3 + TypeScript
- Vite 建構工具
- Element Plus UI 元件庫
- Vue Router 路由
- Pinia 狀態管理
- UnoCSS 原子化 CSS
- Fluent Vue 國際化

**連接埠**: 開發伺服器預設連接埠透過 Vite 設定

**功能職責**:
- 行銷首頁展示
- 用戶登入/註冊
- 公司管理
- 員工管理
- 團隊管理
- 專案管理
- 論壇功能
- 管理員功能

**主要檢視**:
- `Home.vue` - 首頁
- `Login.vue` - 登入頁
- `Register.vue` - 註冊頁
- `Company.vue` - 公司列表
- `CompanyDetail.vue` - 公司詳情
- `Employees.vue` - 員工列表
- `EmployeeDetail.vue` - 員工詳情
- `Team.vue` - 團隊列表
- `TeamDetail.vue` - 團隊詳情
- `Project.vue` - 專案列表
- `ProjectDetail.vue` - 專案詳情
- `Forum.vue` - 論壇
- `ForumDetail.vue` - 論壇詳情
- `Dashboard.vue` - 儀表板
- `Download.vue` - 下載頁
- `admin/AdminDashboard.vue` - 管理員儀表板
- `admin/Users.vue` - 用戶管理

### 2. ai-empire（帝國模組）

**目錄位置**: `frontends/ai-empire/`

**技術棧**:
- Vue 3 + TypeScript
- Vite + Vite-SSG
- Element Plus UI 元件庫
- Vue Router 路由
- Pinia 狀態管理
- UnoCSS 原子化 CSS
- Fluent Vue 國際化
- TipTap 富文本編輯器

**功能職責**:
- 帝國管理介面
- AI 顧問（Advisor）管理
- AI 顧問（Consultant）管理
- 軍團（Legion）管理
- 偉大作品（Great Work）管理
- 用戶認證
- 儀表板展示

**主要檢視**:
- `Home.vue` - 首頁
- `Login.vue` - 登入頁
- `Register.vue` - 註冊頁
- `Empire.vue` - 帝國列表
- `EmpireDetail.vue` - 帝國詳情
- `Advisors.vue` - 顧問列表
- `AdvisorDetail.vue` - 顧問詳情
- `Consultants.vue` - 顧問列表
- `ConsultantDetail.vue` - 顧問詳情
- `Legion.vue` - 軍團列表
- `LegionDetail.vue` - 軍團詳情
- `GreatWork.vue` - 偉大作品列表
- `GreatWorkDetail.vue` - 偉大作品詳情

**特色功能**:
- AI 員工管理：將 AI 智慧體作為企業員工
- 角色化 AI 助手：客戶服務、財務、行銷、行政助理、法律等
- 知識管理系統
- 富文本編輯功能

### 3. ai-planet（星球模組）

**目錄位置**: `frontends/ai-planet/`

**技術棧**:
- Vue 3 + TypeScript
- Vite + Vite-SSG
- Element Plus UI 元件庫
- Vue Router 路由
- Pinia 狀態管理
- UnoCSS 原子化 CSS
- Fluent Vue 國際化
- TipTap 富文本編輯器

**功能職責**:
- 星球相關功能介面
- 用戶認證

**主要檢視**:
- `Home.vue` - 首頁
- `Auth.vue` - 認證頁

### 4. ai-waifu（角色模組）

**目錄位置**: `frontends/ai-waifu/`

**技術棧**:
- Vue 3 + TypeScript
- Vite + Vite-SSG
- Element Plus UI 元件庫
- Vue Router 路由
- Pinia 狀態管理
- UnoCSS 原子化 CSS
- Fluent Vue 國際化
- TipTap 富文本編輯器

**功能職責**:
- AI 角色聊天介面
- 角色互動

**主要檢視**:
- `Home.vue` - 首頁
- `Chat.vue` - 聊天介面

### 5. client-h5（H5 客戶端）

**目錄位置**: `frontends/client-h5/`

**技術棧**:
- Vue 3 + TypeScript
- Vite 建構工具
- Element Plus UI 元件庫
- Vue Router 路由
- Pinia 狀態管理
- UnoCSS 原子化 CSS

**功能職責**:
- 行動端 H5 應用
- 工作台檢視
- Augur 協作模式
- 企業微信風格模式
- 應用管理
- 訊息中心
- 聯絡人管理
- 日曆
- 會議
- 郵箱
- 知識庫
- 記憶管理
- 專案管理

**特色功能**:
- 工作模式切換：Augur 協作模式 / 企業微信風格模式
- 多應用整合：考勤、CRM、合約、財務、HR、績效、招募、Wiki、工作流程等
- 多種佈局模式

### 6. client-desktop（桌面客戶端）

**目錄位置**: `frontends/client-desktop/`

**技術棧**:
- Tauri (Rust + WebView)
- 跨平台桌面應用

**功能職責**:
- 桌面端應用程式
- 提供原生桌面體驗

### 7. client-mobile（行動端）

**目錄位置**: `frontends/client-mobile/`

**技術棧**:
- Tauri (Rust + WebView)
- 行動端應用

**功能職責**:
- 行動端應用程式

### 8. client-shared（共享庫）

**目錄位置**: `frontends/client-shared/`

**技術棧**:
- Vue 3 + TypeScript

**功能職責**:
- 共享元件庫
- 共享服務（API 呼叫）
- 共享狀態管理（Pinia stores）
- 共享型別定義
- 共享工具函式
- 共享國際化資源
- 共享外掛

**包含內容**:
- `components/` - 共享 Vue 元件
- `services/` - API 服務和模擬資料
- `stores/` - Pinia 狀態管理
- `types/` - TypeScript 型別定義
- `utils/` - 工具函式
- `locales/` - 國際化資源
- `plugins/` - Vue 外掛

## 後端應用詳解

### 1. ai-company（主應用後端）

**目錄位置**: `backends/ai-company/`

**技術棧**:
- Rust + Tokio 非同步執行時
- Axum Web 框架
- Serde 序列化
- Tracing 日誌
- rust-embed 靜態資源嵌入

**連接埠**: 4002

**功能職責**:
- 提供 HTTP 服務
- 嵌入並提供前端靜態資源
- 路由和靜態檔案服務
- 為主應用前端提供服務端支援

**主要功能**:
- 靜態檔案服務：嵌入 `frontends/ai-company/dist` 目錄的前端建構產物
- 單頁應用路由支援：所有未匹配的路徑都回傳 index.html
- 靜態資源 MIME 型別自動檢測

### 2. ai-empire（帝國模組後端）

**目錄位置**: `backends/ai-empire/`

**技術棧**:
- Rust + Tokio 非同步執行時
- Axum Web 框架
- Serde 序列化
- Tracing 日誌
- rust-embed 靜態資源嵌入

**功能職責**:
- 提供 HTTP 服務
- 嵌入並提供帝國模組前端靜態資源
- 為帝國模組前端提供服務端支援

### 3. ai-planet（星球模組後端）

**目錄位置**: `backends/ai-planet/`

**技術棧**:
- Rust + Tokio 非同步執行時
- Axum Web 框架
- Serde 序列化
- Tracing 日誌
- rust-embed 靜態資源嵌入

**功能職責**:
- 提供 HTTP 服務
- 嵌入並提供星球模組前端靜態資源
- 為星球模組前端提供服務端支援

### 4. ai-waifu（角色模組後端）

**目錄位置**: `backends/ai-waifu/`

**技術棧**:
- Rust + Tokio 非同步執行時
- Axum Web 框架
- Serde 序列化
- Tracing 日誌
- rust-embed 靜態資源嵌入

**功能職責**:
- 提供 HTTP 服務
- 嵌入並提供角色模組前端靜態資源
- 為角色模組前端提供服務端支援

## 應用間關係

### 前端應用關係

```
client-shared (共享庫)
    ↑ 相依
    ├─→ ai-company (主應用)
    ├─→ ai-empire (帝國模組)
    ├─→ ai-planet (星球模組)
    ├─→ ai-waifu (角色模組)
    └─→ client-h5 (H5 客戶端)

client-h5 (H5 客戶端)
    ↑ 參考/靈感
    ├─→ client-desktop (桌面客戶端)
    └─→ client-mobile (行動端)
```

### 前後端配對關係

| 前端應用 | 後端應用 | 說明 |
|---------|---------|------|
| `ai-company` | `ai-company` | 主應用前後端配對 |
| `ai-empire` | `ai-empire` | 帝國模組前後端配對 |
| `ai-planet` | `ai-planet` | 星球模組前後端配對 |
| `ai-waifu` | `ai-waifu` | 角色模組前後端配對 |
| `client-h5` | (待定) | H5 客戶端後端 |
| `client-desktop` | (待定) | 桌面客戶端後端 |
| `client-mobile` | (待定) | 行動端後端 |

### 與實現層的關係

所有後端應用最終都呼叫實現層的 `augur-*` 核心模組來執行業務邏輯：

```
表現層（後端應用）
    ↓ 呼叫
實現層（augur-* 核心模組）
    ↓ 使用
協定層（Skynet 協定）
```

## 部署架構

### 開發環境

- 前端應用透過 Vite 開發伺服器獨立執行
- 後端應用透過 Cargo 獨立執行
- 前後端透過 API 呼叫通訊

### 生產環境

- 前端建構產物嵌入到對應後端應用中（透過 rust-embed）
- 每個後端應用作為獨立服務執行，提供完整的前後端功能
- 各服務監聽不同連接埠

## 技術選型說明

### 前端技術選型

- **Vue 3**: 漸進式 JavaScript 框架，提供優秀的開發體驗和效能
- **TypeScript**: 提供型別安全，提升程式碼可維護性
- **Vite**: 下一代前端建構工具，提供極快的開發體驗
- **Element Plus**: 基於 Vue 3 的元件庫，提供豐富的 UI 元件
- **Pinia**: Vue 3 官方推薦的狀態管理庫
- **UnoCSS**: 原子化 CSS 引擎，提供靈活的樣式方案
- **Fluent Vue**: 國際化方案，支援多語言

### 後端技術選型

- **Rust**: 高效能、記憶體安全的系統程式語言
- **Tokio**: Rust 非同步執行時，提供高效能的 I/O 處理
- **Axum**:  ergonomic and modular web framework，提供良好的開發體驗
- **rust-embed**: 將前端建構產物嵌入到 Rust 二進位檔案中，簡化部署

## 開發指南

### 前端開發

```bash
# 進入前端應用目錄
cd frontends/ai-company

# 安裝相依
npm install

# 啟動開發伺服器
npm run dev

# 建構生產版本
npm run build
```

### 後端開發

```bash
# 進入後端應用目錄
cd backends/ai-company

# 執行開發伺服器
cargo run

# 建構生產版本
cargo build --release
```

### 完整開發流程

1. 先建構前端應用
2. 再執行後端應用（會嵌入前端建構產物）

```bash
# 建構前端
cd frontends/ai-company
npm run build

# 執行後端
cd ../../backends/ai-company
cargo run
```
