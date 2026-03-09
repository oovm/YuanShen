# 数据模型与存储

## 数据模型概述

AI Company 系统采用分层的数据模型设计，确保数据的组织清晰、访问高效、安全可靠。数据模型围绕四大核心概念（公司、项目、团队、员工）构建，同时支持工作节点和工作区的分布式存储。

## 核心数据模型

### 公司（Company）数据模型

```
Company {
  id: 唯一标识
  name: 公司名称
  tagline: 公司标语
  logo: 公司 Logo
  description: 公司描述
  industry: 所属行业
  founded: 成立时间
  ownerId: 所有者 ID
  createdAt: 创建时间
  updatedAt: 更新时间
}
```

#### 组织架构数据
```
OrganizationStructure {
  companyId: 所属公司 ID
  hierarchy: 层级结构
  departments: 部门列表
  positions: 职位体系
  permissions: 权限模型
}
```

#### 企业文化数据
```
CompanyCulture {
  companyId: 所属公司 ID
  mission: 使命宣言
  vision: 愿景描述
  values: 核心价值观
  codeOfConduct: 行为准则
}
```

#### 协作原则数据
```
CollaborationPrinciples {
  companyId: 所属公司 ID
  decisionMaking: 决策方式
  communication: 沟通规范
  knowledgeManagement: 知识管理
  qualityStandards: 质量标准
}
```

### 项目（Project）数据模型

```
Project {
  id: 唯一标识
  name: 项目名称
  description: 项目描述
  companyId: 所属公司 ID
  responsibleTeamId: 负责团队 ID
  status: 项目状态
  startDate: 开始时间
  endDate: 结束时间
  priority: 优先级
  createdAt: 创建时间
  updatedAt: 更新时间
}
```

#### 项目结构数据
```
ProjectStructure {
  projectId: 所属项目 ID
  phases: 阶段列表
  milestones: 里程碑列表
  tasks: 任务分解
  dependencies: 依赖关系
}
```

#### 标准作业流程（SOP）数据
```
StandardOperatingProcedure {
  projectId: 所属项目 ID
  phaseFlows: 阶段流程
  deliverables: 交付物清单
  acceptanceCriteria: 验收标准
  approvalNodes: 审批节点
}
```

#### 团队与子团队数据
```
TeamHierarchy {
  projectId: 所属项目 ID
  mainTeam: 主负责团队
  subTeams: 子团队列表
  collaborationRules: 协作规则
}
```

#### 资源配置数据
```
ResourceConfig {
  projectId: 所属项目 ID
  requiredSkills: 所需技能
  timeEstimates: 时间估算
  budgetPlan: 预算规划
}
```

#### 风险管理数据
```
RiskManagement {
  projectId: 所属项目 ID
  risks: 风险识别
  strategies: 应对策略
  contingencyPlans: 应急预案
  qualityStandards: 质量标准
}
```

### 团队（Team）数据模型

```
Team {
  id: 唯一标识
  name: 团队名称
  description: 团队描述
  companyId: 所属公司 ID
  type: 团队类型
  parentTeamId: 父团队 ID（子团队时使用）
  createdAt: 创建时间
  updatedAt: 更新时间
}
```

#### 团队成员数据
```
TeamMember {
  teamId: 所属团队 ID
  agentId: 智能体 ID
  role: 角色
  responsibilities: 职责列表
  authority: 权限列表
}
```

#### 角色分工数据
```
RoleDivision {
  teamId: 所属团队 ID
  leader: 团队领导
  experts: 专家成员
  coordinators: 协调成员
  supporters: 支持成员
}
```

#### 协作模式数据
```
CollaborationMode {
  teamId: 所属团队 ID
  reportingLines: 汇报关系
  decisionMechanism: 决策机制
  communicationChannels: 沟通渠道
  meetingRhythm: 会议节奏
}
```

#### 团队文化数据
```
TeamCulture {
  teamId: 所属团队 ID
  collaborationPrinciples: 协作原则
  conflictResolution: 冲突处理
  knowledgeSharing: 知识共享
  teamSpirit: 团队精神
}
```

#### 子团队管理数据
```
SubTeamManagement {
  teamId: 所属团队 ID
  subTeams: 子团队列表（包含单个员工）
  invocationRules: 子团队调用规则
  flexibleConfig: 灵活配置
  hierarchicalCollaboration: 层级协作
}
```

### 员工（Employee）数据模型

```
Employee {
  id: 唯一标识
  name: 员工名称
  avatar: 员工头像
  title: 职位
  description: 描述
  roleType: 角色类型
  companyId: 所属公司 ID
  createdAt: 创建时间
  updatedAt: 更新时间
}
```

#### 技能体系数据
```
SkillSystem {
  employeeId: 所属员工 ID
  coreSkills: 核心技能
  professionalSkills: 专业技能
  toolSkills: 工具技能
  softSkills: 软技能
}
```

#### 技能评级数据
```
SkillRating {
  employeeId: 所属员工 ID
  skillId: 技能 ID
  level: 技能级别
  lastUpdated: 最后更新时间
}
```

#### 工作风格数据
```
WorkStyle {
  employeeId: 所属员工 ID
  responseSpeed: 响应速度
  decisionStyle: 决策方式
  communicationStyle: 沟通风格
  riskPreference: 风险偏好
}
```

## 工作节点与工作区数据模型

### 工作节点（Worker Node）数据模型

```
WorkerNode {
  id: 唯一标识
  name: 节点名称
  type: 节点类型
  capabilities: 能力列表
  status: 节点状态
  lastSeen: 最后在线时间
  ownerId: 所有者 ID
  createdAt: 创建时间
  updatedAt: 更新时间
}
```

#### 节点配置数据
```
NodeConfig {
  nodeId: 所属节点 ID
  hardwareSpecs: 硬件规格
  softwareSpecs: 软件规格
  networkConfig: 网络配置
  securityConfig: 安全配置
}
```

#### 节点资源数据
```
NodeResources {
  nodeId: 所属节点 ID
  cpu: CPU 资源
  memory: 内存资源
  storage: 存储资源
  network: 网络资源
}
```

### 工作区（Workspace）数据模型

```
Workspace {
  id: 唯一标识
  name: 工作区名称
  type: 工作区类型
  nodeId: 所属节点 ID
  resources: 资源配置
  security: 安全配置
  lifecycle: 生命周期配置
  createdAt: 创建时间
  updatedAt: 更新时间
}
```

#### 工作区状态数据
```
WorkspaceState {
  workspaceId: 所属工作区 ID
  status: 工作区状态
  activeTasks: 活动任务
  context: 上下文数据
  lastActive: 最后活动时间
}
```

## 数据所有权与访问控制

### 数据所有权模型

```
DataOwnership {
  dataId: 数据 ID
  dataType: 数据类型
  ownerId: 所有者 ID
  ownershipType: 所有权类型
  transferable: 是否可转让
  createdAt: 创建时间
}
```

#### 所有权类型
- **个人所有**：数据完全归个人所有
- **公司所有**：数据归公司所有
- **团队共有**：数据归团队共有
- **公共数据**：数据公开可访问

### 访问控制模型

```
AccessControl {
  resourceId: 资源 ID
  resourceType: 资源类型
  subjectId: 主体 ID
  subjectType: 主体类型
  permissions: 权限列表
  grantedAt: 授予时间
  expiresAt: 过期时间
}
```

#### 权限类型
- **读取（Read）**：读取数据的权限
- **写入（Write）**：修改数据的权限
- **删除（Delete）**：删除数据的权限
- **管理（Admin）**：管理数据的权限
- **分享（Share）**：分享数据的权限

## 数据同步与冲突解决

### 数据同步模型

```
DataSync {
  syncId: 同步 ID
  dataId: 数据 ID
  sourceNodeId: 源节点 ID
  targetNodeId: 目标节点 ID
  syncStatus: 同步状态
  lastSyncAt: 最后同步时间
  syncDirection: 同步方向
}
```

#### 同步策略
- **实时同步**：数据变更立即同步
- **定期同步**：按固定时间间隔同步
- **按需同步**：用户手动触发同步
- **条件同步**：满足特定条件时同步

#### 同步方向
- **单向同步**：从源节点到目标节点
- **双向同步**：源节点和目标节点互相同步
- **多向同步**：多个节点之间同步

### 冲突解决模型

```
ConflictResolution {
  conflictId: 冲突 ID
  dataId: 数据 ID
  node1Id: 节点 1 ID
  node2Id: 节点 2 ID
  conflictType: 冲突类型
  resolutionStrategy: 解决策略
  resolvedAt: 解决时间
  resolvedBy: 解决者 ID
}
```

#### 冲突类型
- **版本冲突**：同一数据的不同版本
- **内容冲突**：数据内容不一致
- **元数据冲突**：元数据不一致
- **权限冲突**：权限设置冲突

#### 解决策略
- **最新优先**：使用最新的版本
- **用户选择**：由用户选择使用哪个版本
- **合并版本**：尝试合并不同版本
- **源优先**：使用源节点的版本
- **目标优先**：使用目标节点的版本

## 存储架构

### 分层存储架构

```
┌─────────────────────────────────────────────────┐
│              应用层（Application）              │
└────────────────────┬────────────────────────┘
                     │
┌────────────────────▼────────────────────────┐
│              服务层（Service）                 │
└────────────────────┬────────────────────────┘
                     │
┌────────────────────▼────────────────────────┐
│              抽象层（Abstraction）            │
│  ┌─────────────┐  ┌─────────────┐          │
│  │ Repository  │  │  Storage    │          │
│  │   Trait     │  │   Trait     │          │
│  └─────────────┘  └─────────────┘          │
└────────────────────┬────────────────────────┘
                     │
        ┌────────────┴────────────┐
        │                         │
┌───────▼────────┐      ┌────────▼─────────┐
│  实现层 1      │      │  实现层 2        │
│  (SQLite)     │      │  (PostgreSQL)    │
└────────────────┘      └──────────────────┘
```

### 存储抽象层

#### Repository Trait
```rust
trait Repository<T> {
    fn create(&self, entity: T) -> Result<T>;
    fn get(&self, id: &str) -> Result<Option<T>>;
    fn update(&self, entity: T) -> Result<T>;
    fn delete(&self, id: &str) -> Result<bool>;
    fn list(&self, query: Query) -> Result<Vec<T>>;
}
```

#### Storage Trait
```rust
trait Storage {
    fn upload(&self, path: &str, data: &[u8]) -> Result<()>;
    fn download(&self, path: &str) -> Result<Vec<u8>>;
    fn delete(&self, path: &str) -> Result<bool>;
    fn exists(&self, path: &str) -> Result<bool>;
    fn list(&self, prefix: &str) -> Result<Vec<String>>;
}
```

### 存储实现

#### 关系型数据库
- **SQLite**：轻量级，适合单机部署
- **PostgreSQL**：企业级，适合生产环境

#### 对象存储
- **文件系统（FS）**：本地文件系统，适合单机部署
- **S3**：对象存储服务，适合生产环境和云部署

## 数据加密与安全

### 数据加密模型

```
DataEncryption {
  dataId: 数据 ID
  encryptionType: 加密类型
  keyId: 密钥 ID
  encryptedAt: 加密时间
}
```

#### 加密类型
- **端到端加密**：只有发送方和接收方能解密
- **静态加密**：数据存储时加密
- **传输加密**：数据传输时加密

### 密钥管理

```
KeyManagement {
  keyId: 密钥 ID
  keyType: 密钥类型
  ownerId: 所有者 ID
  createdAt: 创建时间
  expiresAt: 过期时间
  rotationPolicy: 轮换策略
}
```

## 数据备份与恢复

### 备份策略

```
BackupPolicy {
  policyId: 策略 ID
  dataType: 数据类型
  backupFrequency: 备份频率
  retentionPeriod: 保留期限
  storageLocation: 存储位置
}
```

#### 备份频率
- **实时备份**：数据变更立即备份
- **每日备份**：每天备份一次
- **每周备份**：每周备份一次
- **每月备份**：每月备份一次

### 恢复流程

```
RecoveryProcess {
  recoveryId: 恢复 ID
  backupId: 备份 ID
  targetNodeId: 目标节点 ID
  recoveryStatus: 恢复状态
  startedAt: 开始时间
  completedAt: 完成时间
}
```

## 总结

完善的数据模型与存储设计是 AI Company 系统的基础，通过清晰的数据模型、灵活的访问控制、可靠的数据同步、以及安全的存储架构，确保系统的数据安全、可靠、高效。

理解数据模型与存储，有助于更好地设计、开发和维护 AI Company 系统，充分发挥系统的价值。
