# Media Organizer - 技术架构文档

**版本：** v1.0 (Pure Rust)  
**创建日期：** 2026-03-04  
**作者：** 虾米 🦐

---

## 1. 系统概览

```
┌─────────────────────────────────────────────────────────────────────────┐
│                            CLIENT LAYER                                  │
│  ┌─────────────────────────────────────────────────────────────────────┐│
│  │  Tauri v2 App (React + TypeScript + Rust)                           ││
│  │  ┌───────────────────────────────────────────────────────────────┐  ││
│  │  │  Frontend (Webview)                                            │  ││
│  │  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────┐ │  ││
│  │  │  │  Gallery     │  │  Timeline    │  │  Settings & Config   │ │  ││
│  │  │  │  Component   │  │  Component   │  │  Component           │ │  ││
│  │  │  └──────────────┘  └──────────────┘  └──────────────────────┘ │  ││
│  │  └───────────────────────────────────────────────────────────────┘  ││
│  └─────────────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────────────┘
                                    │
                                    │ Tauri Commands (IPC)
                                    ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                            RUST LAYER                                    │
│  ┌─────────────────────────────────────────────────────────────────────┐│
│  │  Tauri Commands (src-tauri/src/commands/)                           ││
│  │  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌───────────────┐ ││
│  │  │ file::      │ │ tag::       │ │ album::     │ │ search::      │ ││
│  │  │ scan_dir    │ │ create      │ │ create      │ │ query         │ ││
│  │  │ get_files   │ │ add/remove  │ │ add_media   │ │ filter        │ ││
│  │  │ get_meta    │ │ list        │ │ delete      │ │ suggest       │ ││
│  │  └─────────────┘ └─────────────┘ └─────────────┘ └───────────────┘ ││
│  └─────────────────────────────────────────────────────────────────────┘│
│  ┌─────────────────────────────────────────────────────────────────────┐│
│  │  Services (src-tauri/src/services/)                                 ││
│  │  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌───────────────┐ ││
│  │  │ File        │ │ Tag         │ │ Thumbnail   │ │ Search        │ ││
│  │  │ Scanner     │ │ Manager     │ │ Generator   │ │ Engine        │ ││
│  │  └─────────────┘ └─────────────┘ └─────────────┘ └───────────────┘ ││
│  └─────────────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────────────┘
                                    │
                    ┌───────────────┼───────────────┐
                    ▼               ▼               ▼
           ┌─────────────┐ ┌─────────────┐ ┌─────────────┐
           │  DATA LAYER │ │  IMAGE LIB  │ │  FS LAYER   │
           │  SQLite     │ │  image crate│ │  notify     │
           │  rusqlite   │ │  imageproc  │ │  std::fs    │
           └─────────────┘ └─────────────┘ └─────────────┘
```

**核心特点：**
- ✅ **无外部后端** - 所有逻辑在 Rust 层实现
- ✅ **无需联网** - 纯本地运行，隐私安全
- ✅ **单二进制** - Tauri 打包后只有一个可执行文件
- ✅ **高性能** - Rust 处理文件 I/O 和图像处理

---

## 2. 模块详细设计

### 2.1 文件扫描模块 (File Scanner)

```rust
// src-tauri/src/services/file_scanner.rs

use rusqlite::Connection;
use image::DynamicImage;
use notify::{Watcher, RecursiveMode};

pub struct FileScanner {
    db: Connection,
    supported_extensions: HashSet<String>,
}

impl FileScanner {
    pub fn new(db_path: &str) -> Result<Self> {
        let db = Connection::open(db_path)?;
        let supported_extensions = Self::load_supported_extensions();
        Ok(Self { db, supported_extensions })
    }

    /// 扫描目录
    pub fn scan_directory(&mut self, path: &str, incremental: bool) -> Result<ScanResult> {
        // 1. 遍历目录（递归）
        // 2. 过滤支持的文件格式
        // 3. 计算文件 checksum（去重）
        // 4. 提取元数据（EXIF, 视频信息）
        // 5. 生成缩略图
        // 6. 更新数据库
        // 7. 返回扫描统计
    }

    /// 提取文件元数据
    pub fn extract_metadata(&self, file_path: &str) -> Result<MediaMetadata> {
        // 使用 kamadak-exif 读取 EXIF
        // 使用 image  crate 读取分辨率
        // 使用 ffprobe (可选) 读取视频时长
    }

    /// 生成缩略图
    pub fn generate_thumbnail(&self, file_path: &str, size: u32) -> Result<String> {
        // 加载图片/视频帧
        // 缩放至指定大小
        // 保存到缓存目录
        // 返回缩略图路径
    }
}
```

**工作流程：**
```
1. 遍历目录（递归）
2. 过滤支持的文件格式
3. 计算文件 checksum（去重）
4. 提取元数据（EXIF, 视频信息）
5. 生成缩略图
6. 更新数据库
7. 返回扫描统计
```

### 2.2 标签管理模块 (Tag Manager)

```rust
// src-tauri/src/services/tag_manager.rs

pub struct TagManager {
    db: Connection,
}

impl TagManager {
    pub fn new(db: Connection) -> Self {
        Self { db }
    }

    /// 创建标签
    pub fn create_tag(&self, name: &str, parent_id: Option<i64>, color: Option<String>) -> Result<i64> {
        // INSERT INTO tags (name, parent_id, color) VALUES (?, ?, ?)
    }

    /// 为媒体添加标签
    pub fn add_tag_to_media(&self, media_id: i64, tag_id: i64) -> Result<()> {
        // INSERT OR IGNORE INTO media_tags (media_id, tag_id) VALUES (?, ?)
    }

    /// 从媒体移除标签
    pub fn remove_tag_from_media(&self, media_id: i64, tag_id: i64) -> Result<()> {
        // DELETE FROM media_tags WHERE media_id = ? AND tag_id = ?
    }

    /// 获取标签云（按使用频率排序）
    pub fn get_tag_cloud(&self, limit: usize) -> Result<Vec<TagUsage>> {
        // SELECT t.*, COUNT(mt.media_id) as usage_count 
        // FROM tags t LEFT JOIN media_tags mt ON t.id = mt.tag_id
        // GROUP BY t.id ORDER BY usage_count DESC LIMIT ?
    }

    /// 批量打标签
    pub fn batch_add_tags(&self, media_ids: &[i64], tag_ids: &[i64]) -> Result<usize> {
        // 事务批量插入
    }
}
```

### 2.3 搜索模块 (Search Engine)

```rust
// src-tauri/src/services/search_engine.rs

pub struct SearchFilters {
    pub file_type: Option<FileType>,      // image/video
    pub tags: Option<Vec<i64>>,           // 标签 ID 列表
    pub date_from: Option<i64>,           // Unix 时间戳
    pub date_to: Option<i64>,
    pub size_min: Option<i64>,
    pub size_max: Option<i64>,
    pub folder: Option<String>,           // 文件夹路径前缀
}

pub struct SearchEngine {
    db: Connection,
}

impl SearchEngine {
    pub fn new(db: Connection) -> Self {
        Self { db }
    }

    /// 执行搜索
    pub fn search(&self, filters: SearchFilters, page: usize, page_size: usize) -> Result<SearchResult> {
        // 1. 构建 SQL 查询（动态 WHERE 子句）
        // 2. 执行查询
        // 3. 返回分页结果
    }

    /// 获取搜索建议
    pub fn suggest_tags(&self, prefix: &str) -> Result<Vec<String>> {
        // SELECT name FROM tags WHERE name LIKE ? LIMIT 10
    }
}
```

**搜索语法：**
```
# 标签筛选
tag:海滩 tag:日落

# 文件类型
type:image 或 type:video

# 时间范围
date:2025-01-01..2025-12-31

# 文件大小
size:>1MB 或 size:<100KB

# 文件夹
folder:/Photos/2025
```

### 2.4 文件监控模块 (File Watcher)

```rust
// src-tauri/src/services/file_watcher.rs

use notify::{Event, EventKind, RecommendedWatcher, Watcher};

pub struct FileWatcher {
    watcher: RecommendedWatcher,
    tx: Sender<Event>,
}

impl FileWatcher {
    pub fn new(callback: Box<dyn Fn(Event) + Send>) -> Result<Self> {
        // 创建文件监控器
        // 监听指定目录
        // 触发增量扫描
    }

    pub fn watch(&mut self, path: &str) -> Result<()> {
        // 添加监控路径
    }

    pub fn unwatch(&mut self, path: &str) -> Result<()> {
        // 移除监控路径
    }
}
```

---

## 3. Tauri Commands 设计

### 3.1 文件相关命令

```rust
// src-tauri/src/commands/file.rs

#[tauri::command]
pub async fn scan_directory(
    path: String,
    incremental: bool,
    app_state: State<'_, AppState>,
) -> Result<ScanResult, String> {
    // 调用 FileScanner::scan_directory
}

#[tauri::command]
pub async fn get_files(
    filters: FileFilters,
    page: usize,
    page_size: usize,
    app_state: State<'_, AppState>,
) -> Result<Vec<MediaFile>, String> {
    // 调用 SearchEngine::search
}

#[tauri::command]
pub async fn get_file_metadata(
    file_id: i64,
    app_state: State<'_, AppState>,
) -> Result<MediaMetadata, String> {
    // 查询数据库返回详细信息
}

#[tauri::command]
pub async fn get_thumbnail(
    file_id: i64,
    size: u32,
    app_state: State<'_, AppState>,
) -> Result<Vec<u8>, String> {
    // 读取缩略图文件，返回二进制数据
}
```

### 3.2 标签相关命令

```rust
// src-tauri/src/commands/tag.rs

#[tauri::command]
pub async fn create_tag(
    name: String,
    parent_id: Option<i64>,
    color: Option<String>,
    app_state: State<'_, AppState>,
) -> Result<i64, String> {
    // 调用 TagManager::create_tag
}

#[tauri::command]
pub async fn list_tags(
    app_state: State<'_, AppState>,
) -> Result<Vec<Tag>, String> {
    // 查询所有标签
}

#[tauri::command]
pub async fn add_tags_to_media(
    media_id: i64,
    tag_ids: Vec<i64>,
    app_state: State<'_, AppState>,
) -> Result<(), String> {
    // 批量添加标签
}

#[tauri::command]
pub async fn remove_tags_from_media(
    media_id: i64,
    tag_ids: Vec<i64>,
    app_state: State<'_, AppState>,
) -> Result<(), String> {
    // 批量移除标签
}

#[tauri::command]
pub async fn get_tag_cloud(
    limit: usize,
    app_state: State<'_, AppState>,
) -> Result<Vec<TagUsage>, String> {
    // 获取标签云
}
```

### 3.3 相册相关命令

```rust
// src-tauri/src/commands/album.rs

#[tauri::command]
pub async fn create_album(
    name: String,
    description: Option<String>,
    app_state: State<'_, AppState>,
) -> Result<i64, String> {
    // 创建相册
}

#[tauri::command]
pub async fn add_media_to_album(
    album_id: i64,
    media_ids: Vec<i64>,
    app_state: State<'_, AppState>,
) -> Result<(), String> {
    // 添加媒体到相册（软链接）
}

#[tauri::command]
pub async fn list_albums(
    app_state: State<'_, AppState>,
) -> Result<Vec<Album>, String> {
    // 获取相册列表
}
```

---

## 4. 数据库设计

### 4.1 ER 图

```
┌─────────────────┐       ┌─────────────────┐
│   media_files   │       │      tags       │
├─────────────────┤       ├─────────────────┤
│ id (PK)         │       │ id (PK)         │
│ file_path       │       │ name            │
│ file_type       │       │ parent_id (FK)  │
│ file_size       │       │ color           │
│ width           │       │ created_at      │
│ height          │       └────────┬────────┘
│ duration        │                │
│ created_at      │                │
│ modified_at     │                │
│ taken_at        │                │
│ device          │                │
│ gps_lat         │                │
│ gps_lon         │                │
│ checksum        │                │
│ thumbnail_path  │                │
└────────┬────────┘                │
         │                         │
         │  ┌─────────────────┐    │
         └──│   media_tags    │────┘
            ├─────────────────┤
            │ media_id (FK)   │
            │ tag_id (FK)     │
            │ created_at      │
            └─────────────────┘
                  (composite PK)

┌─────────────────┐       ┌─────────────────┐
│     albums      │       │   album_media   │
├─────────────────┤       ├─────────────────┤
│ id (PK)         │       │ album_id (FK)   │
│ name            │       │ media_id (FK)   │
│ description     │       │ position        │
│ cover_media_id  │       └─────────────────┘
│ created_at      │              │
└────────┬────────┘              │
         │                       │
         └───────────────────────┘
```

### 4.2 索引优化

```sql
-- 常用查询索引
CREATE INDEX idx_media_type ON media_files(file_type);
CREATE INDEX idx_media_taken_at ON media_files(taken_at);
CREATE INDEX idx_media_checksum ON media_files(checksum);
CREATE INDEX idx_tags_parent ON tags(parent_id);
CREATE INDEX idx_media_tags_media ON media_tags(media_id);
CREATE INDEX idx_media_tags_tag ON media_tags(tag_id);

-- 复合索引
CREATE INDEX idx_media_type_taken ON media_files(file_type, taken_at);
CREATE INDEX idx_media_folder ON media_files(file_path);
```

---

## 5. Rust Crate 依赖

### 5.1 Cargo.toml 核心依赖

```toml
[package]
name = "media-organizer"
version = "0.1.0"
edition = "2021"

[dependencies]
# Tauri
tauri = { version = "2.0", features = ["shell-open"] }
tauri-plugin-shell = "2.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# 数据库
rusqlite = { version = "0.31", features = ["bundled"] }

# 图像处理
image = "0.25"
imageproc = "0.24"
kamadak-exif = "0.5"  # EXIF 解析

# 文件监控
notify = "6.1"

# 异步
tokio = { version = "1.0", features = ["full"] }
async-channel = "2.0"

# 工具
chrono = "0.4"
uuid = { version = "1.0", features = ["v4"] }
sha2 = "0.10"  # checksum 计算
walkdir = "2.0"  # 目录遍历

# 日志
log = "0.4"
env_logger = "0.11"
```

### 5.2 可选依赖（视频处理）

```toml
# 视频元数据（需要系统安装 ffmpeg）
ffmpeg-sidecar = "1.0"

# 或者纯 Rust 方案（功能有限）
mp4 = "0.14"
```

---

## 6. 性能优化策略

### 6.1 缩略图缓存
```
原始文件 → 首次访问生成缩略图 → 存入缓存目录 → 后续直接读取
缓存策略：LRU，最大 1GB，超出时删除最久未使用
```

### 6.2 虚拟滚动
```
前端使用 react-window 或 tanstack-virtual
只渲染可见区域的媒体项
支持 10 万 + 文件流畅浏览
```

### 6.3 增量扫描
```
使用 notify crate 监控文件系统变化
文件新增/修改/删除时自动触发更新
避免全量扫描的性能开销
```

### 6.4 数据库优化
```
- 分页查询（LIMIT + OFFSET）
- 全文搜索索引（FTS5）
- 定期 VACUUM 优化
- 连接池管理（单连接即可，Rust 性能好）
```

### 6.5 并发处理
```rust
// 使用 tokio 异步处理扫描任务
#[tauri::command]
pub async fn scan_directory(...) -> Result<ScanResult, String> {
    // 在后台线程池执行，不阻塞 UI
    tokio::task::spawn_blocking(move || {
        scanner.scan_directory(path, incremental)
    }).await?
}
```

---

## 7. 项目结构

```
media-organizer/
├── src/                    # 前端代码 (React + TypeScript)
│   ├── components/         # UI 组件
│   │   ├── Gallery/
│   │   ├── Timeline/
│   │   ├── TagCloud/
│   │   └── Search/
│   ├── hooks/              # React Hooks
│   ├── stores/             # 状态管理 (Zustand/Jotai)
│   ├── types/              # TypeScript 类型定义
│   └── App.tsx
├── src-tauri/              # Rust 后端代码
│   ├── src/
│   │   ├── main.rs         # Tauri 入口
│   │   ├── commands/       # Tauri Commands
│   │   │   ├── file.rs
│   │   │   ├── tag.rs
│   │   │   ├── album.rs
│   │   │   └── search.rs
│   │   ├── services/       # 业务逻辑
│   │   │   ├── file_scanner.rs
│   │   │   ├── tag_manager.rs
│   │   │   ├── search_engine.rs
│   │   │   └── file_watcher.rs
│   │   ├── models/         # 数据模型
│   │   │   ├── media.rs
│   │   │   ├── tag.rs
│   │   │   └── album.rs
│   │   └── db/             # 数据库操作
│   │       ├── schema.rs
│   │       └── migrations.rs
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── icons/              # 应用图标
├── docs/                   # 文档
├── package.json
└── tsconfig.json
```

---

## 8. 构建与部署

### 8.1 开发环境

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 Node.js 依赖
npm install

# 安装 Tauri CLI
npm install -g @tauri-apps/cli

# 启动开发模式
npm run tauri dev
```

### 8.2 生产打包

```bash
# 构建桌面应用
npm run tauri build

# 输出
src-tauri/target/release/
├── media-organizer.exe      # Windows
├── media-organizer.app      # macOS
└── media-organizer.AppImage # Linux
```

### 8.3 安装包大小预估

| 平台 | 大小 | 说明 |
|------|------|------|
| Windows | ~15 MB | 包含 Rust 运行时 |
| macOS | ~20 MB | Universal Binary |
| Linux | ~12 MB | AppImage |

---

## 9. 安全考虑

### 9.1 文件访问控制
```
- 只允许访问用户授权的目录
- 路径遍历攻击防护
- 文件操作权限检查
```

### 9.2 数据加密（可选）
```
- SQLite 数据库加密（SQLCipher）
- 敏感配置加密存储
```

---

**文档结束**  
*少爷，纯 Rust 方案更轻量，性能也更好！有啥需要调整的随时说！🦐*
