# 实现层（augur-* 核心模块）

本文档详细描述 AI Company 系统中实现层的各个核心模块。实现层位于表现层和协议层之间，提供核心业务逻辑和功能实现。

## 概述

实现层由多个独立的 `augur-*` 模块组成，每个模块负责特定的功能领域。这些模块通过依赖注入和清晰的接口设计进行协作，为上层应用提供坚实的功能基础。

## 模块列表

实现层包含以下核心模块：

| 模块名称 | 职责 | 状态 |
|---------|------|------|
| [augur-agent](#augur-agent) | 智能体管理 | 核心 |
| [augur-orchestrator](#augur-orchestrator) | 任务编排和工作流管理 | 核心 |
| [augur-organization](#augur-organization) | 组织、部门、角色管理 | 核心 |
| [augur-skill](#augur-skill) | 技能插件和能力扩展 | 核心 |
| [augur-memory](#augur-memory) | 记忆系统管理 | 核心 |
| [augur-persistence](#augur-persistence) | 持久化层实现 | 基础设施 |
| [augur-file-system](#augur-file-system) | 文件存储服务 | 基础设施 |
| [augur-types](#augur-types) | 类型定义和错误处理 | 基础设施 |

## 模块依赖关系图

```
┌─────────────────────────────────────────────────────────────────┐
│                        业务逻辑层                                  │
├─────────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │augur-agent   │  │augur-orchestr│  │augur-organiz │          │
│  │(智能体管理)   │  │ator(编排器)  │  │ation(组织)   │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │augur-skill   │  │augur-memory  │  │              │          │
│  │(技能管理)     │  │(记忆系统)     │  │              │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
└─────────────────────────────────────────────────────────────────┘
                            ↓ 依赖
┌─────────────────────────────────────────────────────────────────┐
│                        基础设施层                                  │
├─────────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │augur-persist │  │augur-file-sys│  │augur-types   │          │
│  │ence(持久化)  │  │tem(文件系统)  │  │(类型定义)     │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
└─────────────────────────────────────────────────────────────────┘
                            ↓ 使用
┌─────────────────────────────────────────────────────────────────┐
│                      协议层（Skynet）                              │
└─────────────────────────────────────────────────────────────────┘
```

## 模块详细说明

### augur-types

**职责：** 提供实现层的共享类型定义和统一的错误处理机制。

**主要功能：**
- 定义核心数据结构（工作区、项目、公司、团队、员工、技能等）
- 统一错误类型 `AugurError` 和错误处理机制
- 提供标准的枚举类型（如 `ProjectStatus`、`RoleType`、`SkillLevel` 等）
- 支持国际化的错误消息（通过 i18n key）

**核心结构体：**
- `Workspace` - 工作区
- `Project` - 项目
- `Company` - 公司
- `Team` - 团队
- `Employee` - 员工
- `Skill` - 技能

**依赖关系：** 无其他 augur-* 模块依赖，是所有其他模块的基础。

### augur-persistence

**职责：** 提供统一的持久化层实现，支持多种数据库后端。

**主要功能：**
- 多数据库支持（SQLite、PostgreSQL）
- 统一的 Repository 接口设计
- 数据库迁移管理
- 实体类型持久化支持

**支持的实体类型：**
- 用户
- 组织
- 部门
- 角色
- 对话
- 消息
- 智能体
- 记忆
- 记忆标签

**使用示例：**
```rust
use augur_persistence::{AugurPersistence, PersistenceConfig, DatabaseType};

let config = PersistenceConfig::default();
let persistence = AugurPersistence::new(config).await?;

let user_repo = persistence.user_repository();
```

**依赖关系：** 依赖 `augur-types`，被业务逻辑模块依赖。

### augur-file-system

**职责：** 提供基于本地文件系统的文件存储服务。

**主要功能：**
- 文件上传和下载
- 文件元数据管理
- 文件访问级别控制
- 文件搜索和过滤
- 用户和组织文件列表
- 文件复制和移动操作
- 多种 MIME 类型检测支持

**使用示例：**
```rust
use augur_file_system::{FsFileService, FsFileServiceConfig};

let config = FsFileServiceConfig::default();
let service = FsFileService::new(config).await?;
```

**依赖关系：** 依赖 `augur-types`，被业务逻辑模块依赖。

### augur-memory

**职责：** 实现记忆系统，支持记忆的创建、存储、检索和管理。

**主要功能：**
- 记忆 CRUD 操作
- 记忆标签管理
- 记忆关系管理
- 关键词搜索
- 标签和时间范围过滤
- 记忆导入和导出
- 对话上下文提取
- 从对话创建记忆

**使用示例：**
```rust
use augur_memory::AugurMemory;
use std::sync::Arc;

let memory_service = AugurMemory::new(
    Arc::new(memory_repository),
    Arc::new(memory_tag_repository),
);
```

**依赖关系：** 依赖 `augur-types`、`augur-persistence`，被 `augur-agent` 等依赖。

### augur-agent

**职责：** 提供智能体（Agent）相关的类型定义和接口，管理智能体的生命周期。

**主要功能：**
- 智能体类型定义
- 智能体状态管理
- 智能体配置接口
- 智能体生命周期管理

**依赖关系：** 依赖 `augur-types`、`augur-memory`、`augur-skill`，被上层应用依赖。

### augur-orchestrator

**职责：** 提供任务编排和工作流管理的类型和接口。

**主要功能：**
- 任务编排接口
- 工作流管理
- 任务调度
- 依赖管理

**依赖关系：** 依赖 `augur-types`，被上层应用依赖。

### augur-organization

**职责：** 提供组织、部门和角色管理的类型和接口。

**主要功能：**
- 组织管理
- 部门结构
- 角色权限
- 成员管理

**依赖关系：** 依赖 `augur-types`、`augur-persistence`，被上层应用依赖。

### augur-skill

**职责：** 提供技能插件和能力扩展的类型和接口。

**主要功能：**
- 技能插件接口
- 技能定义
- 技能执行
- 插件管理

**依赖关系：** 依赖 `augur-types`，被 `augur-agent` 依赖。

## 模块间协作示例

### 示例 1：智能体执行任务

1. 上层应用调用 `augur-orchestrator` 发起任务
2. `augur-orchestrator` 查找可用的 `augur-agent` 智能体
3. `augur-agent` 使用 `augur-skill` 加载所需技能
4. 执行过程中，`augur-agent` 使用 `augur-memory` 记录和检索记忆
5. 所有状态变更通过 `augur-persistence` 持久化

### 示例 2：组织协作

1. 上层应用通过 `augur-organization` 管理组织架构
2. `augur-organization` 使用 `augur-persistence` 存储组织数据
3. 用户上传文件通过 `augur-file-system` 管理
4. 组织内的智能体通过 `augur-agent` 进行协作

## 设计原则

实现层遵循以下设计原则：

1. **单一职责原则** - 每个模块只负责一个明确的功能领域
2. **依赖倒置原则** - 高层模块不依赖低层模块，都依赖于抽象
3. **接口隔离原则** - 提供最小化的接口，避免胖接口
4. **模块化设计** - 模块间通过清晰的边界和依赖关系进行协作
5. **可测试性** - 每个模块都应该易于独立测试

## 扩展指南

当需要添加新功能时，请遵循以下步骤：

1. 确定功能属于哪个现有模块，或是否需要创建新模块
2. 在 `augur-types` 中定义必要的数据结构和错误类型
3. 在对应的业务模块中实现核心逻辑
4. 如需持久化，在 `augur-persistence` 中添加 Repository
5. 如需文件存储，使用 `augur-file-system`
6. 更新本文档，记录新增功能的说明

## 相关文档

- [架构概览](./index.md)
- [去中心化设计](./decentralization.md)
- [安全模型](./security-model.md)
- [数据模型](../data-models.md)
- [Skynet 协议](../skynet/index.md)
