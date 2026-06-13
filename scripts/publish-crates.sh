#!/usr/bin/env bash
# publish-crates.sh - 按依赖顺序发布 repomix workspace 中的 crates
#
# 发布顺序（从底层到顶层）：
#   1. repomix-shared    (无内部依赖)
#   2. repomix-config    (依赖 shared)
#   3. repomix-core      (依赖 shared, config)
#   4. repomix-mcp       (依赖 core, config, shared)
#   5. repomix-cli       (依赖所有)
#
# 用法：
#   ./scripts/publish-crates.sh              # 正常发布
#   ./scripts/publish-crates.sh --dry-run    # 仅预览，不实际发布
#   ./scripts/publish-crates.sh --skip <N>   # 跳过前 N 个 crate

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# 发布顺序（与依赖图一致）
CRATES=(
  "shared:repomix-shared"
  "config:repomix-config"
  "core:repomix-core"
  "mcp:repomix-mcp"
  "cli:repomix-cli"
)

# 需要添加 version 的内部依赖 crate 名称
INTERNAL_DEPS=(
  "repomix-shared"
  "repomix-config"
  "repomix-core"
  "repomix-mcp"
  "repomix-cli"
)

DRY_RUN=false
SKIP_COUNT=0

# 解析参数
while [[ $# -gt 0 ]]; do
  case "$1" in
    --dry-run)
      DRY_RUN=true
      shift
      ;;
    --skip)
      SKIP_COUNT="$2"
      shift 2
      ;;
    *)
      echo "Unknown option: $1"
      exit 1
      ;;
  esac
done

echo "========================================="
echo "  repomix crate 发布脚本"
echo "========================================="
echo ""

if $DRY_RUN; then
  echo "🔍 DRY-RUN 模式：仅预览，不实际发布"
  echo ""
fi

# 检查 cargo 是否可用
if ! command -v cargo &> /dev/null; then
  echo "❌ cargo 未找到，请先安装 Rust"
  exit 1
fi

# 检查 git 状态
if [[ -d "$PROJECT_ROOT/.git" ]]; then
  GIT_STATUS=$(cd "$PROJECT_ROOT" && git status --porcelain 2>/dev/null || true)
  if [[ -n "$GIT_STATUS" ]]; then
    echo "⚠️  警告：工作区有未提交的更改"
    echo ""
  fi
fi

# 收集版本信息用于预览
VERSIONS=()
for entry in "${CRATES[@]}"; do
  IFS=':' read -r dir name <<< "$entry"
  cargo_toml="$PROJECT_ROOT/crates/$dir/Cargo.toml"
  version=$(grep '^version' "$cargo_toml" | head -1 | sed 's/.*"\(.*\)".*/\1/')
  VERSIONS+=("$version")
done

# 检查 crates.io 上是否已存在该版本
is_published() {
  local name="$1"
  local version="$2"
  cargo search "$name" 2>/dev/null | grep -q "^$name = \"$version\""
}

# 预览发布计划
echo "📋 发布计划："
echo "-----------------------------------------"
for i in "${!CRATES[@]}"; do
  IFS=':' read -r dir name <<< "${CRATES[$i]}"
  version="${VERSIONS[$i]}"
  if [[ $i -lt $SKIP_COUNT ]]; then
    echo "  [$((i+1))] $name v$version  ⏭️  (跳过)"
  elif is_published "$name" "$version"; then
    echo "  [$((i+1))] $name v$version  ✅ (已发布)"
  else
    echo "  [$((i+1))] $name v$version"
  fi
done
echo "-----------------------------------------"
echo ""

if $DRY_RUN; then
  echo "💡 实际运行命令预览："
  for i in "${!CRATES[@]}"; do
    if [[ $i -lt $SKIP_COUNT ]]; then
      continue
    fi
    IFS=':' read -r dir name <<< "${CRATES[$i]}"
    echo "  cargo publish -p $name --allow-dirty"
  done
  echo ""
  echo "✅ DRY-RUN 完成（未实际发布）"
  exit 0
fi

# ========== 辅助函数 ==========

# 给单个 Cargo.toml 的内部依赖添加 version 字段
add_versions_to_toml() {
  local toml_file="$1"
  local version="$2"

  for dep in "${INTERNAL_DEPS[@]}"; do
    # 仅匹配该行本身，不向后延伸（避免匹配到 serde 等其他依赖的 version 字段）
    local dep_line
    dep_line=$(grep "^$dep = " "$toml_file" 2>/dev/null || true)

    if [[ -n "$dep_line" ]]; then
      if echo "$dep_line" | grep -q 'version'; then
        continue
      fi
      sed -i.bak "s|^$dep = { path = |$dep = { version = \"$version\", path = |g" "$toml_file"
    fi
  done
}

# 备份所有 Cargo.toml
backup_tomls() {
  echo "📦 备份 Cargo.toml..."
  for entry in "${CRATES[@]}"; do
    IFS=':' read -r dir name <<< "$entry"
    cp "$PROJECT_ROOT/crates/$dir/Cargo.toml" "$PROJECT_ROOT/crates/$dir/Cargo.toml.bak"
  done
}

# 恢复所有 Cargo.toml
restore_tomls() {
  echo "🔄 恢复 Cargo.toml..."
  for entry in "${CRATES[@]}"; do
    IFS=':' read -r dir name <<< "$entry"
    if [[ -f "$PROJECT_ROOT/crates/$dir/Cargo.toml.bak" ]]; then
      mv "$PROJECT_ROOT/crates/$dir/Cargo.toml.bak" "$PROJECT_ROOT/crates/$dir/Cargo.toml"
    fi
  done
}

# 清理备份文件
cleanup_backups() {
  for entry in "${CRATES[@]}"; do
    IFS=':' read -r dir name <<< "$entry"
    rm -f "$PROJECT_ROOT/crates/$dir/Cargo.toml.bak"
  done
}

# 检查版本一致性并获取 shared 的版本
get_shared_version() {
  local version
  version=$(grep '^version' "$PROJECT_ROOT/crates/shared/Cargo.toml" | head -1 | sed 's/.*"\(.*\)".*/\1/')
  echo "$version"
}

# ========== 主流程 ==========

# 设置 trap 以在失败时恢复
trap 'restore_tomls; echo "❌ 发布失败，已恢复 Cargo.toml"; exit 1' ERR

# 备份
backup_tomls

# 获取版本号（使用 shared 的版本作为所有内部依赖的版本）
SHARED_VERSION=$(get_shared_version)
echo "🔧 自动为内部依赖添加 version = \"$SHARED_VERSION\" ..."

# 修改所有非 shared 的 Cargo.toml
for entry in "${CRATES[@]}"; do
  IFS=':' read -r dir name <<< "$entry"
  if [[ "$name" != "repomix-shared" ]]; then
    add_versions_to_toml "$PROJECT_ROOT/crates/$dir/Cargo.toml" "$SHARED_VERSION"
  fi
done

echo ""

# 实际发布
PUBLISHED=()

for i in "${!CRATES[@]}"; do
  IFS=':' read -r dir name <<< "${CRATES[$i]}"
  version="${VERSIONS[$i]}"

  if [[ $i -lt $SKIP_COUNT ]]; then
    echo "⏭️  跳过 $name v$version"
    continue
  fi

  # 自动跳过已发布的版本
  if is_published "$name" "$version"; then
    echo "⏭️  $name v$version 已存在于 crates.io，跳过"
    PUBLISHED+=("$name")
    continue
  fi

  echo "📦 发布 $name v$version ..."

  if cargo publish -p "$name" --allow-dirty 2>&1; then
    echo "✅ $name v$version 发布成功"
    PUBLISHED+=("$name")
  else
    echo "❌ $name v$version 发布失败"
    restore_tomls
    exit 1
  fi

  # 等待 crates.io 索引更新（避免后续 crate 找不到刚发布的依赖）
  if [[ $i -lt $((${#CRATES[@]}-1)) ]]; then
    echo "⏳ 等待 crates.io 索引更新..."
    sleep 10
  fi

  echo ""
done

# 恢复原始 Cargo.toml
restore_tomls

# 清理备份（正常退出时）
cleanup_backups

echo "========================================="
echo "🎉 全部发布完成！"
echo "========================================="
echo ""
echo "已发布："
for name in "${PUBLISHED[@]}"; do
  echo "  ✅ $name"
done