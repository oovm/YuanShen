# ys-cli &amp; ys-protocol 核心功能完善 - The Implementation Plan (Decomposed and Prioritized Task List)

## [x] Task 1: 完善 ys-cli 的 Branch 命令
- **Priority**: P0
- **Depends On**: None
- **Description**: 
  - 扩展 Branch 命令，使其能够列出所有分支
  - 当前分支用特殊标识（如 *）标记
  - 为 Branch 命令的公共 API 添加文档注释
- **Acceptance Criteria Addressed**: [AC-4, AC-6, AC-7]
- **Test Requirements**:
  - `programmatic` TR-1.1: 创建多个分支后，执行 branch 命令能正确列出所有分支
  - `programmatic` TR-1.2: 当前分支前面有 * 标记
  - `human-judgement` TR-1.3: 所有公共 API 都有完整的文档注释
- **Notes**: 现有 Branch 命令只显示当前分支名称

## [x] Task 2: 实现 ys-cli 的 Squash 命令
- **Priority**: P0
- **Depends On**: None
- **Description**: 
  - 实现 Squash 命令，可以将多个提交合并为一个
  - 合并提交的信息可以自定义或自动合并
  - 为 Squash 命令的公共 API 添加文档注释
- **Acceptance Criteria Addressed**: [AC-1, AC-6, AC-7]
- **Test Requirements**:
  - `programmatic` TR-2.1: 创建 3 个连续提交，squash 后变成 1 个提交
  - `programmatic` TR-2.2: 新提交包含所有变更
  - `programmatic` TR-2.3: 分支指针正确指向新提交
  - `human-judgement` TR-2.4: 所有公共 API 都有完整的文档注释
- **Notes**: 从当前分支的最新提交向前合并指定数量的提交

## [x] Task 3: 实现 ys-cli 的 Merge 命令
- **Priority**: P0
- **Depends On**: None
- **Description**: 
  - 实现 Merge 命令，合并另一个分支到当前分支
  - 创建一个合并提交，包含两个父提交
  - 暂不处理冲突，检测到冲突时提示用户
  - 为 Merge 命令的公共 API 添加文档注释
- **Acceptance Criteria Addressed**: [AC-2, AC-6, AC-7]
- **Test Requirements**:
  - `programmatic` TR-3.1: 创建两个分支，各自有独立提交，merge 后创建合并提交
  - `programmatic` TR-3.2: 合并提交有两个父提交
  - `human-judgement` TR-3.3: 所有公共 API 都有完整的文档注释
- **Notes**: 使用 fast-forward 策略当可能时

## [x] Task 4: 实现 ys-cli 的 Reset 命令
- **Priority**: P0
- **Depends On**: None
- **Description**: 
  - 实现 Reset 命令，将当前分支重置到指定提交
  - 支持 --hard 模式，重置工作目录
  - 为 Reset 命令的公共 API 添加文档注释
- **Acceptance Criteria Addressed**: [AC-3, AC-6, AC-7]
- **Test Requirements**:
  - `programmatic` TR-4.1: 执行 reset 命令后，分支指针移动到指定提交
  - `programmatic` TR-4.2: --hard 模式下，工作目录被重置
  - `human-judgement` TR-4.3: 所有公共 API 都有完整的文档注释
- **Notes**: 支持通过提交哈希或分支名指定目标

## [x] Task 5: 完善 ys-protocol 的 Git 协议支持
- **Priority**: P1
- **Depends On**: None
- **Description**: 
  - 完善现有的 pkt_line 模块，添加文档注释
  - 实现 Git 协议的基本握手流程
  - 为所有公共 API 添加文档注释
- **Acceptance Criteria Addressed**: [AC-5, AC-6]
- **Test Requirements**:
  - `programmatic` TR-5.1: PktLineCodec 能正确编解码所有 pkt-line 类型
  - `programmatic` TR-5.2: 添加单元测试覆盖编解码功能
  - `human-judgement` TR-5.3: 所有公共 API 都有完整的文档注释
- **Notes**: 参考 Git 协议文档

## [x] Task 6: 为现有 ys-cli 命令添加文档注释
- **Priority**: P1
- **Depends On**: None
- **Description**: 
  - 为 init、commit、diff、checkout 命令的公共 API 添加文档注释
  - 修复现有代码中的小问题（如 unwrap 使用）
  - 改进错误处理
- **Acceptance Criteria Addressed**: [AC-6, AC-7]
- **Test Requirements**:
  - `human-judgement` TR-6.1: 所有公共 API 都有完整的文档注释
  - `programmatic` TR-6.2: 现有命令能正常工作，没有回归
- **Notes**: 遵循项目现有的代码风格

## [ ] Task 7: 为 ys-protocol 现有代码添加文档注释
- **Priority**: P1
- **Depends On**: Task 5
- **Description**: 
  - 为 ys-protocol 所有公共 API 添加文档注释
  - 确保代码符合 Rust 文档规范
- **Acceptance Criteria Addressed**: [AC-6]
- **Test Requirements**:
  - `human-judgement` TR-7.1: 所有公共 API 都有完整的文档注释
- **Notes**: 使用 cargo doc 可以生成正确的文档
