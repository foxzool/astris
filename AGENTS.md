# Repository Guidelines

## 项目结构与架构总览
- 仓库采用 Rust workspace：根目录 `Cargo.toml` 聚合 `game` 与 `crates/*` 子 crate，并共享 `bevy` 等依赖。
- `game/` 提供最终可执行入口（`src/main.rs`），负责装配插件、载入资源与驱动主循环；编译产物位于 `target/`，保持忽略状态。
- `crates/astris_core` 汇集跨系统公用类型、资源与调度阶段；`crates/astris_character` 专注玩家与 NPC 行为；`crates/astris_level` 管理关卡数据、生成与加载流程。新增功能时优先扩展对应 crate，避免在 `game/` 中堆叠逻辑。
- 共享资产放在 `assets/`；如需测试资产，置于 `assets/tests/` 并在文档说明来源与用途。

## 构建、测试与开发命令
- `just build` / `just build-release`：分别执行 `cargo build` 与 `cargo build --release`，用于调试或性能/发布构建。
- `cargo run -p astris_game`：以默认 `dev_native` 特性运行入口程序，启用热重载；CI 或不支持监控时使用 `--no-default-features --features dev`。
- `just fmt`、`just fmt-check`、`just lint`、`just test`、`just ci`：对应格式化、只读校验、Clippy（`-D warnings`）、单元测试与一体化校验；提交前至少运行 `just ci`。

## 编码风格与命名约定
- 采用 Rust 2024 edition，`rustfmt.toml` 规定 Unix 换行与字段初始化速记；务必运行 `cargo fmt --all`。
- 模块/文件用 snake_case，类型 UpperCamelCase，常量 SCREAMING_SNAKE_CASE；Bevy 组件、资源与系统需以简短注释说明职责。
- 在跨 crate 共享 API 时，小心暴露面，优先通过功能模块的 `pub` re-export 提供入口，保持封装。

## 测试指引
- 统一使用 `cargo test`；每个 crate 在 `src/lib.rs` 末尾用 `#[cfg(test)] mod tests` 管理用例，可借助 `App::new()` 构建最小 ECS 场景。
- 关注纯逻辑单元（如组件更新、资源状态转换）与跨插件集成测试；命名遵循 `should_*` 格式凸显期望行为。
- 当前无覆盖率门槛，但 CI 会阻断失败测试；大型场景可在 `tests/` 建立集成测试 Crate，同步更新文档说明运行步骤。

## 提交与 Pull Request
- Commit 遵循 Conventional Commits，例如 `feat: add new 3D model`、`fix: handle missing texture`。
- PR 描述需包含动机、主要变更、关联 Issue（如有），视觉改动附截图或短视频，并确认本地通过 `cargo fmt --all`、`cargo clippy -D warnings`、`cargo test`。

## 配置与资产提示
- 默认启用 `dev_native` 特性，需要文件系统监控；在容器或 CI 环境可改用 `dev` 或禁用热重载相关特性。
- `.env`、密钥与大体积资产不得提交；如需共享本地配置，更新 README 或本文件说明加载顺序与示例。
- 引入第三方资产前确认许可，必要时提供下载地址及校验值，避免直接入库。
