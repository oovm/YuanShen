# 實現層（augur-* 核心模組）

本文件詳細描述 AI Company 系統中實現層的各個核心模組。實現層位於表現層和協定層之間，提供核心業務邏輯和功能實現。

## 概述

實現層由多個獨立的 `augur-*` 模組組成，每個模組負責特定的功能領域。這些模組透過相依性注入和清晰的介面設計進行協作，為上層應用提供堅實的功能基礎。

## 模組列表

實現層包含以下核心模組：

| 模組名稱 | 職責 | 狀態 |
|---------|------|------|
| [augur-agent](#augur-agent) | 智慧體管理 | 核心 |
| [augur-orchestrator](#augur-orchestrator) | 任務編排和工作流程管理 | 核心 |
| [augur-organization](#augur-organization) | 組織、部門、角色管理 | 核心 |
| [augur-skill](#augur-skill) | 技能外掛和能力擴展 | 核心 |
| [augur-memory](#augur-memory) | 記憶系統管理 | 核心 |
| [augur-persistence](#augur-persistence) | 持久化層實現 | 基礎設施 |
| [augur-file-system](#augur-file-system) | 檔案儲存服務 | 基礎設施 |
| [augur-types](#augur-types) | 型別定義和錯誤處理 | 基礎設施 |

## 模組相依關係圖

```
┌─────────────────────────────────────────────────────────────────┐
│                        業務邏輯層                                  │
├─────────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │augur-agent   │  │augur-orchestr│  │augur-organiz │          │
│  │(智慧體管理)   │  │ator(編排器)  │  │ation(組織)   │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │augur-skill   │  │augur-memory  │  │              │          │
│  │(技能管理)     │  │(記憶系統)     │  │              │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
└─────────────────────────────────────────────────────────────────┘
                            ↓ 相依
┌─────────────────────────────────────────────────────────────────┐
│                        基礎設施層                                  │
├─────────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │augur-persist │  │augur-file-sys│  │augur-types   │          │
│  │ence(持久化)  │  │tem(檔案系統)  │  │(型別定義)     │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
└─────────────────────────────────────────────────────────────────┘
                            ↓ 使用
┌─────────────────────────────────────────────────────────────────┐
│                      協定層（Skynet）                              │
└─────────────────────────────────────────────────────────────────┘
```

## 模組詳細說明

### augur-types

**職責：** 提供實現層的共享型別定義和統一的錯誤處理機制。

**主要功能：**
- 定義核心資料結構（工作區、專案、公司、團隊、員工、技能等）
- 統一錯誤型別 `AugurError` 和錯誤處理機制
- 提供標準的列舉型別（如 `ProjectStatus`、`RoleType`、`SkillLevel` 等）
- 支援國際化的錯誤訊息（透過 i18n key）

**核心結構體：**
- `Workspace` - 工作區
- `Project` - 專案
- `Company` - 公司
- `Team` - 團隊
- `Employee` - 員工
- `Skill` - 技能

**相依關係：** 無其他 augur-* 模組相依，是所有其他模組的基礎。

### augur-persistence

**職責：** 提供統一的持久化層實現，支援多種資料庫後端。

**主要功能：**
- 多資料庫支援（SQLite、PostgreSQL）
- 統一的 Repository 介面設計
- 資料庫遷移管理
- 實體型別持久化支援

**支援的實體型別：**
- 用戶
- 組織
- 部門
- 角色
- 對話
- 訊息
- 智慧體
- 記憶
- 記憶標籤

**使用範例：**
```rust
use augur_persistence::{AugurPersistence, PersistenceConfig, DatabaseType};

let config = PersistenceConfig::default();
let persistence = AugurPersistence::new(config).await?;

let user_repo = persistence.user_repository();
```

**相依關係：** 相依 `augur-types`，被業務邏輯模組相依。

### augur-file-system

**職責：** 提供基於本地檔案系統的檔案儲存服務。

**主要功能：**
- 檔案上傳和下載
- 檔案中繼資料管理
- 檔案存取層級控制
- 檔案搜尋和篩選
- 用戶和組織檔案列表
- 檔案複製和移動操作
- 多種 MIME 型別檢測支援

**使用範例：**
```rust
use augur_file_system::{FsFileService, FsFileServiceConfig};

let config = FsFileServiceConfig::default();
let service = FsFileService::new(config).await?;
```

**相依關係：** 相依 `augur-types`，被業務邏輯模組相依。

### augur-memory

**職責：** 實現記憶系統，支援記憶的建立、儲存、檢索和管理。

**主要功能：**
- 記憶 CRUD 操作
- 記憶標籤管理
- 記憶關聯管理
- 關鍵詞搜尋
- 標籤和時間範圍篩選
- 記憶匯入和匯出
- 對話上下文擷取
- 從對話建立記憶

**使用範例：**
```rust
use augur_memory::AugurMemory;
use std::sync::Arc;

let memory_service = AugurMemory::new(
    Arc::new(memory_repository),
    Arc::new(memory_tag_repository),
);
```

**相依關係：** 相依 `augur-types`、`augur-persistence`，被 `augur-agent` 等相依。

### augur-agent

**職責：** 提供智慧體（Agent）相關的型別定義和介面，管理智慧體的生命週期。

**主要功能：**
- 智慧體型別定義
- 智慧體狀態管理
- 智慧體設定介面
- 智慧體生命週期管理

**相依關係：** 相依 `augur-types`、`augur-memory`、`augur-skill`，被上層應用相依。

### augur-orchestrator

**職責：** 提供任務編排和工作流程管理的型別和介面。

**主要功能：**
- 任務編排介面
- 工作流程管理
- 任務排程
- 相依性管理

**相依關係：** 相依 `augur-types`，被上層應用相依。

### augur-organization

**職責：** 提供組織、部門和角色管理的型別和介面。

**主要功能：**
- 組織管理
- 部門結構
- 角色權限
- 成員管理

**相依關係：** 相依 `augur-types`、`augur-persistence`，被上層應用相依。

### augur-skill

**職責：** 提供技能外掛和能力擴展的型別和介面。

**主要功能：**
- 技能外掛介面
- 技能定義
- 技能執行
- 外掛管理

**相依關係：** 相依 `augur-types`，被 `augur-agent` 相依。

## 模組間協作範例

### 範例 1：智慧體執行任務

1. 上層應用呼叫 `augur-orchestrator` 發起任務
2. `augur-orchestrator` 查找可用的 `augur-agent` 智慧體
3. `augur-agent` 使用 `augur-skill` 載入所需技能
4. 執行過程中，`augur-agent` 使用 `augur-memory` 記錄和檢索記憶
5. 所有狀態變更透過 `augur-persistence` 持久化

### 範例 2：組織協作

1. 上層應用透過 `augur-organization` 管理組織架構
2. `augur-organization` 使用 `augur-persistence` 儲存組織資料
3. 用戶上傳檔案透過 `augur-file-system` 管理
4. 組織內的智慧體透過 `augur-agent` 進行協作

## 設計原則

實現層遵循以下設計原則：

1. **單一職責原則** - 每個模組只負責一個明確的功能領域
2. **相依反轉原則** - 高層模組不依賴低層模組，都依賴於抽象
3. **介面隔離原則** - 提供最小化的介面，避免胖介面
4. **模組化設計** - 模組間透過清晰的邊界和相依關係進行協作
5. **可測試性** - 每個模組都應該易於獨立測試

## 擴展指南

當需要添加新功能時，請遵循以下步驟：

1. 確定功能屬於哪個現有模組，或是否需要建立新模組
2. 在 `augur-types` 中定義必要的資料結構和錯誤型別
3. 在對應的業務模組中實現核心邏輯
4. 如需持久化，在 `augur-persistence` 中添加 Repository
5. 如需檔案儲存，使用 `augur-file-system`
6. 更新本文件，記錄新增功能的說明

## 相關文件

- [架構概覽](./index.md)
- [去中心化設計](./decentralization.md)
- [安全模型](./security-model.md)
- [資料模型](../data-models.md)
- [Skynet 協定](../skynet/index.md)
