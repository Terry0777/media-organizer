import { useState } from 'react'
import { TimelineView } from './views/TimelineView'
import { TagView } from './views/TagView'
import './index.css'
import './App.css'

type View = 'timeline' | 'tags' | 'albums'

function App() {
  const [currentView, setCurrentView] = useState<View>('timeline')

  return (
    <div className="app">
      <header className="app-header">
        <div className="app-logo">
          <span className="logo-icon">📸</span>
          <span className="logo-text">Media Organizer</span>
        </div>
        
        <nav className="app-nav">
          <button
            className={`nav-btn ${currentView === 'timeline' ? 'active' : ''}`}
            onClick={() => setCurrentView('timeline')}
          >
            📁 Timeline
          </button>
          <button
            className={`nav-btn ${currentView === 'tags' ? 'active' : ''}`}
            onClick={() => setCurrentView('tags')}
          >
            🏷️ Tags
          </button>
          <button
            className={`nav-btn ${currentView === 'albums' ? 'active' : ''}`}
            onClick={() => setCurrentView('albums')}
            disabled
            title="Coming soon"
          >
            📔 Albums
          </button>
        </nav>

        <div className="app-actions">
          <button className="icon-btn" title="Settings">
            ⚙️
          </button>
        </div>
      </header>

      <main className="app-main">
        {currentView === 'timeline' && <TimelineView />}
        {currentView === 'tags' && <TagView />}
        {currentView === 'albums' && (
          <div className="empty-state">
            <div className="empty-icon">🚧</div>
            <h2>Coming Soon</h2>
            <p>Albums feature is under development</p>
          </div>
        )}
      </main>
    </div>
  )
}

export default App
