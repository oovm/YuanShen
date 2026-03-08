# ys-cli & ys-protocol 核心功能完善 - Product Requirement Document

## Overview
- **Summary**: 完善 ys-cli 命令行工具的核心功能（达到 60% 完成度），同时完善 ys-protocol 协议库（达到 20% 完成度），使其成为一个可用的现代化版本管理系统。
- **Purpose**: 解决当前 ys-cli 只有部分功能可用的问题，提供基本的版本控制功能；同时完善 ys-protocol 的 Git 协议支持，为后续网络功能打下基础。
- **Target Users**: 需要使用简单、高效版本管理系统的开发者和团队。

## Goals
- ys-cli 达到 60% 核心功能可用：完善现有命令并实现 Squash、Merge、Reset 功能
- ys-protocol 达到 20% 完成度：完善 Git 协议支持，实现基本的 pkt-line 协议和 Git 协议解析
- 为所有公共结构体、枚举、方法、字段添加文档注释
- 修复现有代码中的问题，提高代码质量

## Non-Goals (Out of Scope)
- 实现 Rebase、Orphan、Stash、Garbage Collect 等高级功能
- 实现完整的 Git 服务器功能
- 实现与其他 VCS 系统的互操作性（SVN、P4 等）
- 图形用户界面（GUI）
- 插件系统
- 多租户支持

## Background & Context
- ys-cli 是一个现代化的异步版本管理工具，目前已有 init、commit、diff、branch、checkout 等基本功能，但 Squash、Merge、Rebase、Reset、Orphan 等功能尚未实现
- ys-protocol 目前只实现了 Git pkt-line 协议的基础编解码功能，需要进一步完善
- 项目使用 Rust 编写，基于异步 tokio 运行时
- 已有 ys-types 和 ys-storage 作为底层依赖库

## Functional Requirements
- **FR-1**: ys-cli 的 Squash 命令可以将多个提交合并为一个
- **FR-2**: ys-cli 的 Merge 命令可以合并两个分支
- **FR-3**: ys-cli 的 Reset 命令可以将当前分支重置到指定提交
- **FR-4**: ys-cli 的 Branch 命令可以列出所有分支并显示当前分支
- **FR-5**: ys-protocol 提供完整的 Git pkt-line 协议支持
- **FR-6**: ys-protocol 实现 Git 协议的基本握手和数据传输
- **FR-7**: 所有公共 API 都有完整的文档注释
- **FR-8**: 现有命令的错误处理得到改进

## Non-Functional Requirements
- **NFR-1**: 所有命令执行时间应在合理范围内（简单操作 &lt; 1 秒）
- **NFR-2**: 代码应遵循 Rust 最佳实践和项目现有风格
- **NFR-3**: 所有公共 API 必须有完整的文档注释
- **NFR-4**: 错误信息应清晰、有用

## Constraints
- **Technical**: 必须使用 Rust，基于现有代码库，使用 tokio 异步运行时
- **Business**: 优先保证核心功能的稳定性和可靠性
- **Dependencies**: ys-types, ys-storage, clap, tokio, serde

## Assumptions
- ys-types 和 ys-storage 提供的基础功能是稳定的
- 项目现有的 Commit、SnapshotTree 等数据结构能够满足新功能的需求
- 用户已经了解基本的版本控制概念

## Acceptance Criteria

### AC-1: Squash 命令可用
- **Given**: 有多个连续的提交
- **When**: 执行 squash 命令合并这些提交
- **Then**: 多个提交被合并为一个，新提交包含所有变更，分支指向新提交
- **Verification**: `programmatic`
- **Notes**: 需要考虑提交信息的合并

### AC-2: Merge 命令可用
- **Given**: 有两个分支，各自有独立的提交
- **When**: 执行 merge 命令合并另一个分支到当前分支
- **Then**: 创建一个合并提交，包含两个分支的变更
- **Verification**: `programmatic`
- **Notes**: 暂不处理冲突，直接提示用户

### AC-3: Reset 命令可用
- **Given**: 当前分支有多个提交
- **When**: 执行 reset 命令重置到指定提交
- **Then**: 分支指针移动到指定提交
- **Verification**: `programmatic`
- **Notes**: 支持 --hard 模式，重置工作目录

### AC-4: Branch 命令完善
- **Given**: 仓库中有多个分支
- **When**: 执行 branch 命令
- **Then**: 列出所有分支，当前分支用特殊标识标记
- **Verification**: `programmatic`

### AC-5: ys-protocol 完善
- **Given**: 需要与 Git 协议进行交互
- **When**: 使用 ys-protocol 的 API
- **Then**: 可以正确编解码 Git pkt-line 协议数据
- **Verification**: `programmatic`

### AC-6: 文档注释完善
- **Given**: 代码库中的公共 API
- **When**: 查看代码或生成文档
- **Then**: 所有公共结构体、枚举、方法、字段都有文档注释
- **Verification**: `human-judgment`

### AC-7: 现有功能稳定
- **Given**: 已有的 init、commit、diff、checkout 命令
- **When**: 执行这些命令
- **Then**: 它们能正常工作，没有回归
- **Verification**: `programmatic`

## Open Questions
- [ ] Merge 命令是否需要处理冲突？当前计划是暂不处理，直接提示用户
- [ ] Reset 命令是否需要支持 --soft 和 --mixed 模式？
- [ ] ys-protocol 需要支持哪些 Git 协议具体功能？
