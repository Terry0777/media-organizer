# Media Organizer - 快速启动指南

**版本：** v1.0 (Pure Rust)  
**创建日期：** 2026-03-04

---

## 1. 环境要求

- **操作系统：** Windows 10+ / macOS 11+ / Linux (Ubuntu 20.04+)
- **Rust：** 1.75+ (必须)
- **Node.js：** 18+ (前端开发)
- **内存：** 最低 4GB，推荐 8GB+
- **磁盘：** 至少 500MB 可用空间（不含媒体文件）

### 1.1 安装 Rust

```bash
# Linux/macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Windows (PowerShell)
winget install Rustlang.Rustup
# 或下载安装程序：https://rustup.rs
```

### 1.2 验证安装

```bash
rustc --version    # 应显示 1.75+
cargo --version
node --version     # 应显示 18+
npm --version
```

---

## 2. 项目结构

```
media-organizer/
├── docs/                    # 文档
│   ├── PRD.md              # 产品需求文档
│   ├── ARCHITECTURE.md     # 技术架构文档
│   ├── DIAGRAMS.md         # 架构图集
│   └── QUICKSTART.md       # 本文件
├── src/                     # 前端代码 (React + TypeScript)
│   ├── components/         # UI 组件
│   ├── hooks/              # React Hooks
│   ├── stores/             # 状态管理
│   └── App.tsx
├── src-tauri/               # Rust 后端代码
│   ├── src/
│   │   ├── main.rs         # Tauri 入口
│   │   ├── commands/       # Tauri Commands
│   │   ├── services/       # 业务逻辑
│   │   └── models/         # 数据模型
│   ├── Cargo.toml          # Rust 依赖
│   └── tauri.conf.json     # Tauri 配置
├── package.json             # Node 依赖
└── tsconfig.json            # TypeScript 配置
```

---

## 3. 快速开始

### 3.1 初始化项目

```bash
cd /home/admin/.openclaw/workspace/projects/media-organizer

# 安装前端依赖
npm install

# 验证 Rust 依赖（首次会下载编译）
cd src-tauri
cargo check
```

### 3.2 启动开发环境

```bash
# 返回项目根目录
cd ..

# 启动开发模式（自动热重载）
npm run tauri dev
```

**说明：**
- 首次启动会编译 Rust 代码，可能需要 2-5 分钟
- 后续启动只需几秒钟
- 前端和 Rust 代码修改都会自动重载

### 3.3 构建生产版本

```bash
# 构建桌面应用
npm run tauri build

# 输出位置
src-tauri/target/release/bundle/
├── msi/          # Windows 安装包
├── dmg/          # macOS 安装包
└── appimage/     # Linux 安装包
```

---

## 4. 核心依赖说明

### 4.1 Rust Crate (src-tauri/Cargo.toml)

```toml
[package]
name = "media-organizer"
version = "0.1.0"
edition = "2021"

[dependencies]
# Tauri 核心
tauri = { version = "2.0", features = [] }
tauri-plugin-shell = "2.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# 数据库
rusqlite = { version = "0.31", features = ["bundled"] }

# 图像处理
image = "0.25"
kamadak-exif = "0.5"

# 文件监控
notify = "6.1"

# 异步运行时
tokio = { version = "1.0", features = ["full"] }

# 工具
chrono = "0.4"
sha2 = "0.10"
walkdir = "2.0"
uuid = { version = "1.0", features = ["v4"] }

# 日志
log = "0.4"
env_logger = "0.11"
```

### 4.2 Node 依赖 (package.json)

```json
{
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "@tanstack/react-virtual": "^3.0.0",
    "zustand": "^4.5.0"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^2.0.0",
    "@tauri-apps/api": "^2.0.0",
    "typescript": "^5.0.0",
    "vite": "^5.0.0"
  }
}
```

---

## 5. 开发指南

### 5.1 添加新的 Tauri Command

```rust
// src-tauri/src/commands/file.rs

use tauri::State;
use crate::services::file_scanner::FileScanner;
use crate::AppState;

#[tauri::command]
pub async fn get_files(
    page: usize,
    page_size: usize,
    state: State<'_, AppState>,
) -> Result<Vec<MediaFile>, String> {
    let scanner = state.scanner.lock().map_err(|e| e.to_string())?;
    scanner.get_files(page, page_size)
        .map_err(|e| e.to_string())
}
```

**前端调用：**

```typescript
// src/components/Gallery/index.tsx
import { invoke } from '@tauri-apps/api/core';

const files = await invoke<MediaFile[]>('get_files', {
  page: 1,
  pageSize: 50
});
```

### 5.2 添加新的文件格式支持

```rust
// src-tauri/src/services/file_scanner.rs

impl FileScanner {
    fn load_supported_extensions() -> HashSet<String> {
        let mut exts = HashSet::new();
        
        // 图片格式
        exts.insert(".jpg".to_string());
        exts.insert(".jpeg".to_string());
        exts.insert(".png".to_string());
        exts.insert(".gif".to_string());
        exts.insert(".webp".to_string());
        exts.insert(".heic".to_string());
        exts.insert(".raw".to_string());
        
        // 视频格式
        exts.insert(".mp4".to_string());
        exts.insert(".mov".to_string());
        exts.insert(".avi".to_string());
        exts.insert(".mkv".to_string());
        exts.insert(".webm".to_string());
        
        exts
    }
}
```

### 5.3 数据库迁移

```rust
// src-tauri/src/db/migrations.rs

pub fn run_migrations(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS media_files (
            id INTEGER PRIMARY KEY,
            file_path TEXT UNIQUE NOT NULL,
            file_type TEXT NOT NULL,
            -- ... 其他字段
        );
        
        CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY,
            name TEXT UNIQUE NOT NULL,
            parent_id INTEGER REFERENCES tags(id),
            color TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
        
        CREATE TABLE IF NOT EXISTS media_tags (
            media_id INTEGER REFERENCES media_files(id),
            tag_id INTEGER REFERENCES tags(id),
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            PRIMARY KEY (media_id, tag_id)
        );
        "
    )?;
    Ok(())
}
```

---

## 6. 常见问题

### Q1: Rust 编译失败？

```bash
# 更新 Rust 到最新版本
rustup update

# 清理构建缓存
cargo clean

# 重新编译
cargo build
```

### Q2: Tauri 开发模式启动慢？

- 首次启动需要编译 Rust 代码，正常需要 2-5 分钟
- 后续启动会复用编译结果，只需几秒钟
- 可以安装 `sccache` 加速编译：
  ```bash
  cargo install sccache
  export RUSTC_WRAPPER=sccache
  ```

### Q3: 图片格式不支持？

- 检查 `image` crate 版本是否支持该格式
- HEIC 格式需要额外依赖：
  ```toml
  libheif-rs = "1.0"
  ```

### Q4: 视频时长无法获取？

- 安装系统级 `ffprobe`：
  ```bash
  # Ubuntu/Debian
  sudo apt install ffmpeg
  
  # macOS
  brew install ffmpeg
  
  # Windows
  winget install ffmpeg
  ```

### Q5: 扫描速度慢？

- 调整并发数量（默认 4 线程）
- 关闭增量扫描进行首次全量扫描
- 排除大目录（如系统文件夹）

---

## 7. 开发任务清单

### Phase 1 - 基础框架 (Week 1-2)
- [ ] 初始化 Tauri 项目
- [ ] 实现文件扫描器（Rust）
- [ ] 实现数据库模型（SQLite）
- [ ] 实现基础 Tauri Commands
- [ ] 前端基础界面（网格视图）

### Phase 2 - 标签系统 (Week 3-4)
- [ ] 实现标签管理（Rust）
- [ ] 实现批量打标签
- [ ] 前端标签浏览器
- [ ] 标签筛选功能

### Phase 3 - 浏览体验 (Week 5-6)
- [ ] 时间线视图
- [ ] 虚拟滚动（大数据量优化）
- [ ] 缩略图缓存
- [ ] 幻灯片视图

### Phase 4 - 优化与测试 (Week 7-8)
- [ ] 增量扫描（文件监控）
- [ ] 性能优化
- [ ] UI/UX 打磨
- [ ] 打包发布

---

## 8. 调试技巧

### 8.1 查看 Rust 日志

```bash
# 设置日志级别
RUST_LOG=debug npm run tauri dev

# 日志输出位置
# Linux: ~/.local/share/media-organizer/logs/
# macOS: ~/Library/Application Support/media-organizer/logs/
# Windows: %APPDATA%/media-organizer/logs/
```

### 8.2 前端调试

```bash
# 开发模式下按 F12 打开开发者工具
# 可以查看 Console、Network、Performance
```

### 8.3 数据库检查

```bash
# 使用 SQLite 命令行工具
sqlite3 ~/.local/share/media-organizer/media_organizer.db

# 查看表
.tables

# 查询媒体文件
SELECT * FROM media_files LIMIT 10;
```

---

## 9. 下一步

1. ✅ **阅读 PRD.md** - 了解产品全貌
2. ✅ **阅读 ARCHITECTURE.md** - 理解技术设计
3. ✅ **阅读 DIAGRAMS.md** - 查看架构图
4. 🚀 **开始开发 MVP** - 从文件扫描模块入手

---

**有问题随时找我，少爷！🦐**  
*祝开发顺利！*
