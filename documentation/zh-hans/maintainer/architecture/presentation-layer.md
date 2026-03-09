# 表现层（应用端）

表现层是用户直接交互的界面，包括各类应用程序和前端界面。本层位于三层架构的最顶层，通过调用实现层的核心模块来提供服务。

## 整体架构

表现层由两个主要部分组成：
- **前端应用** - 位于 `frontends/` 目录，提供用户界面
- **后端应用** - 位于 `backends/` 目录，提供服务端支持

### 表现层组件关系图

```
┌─────────────────────────────────────────────────────────────┐
│                        前端应用 (frontends/)                  │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │  ai-company  │  │  ai-empire   │  │  ai-planet   │    │
│  │  (主应用)     │  │  (帝国模块)   │  │  (星球模块)   │    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │  ai-waifu    │  │  client-h5    │  │  client-desktop│  │
│  │  (角色模块)   │  │  (H5客户端)   │  │  (桌面客户端)   │  │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
│  ┌──────────────┐  ┌──────────────┐                        │
│  │client-mobile │  │ client-shared │                        │
│  │(移动端)      │  │ (共享库)      │                        │
│  └──────────────┘  └──────────────┘                        │
└─────────────────────────────────────────────────────────────┘
                            ↓ HTTP/WebSocket
┌─────────────────────────────────────────────────────────────┐
│                        后端应用 (backends/)                   │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │  ai-company  │  │  ai-empire   │  │  ai-planet   │    │
│  │  (主应用)     │  │  (帝国模块)   │  │  (星球模块)   │    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
│  ┌──────────────┐                                            │
│  │  ai-waifu    │                                            │
│  │  (角色模块)   │                                            │
│  └──────────────┘                                            │
└─────────────────────────────────────────────────────────────┘
                            ↓ 调用
┌─────────────────────────────────────────────────────────────┐
│              实现层（augur-* 核心模块）                       │
└─────────────────────────────────────────────────────────────┘
```

## 前端应用详解

### 1. ai-company（主应用）

**目录位置**: `frontends/ai-company/`

**技术栈**:
- Vue 3 + TypeScript
- Vite 构建工具
- Element Plus UI 组件库
- Vue Router 路由
- Pinia 状态管理
- UnoCSS 原子化 CSS
- Fluent Vue 国际化

**端口**: 开发服务器默认端口通过 Vite 配置

**功能职责**:
- 营销首页展示
- 用户登录/注册
- 公司管理
- 员工管理
- 团队管理
- 项目管理
- 论坛功能
- 管理员功能

**主要视图**:
- `Home.vue` - 首页
- `Login.vue` - 登录页
- `Register.vue` - 注册页
- `Company.vue` - 公司列表
- `CompanyDetail.vue` - 公司详情
- `Employees.vue` - 员工列表
- `EmployeeDetail.vue` - 员工详情
- `Team.vue` - 团队列表
- `TeamDetail.vue` - 团队详情
- `Project.vue` - 项目列表
- `ProjectDetail.vue` - 项目详情
- `Forum.vue` - 论坛
- `ForumDetail.vue` - 论坛详情
- `Dashboard.vue` - 仪表板
- `Download.vue` - 下载页
- `admin/AdminDashboard.vue` - 管理员仪表板
- `admin/Users.vue` - 用户管理

### 2. ai-empire（帝国模块）

**目录位置**: `frontends/ai-empire/`

**技术栈**:
- Vue 3 + TypeScript
- Vite + Vite-SSG
- Element Plus UI 组件库
- Vue Router 路由
- Pinia 状态管理
- UnoCSS 原子化 CSS
- Fluent Vue 国际化
- TipTap 富文本编辑器

**功能职责**:
- 帝国管理界面
- AI 顾问（Advisor）管理
- AI 顾问（Consultant）管理
- 军团（Legion）管理
- 伟大作品（Great Work）管理
- 用户认证
- 仪表板展示

**主要视图**:
- `Home.vue` - 首页
- `Login.vue` - 登录页
- `Register.vue` - 注册页
- `Empire.vue` - 帝国列表
- `EmpireDetail.vue` - 帝国详情
- `Advisors.vue` - 顾问列表
- `AdvisorDetail.vue` - 顾问详情
- `Consultants.vue` - 顾问列表
- `ConsultantDetail.vue` - 顾问详情
- `Legion.vue` - 军团列表
- `LegionDetail.vue` - 军团详情
- `GreatWork.vue` - 伟大作品列表
- `GreatWorkDetail.vue` - 伟大作品详情

**特色功能**:
- AI 员工管理：将 AI 智能体作为企业员工
- 角色化 AI 助手：客户服务、财务、营销、行政助理、法律等
- 知识管理系统
- 富文本编辑功能

### 3. ai-planet（星球模块）

**目录位置**: `frontends/ai-planet/`

**技术栈**:
- Vue 3 + TypeScript
- Vite + Vite-SSG
- Element Plus UI 组件库
- Vue Router 路由
- Pinia 状态管理
- UnoCSS 原子化 CSS
- Fluent Vue 国际化
- TipTap 富文本编辑器

**功能职责**:
- 星球相关功能界面
- 用户认证

**主要视图**:
- `Home.vue` - 首页
- `Auth.vue` - 认证页

### 4. ai-waifu（角色模块）

**目录位置**: `frontends/ai-waifu/`

**技术栈**:
- Vue 3 + TypeScript
- Vite + Vite-SSG
- Element Plus UI 组件库
- Vue Router 路由
- Pinia 状态管理
- UnoCSS 原子化 CSS
- Fluent Vue 国际化
- TipTap 富文本编辑器

**功能职责**:
- AI 角色聊天界面
- 角色交互

**主要视图**:
- `Home.vue` - 首页
- `Chat.vue` - 聊天界面

### 5. client-h5（H5 客户端）

**目录位置**: `frontends/client-h5/`

**技术栈**:
- Vue 3 + TypeScript
- Vite 构建工具
- Element Plus UI 组件库
- Vue Router 路由
- Pinia 状态管理
- UnoCSS 原子化 CSS

**功能职责**:
- 移动端 H5 应用
- 工作台视图
- Augur 协作模式
- 企业微信风格模式
- 应用管理
- 消息中心
- 联系人管理
- 日历
- 会议
- 邮箱
- 知识库
- 记忆管理
- 项目管理

**特色功能**:
- 工作模式切换：Augur 协作模式 / 企业微信风格模式
- 多应用集成：考勤、CRM、合同、财务、HR、绩效、招聘、Wiki、工作流等
- 多种布局模式

### 6. client-desktop（桌面客户端）

**目录位置**: `frontends/client-desktop/`

**技术栈**:
- Tauri (Rust + WebView)
- 跨平台桌面应用

**功能职责**:
- 桌面端应用程序
- 提供原生桌面体验

### 7. client-mobile（移动端）

**目录位置**: `frontends/client-mobile/`

**技术栈**:
- Tauri (Rust + WebView)
- 移动端应用

**功能职责**:
- 移动端应用程序

### 8. client-shared（共享库）

**目录位置**: `frontends/client-shared/`

**技术栈**:
- Vue 3 + TypeScript

**功能职责**:
- 共享组件库
- 共享服务（API 调用）
- 共享状态管理（Pinia stores）
- 共享类型定义
- 共享工具函数
- 共享国际化资源
- 共享插件

**包含内容**:
- `components/` - 共享 Vue 组件
- `services/` - API 服务和模拟数据
- `stores/` - Pinia 状态管理
- `types/` - TypeScript 类型定义
- `utils/` - 工具函数
- `locales/` - 国际化资源
- `plugins/` - Vue 插件

## 后端应用详解

### 1. ai-company（主应用后端）

**目录位置**: `backends/ai-company/`

**技术栈**:
- Rust + Tokio 异步运行时
- Axum Web 框架
- Serde 序列化
- Tracing 日志
- rust-embed 静态资源嵌入

**端口**: 4002

**功能职责**:
- 提供 HTTP 服务
- 嵌入并提供前端静态资源
- 路由和静态文件服务
- 为主应用前端提供服务端支持

**主要功能**:
- 静态文件服务：嵌入 `frontends/ai-company/dist` 目录的前端构建产物
- 单页应用路由支持：所有未匹配的路径都返回 index.html
- 静态资源 MIME 类型自动检测

### 2. ai-empire（帝国模块后端）

**目录位置**: `backends/ai-empire/`

**技术栈**:
- Rust + Tokio 异步运行时
- Axum Web 框架
- Serde 序列化
- Tracing 日志
- rust-embed 静态资源嵌入

**功能职责**:
- 提供 HTTP 服务
- 嵌入并提供帝国模块前端静态资源
- 为帝国模块前端提供服务端支持

### 3. ai-planet（星球模块后端）

**目录位置**: `backends/ai-planet/`

**技术栈**:
- Rust + Tokio 异步运行时
- Axum Web 框架
- Serde 序列化
- Tracing 日志
- rust-embed 静态资源嵌入

**功能职责**:
- 提供 HTTP 服务
- 嵌入并提供星球模块前端静态资源
- 为星球模块前端提供服务端支持

### 4. ai-waifu（角色模块后端）

**目录位置**: `backends/ai-waifu/`

**技术栈**:
- Rust + Tokio 异步运行时
- Axum Web 框架
- Serde 序列化
- Tracing 日志
- rust-embed 静态资源嵌入

**功能职责**:
- 提供 HTTP 服务
- 嵌入并提供角色模块前端静态资源
- 为角色模块前端提供服务端支持

## 应用间关系

### 前端应用关系

```
client-shared (共享库)
    ↑ 依赖
    ├─→ ai-company (主应用)
    ├─→ ai-empire (帝国模块)
    ├─→ ai-planet (星球模块)
    ├─→ ai-waifu (角色模块)
    └─→ client-h5 (H5 客户端)

client-h5 (H5 客户端)
    ↑ 参考/灵感
    ├─→ client-desktop (桌面客户端)
    └─→ client-mobile (移动端)
```

### 前后端配对关系

| 前端应用 | 后端应用 | 说明 |
|---------|---------|------|
| `ai-company` | `ai-company` | 主应用前后端配对 |
| `ai-empire` | `ai-empire` | 帝国模块前后端配对 |
| `ai-planet` | `ai-planet` | 星球模块前后端配对 |
| `ai-waifu` | `ai-waifu` | 角色模块前后端配对 |
| `client-h5` | (待定) | H5 客户端后端 |
| `client-desktop` | (待定) | 桌面客户端后端 |
| `client-mobile` | (待定) | 移动端后端 |

### 与实现层的关系

所有后端应用最终都调用实现层的 `augur-*` 核心模块来执行业务逻辑：

```
表现层（后端应用）
    ↓ 调用
实现层（augur-* 核心模块）
    ↓ 使用
协议层（Skynet 协议）
```

## 部署架构

### 开发环境

- 前端应用通过 Vite 开发服务器独立运行
- 后端应用通过 Cargo 独立运行
- 前后端通过 API 调用通信

### 生产环境

- 前端构建产物嵌入到对应后端应用中（通过 rust-embed）
- 每个后端应用作为独立服务运行，提供完整的前后端功能
- 各服务监听不同端口

## 技术选型说明

### 前端技术选型

- **Vue 3**: 渐进式 JavaScript 框架，提供优秀的开发体验和性能
- **TypeScript**: 提供类型安全，提升代码可维护性
- **Vite**: 下一代前端构建工具，提供极快的开发体验
- **Element Plus**: 基于 Vue 3 的组件库，提供丰富的 UI 组件
- **Pinia**: Vue 3 官方推荐的状态管理库
- **UnoCSS**: 原子化 CSS 引擎，提供灵活的样式方案
- **Fluent Vue**: 国际化方案，支持多语言

### 后端技术选型

- **Rust**: 高性能、内存安全的系统编程语言
- **Tokio**: Rust 异步运行时，提供高性能的 I/O 处理
- **Axum**:  ergonomic and modular web framework，提供良好的开发体验
- **rust-embed**: 将前端构建产物嵌入到 Rust 二进制文件中，简化部署

## 开发指南

### 前端开发

```bash
# 进入前端应用目录
cd frontends/ai-company

# 安装依赖
npm install

# 启动开发服务器
npm run dev

# 构建生产版本
npm run build
```

### 后端开发

```bash
# 进入后端应用目录
cd backends/ai-company

# 运行开发服务器
cargo run

# 构建生产版本
cargo build --release
```

### 完整开发流程

1. 先构建前端应用
2. 再运行后端应用（会嵌入前端构建产物）

```bash
# 构建前端
cd frontends/ai-company
npm run build

# 运行后端
cd ../../backends/ai-company
cargo run
```
