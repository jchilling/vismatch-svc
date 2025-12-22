import { useState } from 'react';
import ImageUpload from './components/ImageUpload';
import ImageCompare from './components/ImageCompare';
import './App.css';

type Tab = 'compare' | 'upload';

function App() {
  const [activeTab, setActiveTab] = useState<Tab>('compare');
  const [projectName, setProjectName] = useState('');

  return (
    <div className="app">
      <header className="app-header">
        <h1>相似影像辨識服務</h1>
        <p className="subtitle">Image Similarity Matching Service</p>
      </header>

      <main className="app-main">
        <div className="project-input-section">
          <label htmlFor="project-name">專案名稱</label>
          <input
            id="project-name"
            type="text"
            value={projectName}
            onChange={(e) => setProjectName(e.target.value)}
            placeholder="請輸入專案名稱"
            className="project-input"
          />
        </div>

        <div className="tabs">
          <button
            className={`tab ${activeTab === 'compare' ? 'active' : ''}`}
            onClick={() => setActiveTab('compare')}
          >
            圖片比對
          </button>
          <button
            className={`tab ${activeTab === 'upload' ? 'active' : ''}`}
            onClick={() => setActiveTab('upload')}
          >
            圖片上傳
          </button>
        </div>

        <div className="content-area">
          {activeTab === 'compare' && <ImageCompare projectName={projectName} />}
          {activeTab === 'upload' && (
            <ImageUpload
              projectName={projectName}
              onUploadSuccess={() => {
                // Optionally refresh or show success message
              }}
            />
          )}
        </div>
      </main>

      <footer className="app-footer">
        <p>內部政府工具 | Internal Government Tool</p>
      </footer>
    </div>
  );
}

export default App;
