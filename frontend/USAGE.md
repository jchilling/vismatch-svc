# 使用說明 / Usage Guide

## 快速開始 / Quick Start

### 1. 啟動服務 / Start Services

首先，確保後端服務正在運行：
First, make sure the backend service is running:

```bash
# 從專案根目錄 / From project root
docker compose up -d image-compare-srv
```

然後啟動前端：
Then start the frontend:

```bash
cd frontend
npm run dev
```

訪問 http://localhost:5173 打開應用程式
Visit http://localhost:5173 to open the application

---

## 功能說明 / Features

### 功能一：圖片比對 / Image Comparison

**用途 / Purpose:**
上傳一張圖片，在指定專案的資料庫中尋找最相似的圖片。
Upload an image to find the most similar images from a project's database.

**使用步驟 / Steps:**

1. **輸入專案名稱 / Enter Project Name**
   - 在頂部的「專案名稱」輸入框中，輸入要搜尋的專案名稱
   - Enter the project name you want to search in the "專案名稱" input field at the top
   - 例如：`my_project`、`test_dataset` 等
   - Example: `my_project`, `test_dataset`, etc.

2. **切換到「圖片比對」標籤 / Switch to "圖片比對" Tab**
   - 點擊「圖片比對」標籤（預設已選中）
   - Click the "圖片比對" tab (selected by default)

3. **選擇要比對的圖片 / Select Image to Compare**
   - 點擊「選擇要比對的圖片」按鈕
   - Click "選擇要比對的圖片" button
   - 選擇一張圖片檔案（支援 JPG、PNG、GIF 等格式）
   - Select an image file (supports JPG, PNG, GIF, etc.)

4. **選擇是否包含圖片資料 / Choose Whether to Include Image Data**
   - ✅ 勾選「包含圖片資料」：結果會顯示相似圖片的縮圖
   - ✅ Check "包含圖片資料"：Results will show thumbnails of similar images
   - ❌ 不勾選：只顯示圖片名稱和相似度分數
   - ❌ Uncheck：Only show image names and similarity scores

5. **開始比對 / Start Comparison**
   - 點擊「開始比對」按鈕
   - Click "開始比對" button
   - 等待比對完成（會顯示「比對中...」）
   - Wait for comparison to complete (shows "比對中...")

6. **查看結果 / View Results**
   - 系統會顯示最相似的 3 張圖片（按相似度排序）
   - System will show the top 3 most similar images (sorted by similarity)
   - 每張圖片顯示：
   - Each image shows:
     - 排名（#1, #2, #3）
     - Rank (#1, #2, #3)
     - 圖片名稱
     - Image name
     - 距離分數（越低越相似）
     - Distance score (lower = more similar)
     - 圖片縮圖（如果勾選了包含圖片資料）
     - Image thumbnail (if "包含圖片資料" is checked)

**範例情境 / Example Scenario:**
- 你有一張嫌疑車輛的照片，想從資料庫中找出相似的車輛
- You have a photo of a suspect vehicle and want to find similar vehicles from the database
- 輸入專案名稱：`vehicle_database`
- Enter project name: `vehicle_database`
- 上傳照片並比對
- Upload photo and compare
- 查看最相似的結果
- View the most similar results

---

### 功能二：圖片上傳 / Image Upload

**用途 / Purpose:**
將新圖片上傳到指定專案的資料庫中，供後續比對使用。
Upload new images to a project's database for future comparisons.

**使用步驟 / Steps:**

1. **輸入專案名稱 / Enter Project Name**
   - 在頂部的「專案名稱」輸入框中，輸入專案名稱
   - Enter the project name in the "專案名稱" input field at the top
   - 如果專案不存在，系統會自動建立
   - If the project doesn't exist, the system will create it automatically

2. **切換到「圖片上傳」標籤 / Switch to "圖片上傳" Tab**
   - 點擊「圖片上傳」標籤
   - Click the "圖片上傳" tab

3. **選擇要上傳的圖片 / Select Image to Upload**
   - 點擊「選擇圖片檔案」按鈕
   - Click "選擇圖片檔案" button
   - 選擇一張圖片檔案
   - Select an image file
   - 圖片預覽會自動顯示
   - Image preview will be displayed automatically

4. **確認圖片名稱 / Confirm Image Name**
   - 系統會自動填入檔案名稱
   - System will auto-fill the file name
   - 可以修改圖片名稱（建議使用有意義的名稱）
   - You can modify the image name (recommended to use meaningful names)
   - 例如：`vehicle_001.jpg`、`person_20240101.png`
   - Example: `vehicle_001.jpg`, `person_20240101.png`

5. **上傳圖片 / Upload Image**
   - 點擊「上傳圖片」按鈕
   - Click "上傳圖片" button
   - 等待上傳完成（會顯示「上傳中...」）
   - Wait for upload to complete (shows "上傳中...")

6. **確認成功 / Confirm Success**
   - 看到成功訊息後，圖片已加入專案資料庫
   - After seeing success message, image has been added to project database
   - 可以立即使用「圖片比對」功能搜尋這張圖片
   - You can immediately use "圖片比對" to search for this image

**範例情境 / Example Scenario:**
- 建立一個新的專案來儲存車輛照片
- Create a new project to store vehicle photos
- 輸入專案名稱：`vehicle_database`
- Enter project name: `vehicle_database`
- 上傳多張車輛照片
- Upload multiple vehicle photos
- 之後可以用「圖片比對」功能搜尋相似車輛
- Later use "圖片比對" to search for similar vehicles

---

## 專案結構說明 / Project Structure

圖片儲存在後端的 `image_root/` 目錄下，按專案名稱組織：

Images are stored in the backend's `image_root/` directory, organized by project name:

```
image_root/
├── my_project/          # 專案名稱 / Project name
│   ├── image1.jpg       # 上傳的圖片 / Uploaded images
│   ├── image2.png
│   └── ...
└── another_project/
    ├── photo1.jpg
    └── ...
```

---

## 常見問題 / FAQ

### Q: 專案名稱可以包含中文嗎？
**A:** 可以，專案名稱支援中文、英文、數字等字元。

### Q: Can project names contain Chinese characters?
**A:** Yes, project names support Chinese, English, numbers, and other characters.

---

### Q: 比對結果中的「距離分數」是什麼意思？
**A:** 距離分數越低，表示圖片越相似。分數為 0 表示完全相同。

### Q: What does "距離分數" (distance score) mean in comparison results?
**A:** Lower distance score means more similar images. A score of 0 means identical.

---

### Q: 為什麼比對時看不到圖片縮圖？
**A:** 請確認在比對前勾選了「包含圖片資料」選項。

### Q: Why can't I see image thumbnails in comparison results?
**A:** Make sure you checked "包含圖片資料" before comparing.

---

### Q: 上傳失敗怎麼辦？
**A:** 請檢查：
- 後端服務是否正在運行
- 專案名稱是否正確
- 圖片檔案格式是否支援

### Q: What if upload fails?
**A:** Please check:
- Is the backend service running?
- Is the project name correct?
- Is the image file format supported?

---

## 技術細節 / Technical Details

- **前端技術 / Frontend:** React 19 + TypeScript + Vite
- **後端 API / Backend API:** Rust + Axum
- **圖片格式支援 / Supported Formats:** JPG, PNG, GIF, WebP 等
- **API 端點 / API Endpoints:**
  - `POST /diff` - 圖片比對
  - `POST /upload` - 圖片上傳

---

## 聯絡資訊 / Contact

如有問題，請參考專案 README 或聯絡開發團隊。
For issues, please refer to the project README or contact the development team.

