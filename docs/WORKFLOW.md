# Media Organizer - 开发工作流

**版本：** v1.0  
**创建日期：** 2026-03-04  
**最后更新：** 2026-03-04

---

## 📋 分支管理规范

### 分支策略

| 分支 | 用途 | 保护 | 直接 Push |
|------|------|------|-----------|
| `main` | 生产分支，稳定版本 | ✅ 受保护 | ❌ 禁止 |
| `dev` | 开发分支，集成分支 | ✅ 受保护 | ❌ 禁止 |
| `feature/*` | 功能开发分支 | ❌ 不保护 | ✅ 允许 |
| `bugfix/*` | 修复分支 | ❌ 不保护 | ✅ 允许 |
| `hotfix/*` | 紧急修复分支 | ❌ 不保护 | ✅ 允许 |

---

## 🚫 禁止事项

- ❌ **禁止** 直接在 `main` 分支上开发
- ❌ **禁止** 直接 Push 到 `main` 分支
- ❌ **禁止** 在 `main` 分支上直接 Commit
- ❌ **禁止** 跳过 Code Review Merge 到 `main`

---

## ✅ 开发流程

### 1. 创建开发分支

```bash
# 确保基于最新 main
git checkout main
git pull origin main

# 创建新分支（命名规范：类型/描述）
git checkout -b feature/file-scanner
# 或
git checkout -b bugfix/thumbnail-cache
# 或
git checkout -b hotfix/crash-fix
```

### 2. 开发并提交

```bash
# 开发过程中多次提交
git add .
git commit -m "feat: implement file scanner"

# 推送到远程分支
git push -u origin feature/file-scanner
```

### 3. 请求 Merge

**开发完成后，在 Telegram 上通知 Terry：**

```
🦞 Media Organizer - Code Review 请求

📋 分支：feature/file-scanner → main
👤 开发者：@developer
📝 变更说明：
  - 实现文件扫描器
  - 支持图片/视频格式
  - 提取 EXIF 元数据

🔗 PR 链接：https://github.com/Terry0777/media-organizer/pull/1

请 Terry 进行 Code Review 并 Merge！
```

### 4. Code Review & Merge

**Terry 操作：**

1. 点击 PR 链接
2. 查看 Files changed
3. 添加 Review 评论（如有问题）
4. 批准并 Merge（Squash and Merge 或 Rebase and Merge）
5. 删除已合并的分支

---

## 📝 Commit 消息规范

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Type 类型

| Type | 说明 |
|------|------|
| `feat` | 新功能 |
| `fix` | Bug 修复 |
| `docs` | 文档更新 |
| `style` | 代码格式（不影响功能） |
| `refactor` | 重构 |
| `perf` | 性能优化 |
| `test` | 测试相关 |
| `chore` | 构建/工具/配置 |

### 示例

```bash
# 新功能
git commit -m "feat(scanner): implement file scanner with metadata extraction"

# Bug 修复
git commit -m "fix(thumbnail): resolve crash on large images"

# 文档更新
git commit -m "docs: update workflow guide"

# 重构
git commit -m "refactor(database): optimize SQLite queries"
```

---

## 🔧 GitHub 分支保护设置

### 设置步骤（Terry 操作）

1. 打开 https://github.com/Terry0777/media-organizer/settings/branches
2. 点击 **Add branch protection rule**
3. Branch name pattern: `main`
4. 勾选以下选项：
   - ✅ **Require a pull request before merging**
     - ✅ Require approvals (1)
     - ✅ Dismiss stale pull request approvals when new commits are pushed
   - ✅ **Require status checks to pass before merging**
   - ✅ **Require branches to be up to date before merging**
   - ✅ **Include administrators**
   - ✅ **Allow force pushes** (取消勾选)
   - ✅ **Allow deletions** (取消勾选)
5. 点击 **Create**

### 同样设置保护 `dev` 分支

---

## 📊 分支命名示例

```
# 功能开发
feature/file-scanner
feature/tag-management
feature/timeline-view
feature/thumbnail-cache

# Bug 修复
bugfix/crash-on-heic
bugfix/memory-leak
bugfix/wrong-exif-orientation

# 紧急修复
hotfix/login-issue
hotfix/build-failure

# 实验性（可选）
experiment/ai-tagging
experiment/cloud-sync
```

---

## 🔄 日常开发流程

### 开始一天的工作

```bash
# 1. 切换到 main 并更新
git checkout main
git pull origin main

# 2. 创建/切换到开发分支
git checkout -b feature/xxx
# 或
git checkout feature/xxx
git pull origin feature/xxx
```

### 开发中

```bash
# 多次提交
git add .
git commit -m "feat: xxx"

# 定期推送到远程
git push origin feature/xxx
```

### 完成开发

```bash
# 1. 确保分支最新
git pull origin feature/xxx

# 2. 变基到最新 main
git checkout main
git pull origin main
git checkout feature/xxx
git rebase main

# 3. 解决冲突（如有）
# 4. 推送（可能需要 force）
git push --force-with-lease origin feature/xxx

# 5. 创建 PR 并通知 Terry
```

---

## 📢 Telegram 通知模板

### Code Review 请求

```
🦞 Media Organizer - Code Review 请求

📋 分支：{branch} → main
👤 开发者：{developer}
📝 变更说明：
  - {change 1}
  - {change 2}
  - {change 3}

🔗 PR 链接：{pr_url}

请 Terry 进行 Code Review 并 Merge！
```

### 紧急修复通知

```
🚨 Media Organizer - Hotfix

📋 分支：{branch} → main
🔥 问题：{issue_description}
👤 开发者：{developer}

🔗 PR 链接：{pr_url}

紧急修复，请优先 Review！
```

---

## 🎯 最佳实践

1. **小步提交** - 每个 Commit 做一件事
2. **及时推送** - 每天至少 Push 一次
3. **保持同步** - 定期 Rebase 到最新 main
4. **清晰描述** - Commit 消息说明为什么，不只是做什么
5. **Code Review** - 认真 Review 他人代码
6. **测试先行** - 重要功能先写测试

---

## 📞 问题处理

### 如果不小心 Push 到 main

```bash
# 1. 立即回退（本地）
git checkout main
git reset --hard HEAD~1

# 2. 创建新分支
git checkout -b feature/rescue

# 3. 推送新分支
git push -u origin feature/rescue

# 4. 通知 Terry 恢复 main 分支
```

### 如果需要紧急修复

```bash
# 使用 hotfix 分支
git checkout main
git checkout -b hotfix/urgent-fix
# 修复...
git push -u origin hotfix/urgent-fix
# 通知 Terry 优先 Review
```

---

**文档结束**  
*遵守工作流，开发更高效！🦞*
