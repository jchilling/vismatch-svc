import { useState } from 'react';
import { api, fileToBase64 } from '../services/api';
import type { UploadImageResp } from '../types/api';

interface ImageUploadProps {
  projectName: string;
  onUploadSuccess?: () => void;
}

interface FileUploadStatus {
  file: File;
  status: 'pending' | 'uploading' | 'success' | 'error';
  message?: string;
}

export default function ImageUpload({ projectName, onUploadSuccess }: ImageUploadProps) {
  const [files, setFiles] = useState<FileUploadStatus[]>([]);
  const [loading, setLoading] = useState(false);
  const [message, setMessage] = useState<{ type: 'success' | 'error'; text: string } | null>(null);

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const selectedFiles = Array.from(e.target.files || []);
    
    if (selectedFiles.length === 0) return;

    // Validate all files are images
    const invalidFiles = selectedFiles.filter(f => !f.type.startsWith('image/'));
    if (invalidFiles.length > 0) {
      setMessage({ type: 'error', text: `有 ${invalidFiles.length} 個檔案不是圖片格式` });
      return;
    }

    // Add new files to the list
    const newFiles: FileUploadStatus[] = selectedFiles.map(file => ({
      file,
      status: 'pending' as const,
    }));

    setFiles(prev => [...prev, ...newFiles]);
    setMessage(null);
  };

  const removeFile = (index: number) => {
    setFiles(prev => prev.filter((_, i) => i !== index));
  };

  const handleUpload = async () => {
    if (files.length === 0) {
      setMessage({ type: 'error', text: '請至少選擇一張圖片' });
      return;
    }

    if (!projectName.trim()) {
      setMessage({ type: 'error', text: '請輸入專案名稱' });
      return;
    }

    setLoading(true);
    setMessage(null);

    // Update all files to uploading status
    setFiles(prev => prev.map(f => ({ ...f, status: 'uploading' as const })));

    let successCount = 0;
    let errorCount = 0;

    // Upload files sequentially
    for (let i = 0; i < files.length; i++) {
      const fileStatus = files[i];
      
      try {
        const base64Data = await fileToBase64(fileStatus.file);
        const response: UploadImageResp = await api.uploadImage({
          project_name: projectName,
          image_name: fileStatus.file.name,
          data: base64Data,
        });

        if (response.success) {
          successCount++;
          setFiles(prev => prev.map((f, idx) => 
            idx === i 
              ? { ...f, status: 'success' as const, message: '上傳成功' }
              : f
          ));
        } else {
          errorCount++;
          setFiles(prev => prev.map((f, idx) => 
            idx === i 
              ? { ...f, status: 'error' as const, message: response.message || '上傳失敗' }
              : f
          ));
        }
      } catch (error) {
        errorCount++;
        setFiles(prev => prev.map((f, idx) => 
          idx === i 
            ? { ...f, status: 'error' as const, message: error instanceof Error ? error.message : '上傳失敗' }
            : f
        ));
      }
    }

    // Show summary message
    if (successCount > 0 && errorCount === 0) {
      setMessage({ type: 'success', text: `成功上傳 ${successCount} 張圖片` });
      // Clear successful uploads after a delay
      setTimeout(() => {
        setFiles(prev => prev.filter(f => f.status !== 'success'));
        onUploadSuccess?.();
      }, 2000);
    } else if (successCount > 0 && errorCount > 0) {
      setMessage({ type: 'error', text: `成功 ${successCount} 張，失敗 ${errorCount} 張` });
    } else {
      setMessage({ type: 'error', text: `所有圖片上傳失敗` });
    }

    setLoading(false);
  };

  return (
    <div className="image-upload-container">
      <h2>上傳圖片</h2>
      
      <div className="form-group">
        <label htmlFor="file-input">選擇圖片檔案（可多選）</label>
        <input
          id="file-input"
          type="file"
          accept="image/*"
          multiple
          onChange={handleFileChange}
          disabled={loading}
        />
        <small className="form-hint">可一次選擇多張圖片進行上傳</small>
      </div>

      {files.length > 0 && (
        <div className="files-list">
          <h3>已選擇的檔案 ({files.length})</h3>
          <div className="files-grid">
            {files.map((fileStatus, index) => (
              <div key={index} className={`file-item file-${fileStatus.status}`}>
                <div className="file-preview">
                  <img 
                    src={URL.createObjectURL(fileStatus.file)} 
                    alt={fileStatus.file.name}
                    className="file-thumbnail"
                  />
                  <button
                    className="file-remove"
                    onClick={() => removeFile(index)}
                    disabled={loading || fileStatus.status === 'uploading'}
                    title="移除"
                  >
                    ×
                  </button>
                </div>
                <div className="file-info">
                  <div className="file-name" title={fileStatus.file.name}>
                    {fileStatus.file.name}
                  </div>
                  <div className="file-status">
                    {fileStatus.status === 'pending' && <span className="status-pending">等待上傳</span>}
                    {fileStatus.status === 'uploading' && <span className="status-uploading">上傳中...</span>}
                    {fileStatus.status === 'success' && <span className="status-success">✓ 成功</span>}
                    {fileStatus.status === 'error' && (
                      <span className="status-error" title={fileStatus.message}>
                        ✗ 失敗
                      </span>
                    )}
                  </div>
                  {fileStatus.message && fileStatus.status === 'error' && (
                    <div className="file-error-message">{fileStatus.message}</div>
                  )}
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      <button
        onClick={handleUpload}
        disabled={loading || files.length === 0}
        className="btn btn-primary"
      >
        {loading ? `上傳中... (${files.filter(f => f.status === 'uploading').length}/${files.length})` : `上傳 ${files.length} 張圖片`}
      </button>

      {message && (
        <div className={`message message-${message.type}`}>
          {message.text}
        </div>
      )}
    </div>
  );
}

