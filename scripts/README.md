# 发布平台包（darwin-arm64）
bun scripts/publish-npm.mjs platform --npm-suffix darwin-arm64 --binary ./target/release/repomix --version 2.0.0

# 发布主包
bun scripts/publish-npm.mjs main --version 2.0.0
