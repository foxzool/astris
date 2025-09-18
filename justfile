# justfile — 常用开发脚本
# 安装 just: macOS `brew install just`，Linux 参见 https://github.com/casey/just

set shell := ["bash", "-eu", "-o", "pipefail", "-c"]
set dotenv-load := true
set export := true


# 默认目标：列出所有任务
default:
    @just --list


# 构建 / 发布构建
build:
    cargo build

build-release:
    cargo build --release


# 代码质量：格式化、检查、测试
fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

lint:
    cargo clippy --workspace --all-targets --all-features -- -Dwarnings

test:
    cargo test

# 一次性本地校验（CI 近似）：格式检查 + Lint + 测试
ci: fmt-check lint test

# 清理构建产物
clean:
    cargo clean
