import { useState } from 'react'
import './App.css'

function App() {
  const [count, setCount] = useState(0)

  return (
    <div className="app">
      <header className="header">
        <div className="logo">🦞 Media Organizer</div>
        <div className="search-bar">
          <input type="text" placeholder="🔍 搜索文件、标签..." />
        </div>
        <div className="header-actions">
          <button className="icon-btn">⚙️</button>
        </div>
      </header>

      <div className="main-container">
        {/* 侧边栏 */}
        <aside className="sidebar">
          <nav className="nav-section">
            <div className="nav-title">库</div>
            <a href="#" className="nav-item active">📁 时间线</a>
            <a href="#" className="nav-item">📂 文件夹</a>
            <a href="#" className="nav-item">🏷️ 标签</a>
            <a href="#" className="nav-item">📸 相册</a>
          </nav>

          <div className="divider" />

          <nav className="nav-section">
            <div className="nav-title">标签</div>
            <a href="#" className="nav-item">#海滩 <span className="badge">124</span></a>
            <a href="#" className="nav-item">#人物 <span className="badge">89</span></a>
            <a href="#" className="nav-item">#美食 <span className="badge">56</span></a>
            <button className="add-tag-btn">+ 新建标签</button>
          </nav>
        </aside>

        {/* 主内容区 */}
        <main className="content">
          <div className="content-header">
            <h2>时间线</h2>
            <div className="view-controls">
              <span className="text-secondary">共 12,453 个文件</span>
            </div>
          </div>

          <div className="media-grid">
            {/* 示例媒体卡片 */}
            {[...Array(12)].map((_, i) => (
              <div key={i} className="media-card">
                <div className="media-thumbnail">
                  <div className="placeholder">🖼️</div>
                </div>
                <div className="media-info">
                  <span className="media-type">📷</span>
                  <span className="media-size">2.4 MB</span>
                </div>
              </div>
            ))}
          </div>
        </main>

        {/* 详情面板 */}
        <aside className="inspector">
          <div className="inspector-header">
            <h3>详情</h3>
          </div>
          <div className="inspector-content">
            <div className="inspector-placeholder">
              <p className="text-secondary">选择一个文件查看详情</p>
            </div>
          </div>
        </aside>
      </div>
    </div>
  )
}

export default App
