import { useState } from 'react';
import { api, fileToBase64 } from '../services/api';
import type { CompareImageResp, SimilarImageEntry } from '../types/api';

interface ImageCompareProps {
  projectName: string;
}

export default function ImageCompare({ projectName }: ImageCompareProps) {
  const [file, setFile] = useState<File | null>(null);
  const [preview, setPreview] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [results, setResults] = useState<SimilarImageEntry[]>([]);
  const [withImage, setWithImage] = useState(true);
  const [message, setMessage] = useState<{ type: 'success' | 'error'; text: string } | null>(null);

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const selectedFile = e.target.files?.[0];
    if (selectedFile) {
      if (!selectedFile.type.startsWith('image/')) {
        setMessage({ type: 'error', text: '請選擇圖片檔案' });
        return;
      }
      setFile(selectedFile);
      setResults([]);
      
      // Create preview
      const reader = new FileReader();
      reader.onload = (e) => {
        setPreview(e.target?.result as string);
      };
      reader.readAsDataURL(selectedFile);
    }
  };

  const handleCompare = async () => {
    if (!file) {
      setMessage({ type: 'error', text: '請先選擇要比對的圖片' });
      return;
    }

    if (!projectName.trim()) {
      setMessage({ type: 'error', text: '請輸入專案名稱' });
      return;
    }

    setLoading(true);
    setMessage(null);

    try {
      const base64Data = await fileToBase64(file);
      const response: CompareImageResp = await api.compareImage({
        project_name: projectName,
        data: base64Data,
        with_image: withImage,
      });

      if (response.success) {
        setResults(response.compare_result);
        setMessage({
          type: 'success',
          text: `找到 ${response.compare_result.length} 個相似圖片`,
        });
      } else {
        setMessage({ type: 'error', text: response.message || '比對失敗' });
        setResults([]);
      }
    } catch (error) {
      setMessage({
        type: 'error',
        text: error instanceof Error ? error.message : '比對失敗，請稍後再試',
      });
      setResults([]);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="image-compare-container">
      <h2>圖片相似度比對</h2>
      
      <div className="form-group">
        <label htmlFor="compare-file-input">選擇要比對的圖片</label>
        <input
          id="compare-file-input"
          type="file"
          accept="image/*"
          onChange={handleFileChange}
          disabled={loading}
        />
      </div>

      {preview && (
        <div className="preview-container">
          <h3>查詢圖片</h3>
          <img src={preview} alt="查詢圖片" className="preview-image" />
        </div>
      )}

      <div className="form-group checkbox-group">
        <label>
          <input
            type="checkbox"
            checked={withImage}
            onChange={(e) => setWithImage(e.target.checked)}
            disabled={loading}
          />
          包含圖片資料（顯示相似圖片）
        </label>
      </div>

      <button
        onClick={handleCompare}
        disabled={loading || !file}
        className="btn btn-primary"
      >
        {loading ? '比對中...' : '開始比對'}
      </button>

      {message && (
        <div className={`message message-${message.type}`}>
          {message.text}
        </div>
      )}

      {results.length > 0 && (
        <div className="results-container">
          <h3>比對結果（相似度由高到低）</h3>
          <div className="results-grid">
            {results.map((result, index) => (
              <div key={index} className="result-card">
                <div className="result-header">
                  <span className="result-rank">#{index + 1}</span>
                  <span className="result-name">{result.image_name}</span>
                </div>
                <div className="result-distance">
                  距離分數: {result.distance.toFixed(2)}
                </div>
                {result.data && (
                  <div className="result-image">
                    <img
                      src={`data:image/png;base64,${result.data}`}
                      alt={result.image_name}
                      className="result-img"
                    />
                  </div>
                )}
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}

