import { useState } from 'react';
import ImageUpload from './components/ImageUpload';
import ImageCompare from './components/ImageCompare';
import { api } from './services/api';
import './App.css';

type Tab = 'compare' | 'upload';

function App() {
  const [activeTab, setActiveTab] = useState<Tab>('compare');
  const [projectName, setProjectName] = useState('');
  const [deleteLoading, setDeleteLoading] = useState(false);
  const [deleteMessage, setDeleteMessage] = useState<{ type: 'success' | 'error'; text: string } | null>(null);

  const handleDeleteProject = async () => {
    if (!projectName.trim()) {
      setDeleteMessage({ type: 'error', text: '請輸入專案名稱' });
      return;
    }

    if (!confirm(`確定要刪除專案 "${projectName}" 嗎？此操作無法復原！\n\nAre you sure you want to delete project "${projectName}"? This action cannot be undone!`)) {
      return;
    }

    setDeleteLoading(true);
    setDeleteMessage(null);

    try {
      const response = await api.deleteProject({ project_name: projectName });
      if (response.success) {
        setDeleteMessage({ type: 'success', text: response.message });
        setProjectName(''); // Clear project name after successful deletion
      } else {
        setDeleteMessage({ type: 'error', text: response.message });
      }
    } catch (error) {
      console.error('Delete project error:', error);
      let errorMessage = '刪除失敗，請稍後再試';
      if (error instanceof Error) {
        errorMessage = error.message;
        // Check if it's a network error
        if (error.message.includes('Network Error') || error.message.includes('Failed to fetch')) {
          errorMessage = '無法連接到伺服器，請確認後端服務是否正在運行';
        }
      }
      setDeleteMessage({
        type: 'error',
        text: errorMessage,
      });
    } finally {
      setDeleteLoading(false);
    }
  };

  return (
    <div className="app">
      <header className="app-header">
        <h1>相似影像辨識服務</h1>
        <p className="subtitle">Image Similarity Matching Service</p>
      </header>

      <main className="app-main">
        <div className="project-input-section">
          <div className="project-input-row">
            <div className="project-input-group">
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
            {projectName.trim() && (
              <button
                onClick={handleDeleteProject}
                disabled={deleteLoading}
                className="btn btn-danger"
              >
                {deleteLoading ? '刪除中...' : '刪除專案'}
              </button>
            )}
          </div>
          {deleteMessage && (
            <div className={`message message-${deleteMessage.type}`}>
              {deleteMessage.text}
            </div>
          )}
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
