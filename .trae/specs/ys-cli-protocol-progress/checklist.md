# ys-cli &amp; ys-protocol 核心功能完善 - Verification Checklist

## ys-cli 功能验证
- [ ] Branch 命令能正确列出所有分支
- [ ] Branch 命令正确标记当前分支
- [ ] Squash 命令能合并多个提交
- [ ] Squash 命令后分支指针正确
- [ ] Merge 命令能创建合并提交
- [ ] Merge 命令处理 fast-forward 情况
- [ ] Reset 命令能移动分支指针
- [ ] Reset --hard 能重置工作目录
- [ ] 所有公共 API 有文档注释
- [ ] 现有命令（init、commit、diff、checkout）工作正常

## ys-protocol 功能验证
- [ ] PktLineCodec 能正确编码 Data 类型
- [ ] PktLineCodec 能正确编码 Flush 类型
- [ ] PktLineCodec 能正确编码 Delim 类型
- [ ] PktLineCodec 能正确编码 ResponseEnd 类型
- [ ] PktLineCodec 能正确解码所有类型
- [ ] 所有公共 API 有文档注释
- [ ] 单元测试覆盖编解码功能

## 代码质量验证
- [ ] 没有使用不必要的 unwrap()
- [ ] 错误处理完善且友好
- [ ] 代码符合 Rust 风格
- [ ] cargo clippy 没有警告
- [ ] cargo build 成功
- [ ] 项目可以编译通过
