# 源神 (YuanShen) 项目完善 - The Implementation Plan (Decomposed and Prioritized Task List)

## [x] Task 1: 修复 ys-storage 测试构建问题
- **Priority**: P0
- **Depends On**: None
- **Description**: 
  - 修复 limbo_fs 测试中的导入问题
  - 为测试配置正确的 feature flag 或调整测试代码
- **Acceptance Criteria Addressed**: [AC-1, AC-2]
- **Test Requirements**:
  - `programmatic` TR-1.1: `cargo test -p ys-storage` 成功编译并运行
- **Notes**: 问题在于 LimboFsStorage 被 `#[cfg(feature = "limbo")]` 条件编译，但测试未启用该 feature

## [x] Task 2: 启用 ys-types 的核心测试
- **Priority**: P0
- **Depends On**: None
- **Description**: 
  - 检查并启用 ys-types 中被注释的测试用例
  - 确保这些测试能够正常运行
- **Acceptance Criteria Addressed**: [AC-2]
- **Test Requirements**:
  - `programmatic` TR-2.1: `cargo test -p ys-types` 所有测试通过
- **Notes**: 目前 ys-types/tests/main.rs 中大部分测试被注释掉了

## [x] Task 3: 验证并完善代码格式
- **Priority**: P1
- **Depends On**: None
- **Description**: 
  - 运行 rustfmt 确保所有代码符合格式规范
  - 检查是否有格式问题需要修复
- **Acceptance Criteria Addressed**: [AC-4]
- **Test Requirements**:
  - `programmatic` TR-3.1: `cargo fmt --all --check` 无错误
- **Notes**: 保持项目现有的代码风格

## [x] Task 4: 验证基本 CLI 命令功能
- **Priority**: P1
- **Depends On**: [Task 1, Task 2]
- **Description**: 
  - 编译 ys-tools 二进制
  - 在临时目录中测试 init、commit、diff 等基本命令
- **Acceptance Criteria Addressed**: [AC-3]
- **Test Requirements**:
  - `programmatic` TR-4.1: ys-tools 能够成功编译
  - `programmatic` TR-4.2: init 命令能够创建仓库
  - `programmatic` TR-4.3: commit 命令能够成功提交更改
  - `programmatic` TR-4.4: diff 命令能够正确显示差异

## [x] Task 5: 清理未使用的代码和警告
- **Priority**: P2
- **Depends On**: None
- **Description**: 
  - 修复或清理测试中的未使用常量警告
  - 清理其他明显的未使用代码
- **Acceptance Criteria Addressed**: [AC-1]
- **Test Requirements**:
  - `human-judgement` TR-5.1: 代码审查确认无明显的未使用代码
- **Notes**: 保持代码整洁，但不要过度重构
