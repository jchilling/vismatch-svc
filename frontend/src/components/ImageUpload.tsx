import { useState } from 'react';
import { api, fileToBase64 } from '../services/api';
import type { UploadImageResp } from '../types/api';

interface ImageUploadProps {
  projectName: string;
  onUploadSuccess?: () => void;
}

export default function ImageUpload({ projectName, onUploadSuccess }: ImageUploadProps) {
  const [file, setFile] = useState<File | null>(null);
  const [imageName, setImageName] = useState('');
  const [preview, setPreview] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [message, setMessage] = useState<{ type: 'success' | 'error'; text: string } | null>(null);

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const selectedFile = e.target.files?.[0];
    if (selectedFile) {
      // Validate image file
      if (!selectedFile.type.startsWith('image/')) {
        setMessage({ type: 'error', text: '請選擇圖片檔案' });
        return;
      }
      setFile(selectedFile);
      setImageName(selectedFile.name);
      
      // Create preview
      const reader = new FileReader();
      reader.onload = (e) => {
        setPreview(e.target?.result as string);
      };
      reader.readAsDataURL(selectedFile);
    }
  };

  const handleUpload = async () => {
    if (!file || !imageName.trim()) {
      setMessage({ type: 'error', text: '請選擇圖片並輸入圖片名稱' });
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
      const response: UploadImageResp = await api.uploadImage({
        project_name: projectName,
        image_name: imageName,
        data: base64Data,
      });

      if (response.success) {
        setMessage({ type: 'success', text: response.message || '圖片上傳成功' });
        setFile(null);
        setImageName('');
        setPreview(null);
        onUploadSuccess?.();
      } else {
        setMessage({ type: 'error', text: response.message || '上傳失敗' });
      }
    } catch (error) {
      setMessage({
        type: 'error',
        text: error instanceof Error ? error.message : '上傳失敗，請稍後再試',
      });
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="image-upload-container">
      <h2>上傳圖片</h2>
      
      <div className="form-group">
        <label htmlFor="file-input">選擇圖片檔案</label>
        <input
          id="file-input"
          type="file"
          accept="image/*"
          onChange={handleFileChange}
          disabled={loading}
        />
      </div>

      {preview && (
        <div className="preview-container">
          <img src={preview} alt="預覽" className="preview-image" />
        </div>
      )}

      <div className="form-group">
        <label htmlFor="image-name">圖片名稱</label>
        <input
          id="image-name"
          type="text"
          value={imageName}
          onChange={(e) => setImageName(e.target.value)}
          placeholder="例如: image.jpg"
          disabled={loading}
        />
      </div>

      <button
        onClick={handleUpload}
        disabled={loading || !file || !imageName.trim()}
        className="btn btn-primary"
      >
        {loading ? '上傳中...' : '上傳圖片'}
      </button>

      {message && (
        <div className={`message message-${message.type}`}>
          {message.text}
        </div>
      )}
    </div>
  );
}

