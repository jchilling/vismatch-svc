# API Documentation

Complete API reference for vismatch-svc.

## Base URL

- **Local Development**: `http://localhost:3000`
- **Production**: Configure via environment variables

## Authentication

Currently, the API does not require authentication. For production deployment, consider adding authentication middleware.

## Endpoints

### 1. Compare Image

Find similar images in a project database.

**Endpoint:** `POST /diff`

**Request Body:**
```json
{
  "project_name": "string",
  "data": "string (base64 encoded image)",
  "with_image": boolean
}
```

**Parameters:**
- `project_name` (string, required): Name of the project to search in
- `data` (string, required): Base64-encoded image data (with or without `data:image/...;base64,` prefix)
- `with_image` (boolean, required): Whether to include image data in response

**Response:**
```json
{
  "success": true,
  "message": "success",
  "project_name": "string",
  "compare_result": [
    {
      "image_name": "string",
      "distance": 0.0,
      "data": "string (base64 image, optional)"
    }
  ]
}
```

**Response Fields:**
- `success` (boolean): Whether the operation succeeded
- `message` (string): Status message
- `project_name` (string): The project that was searched
- `compare_result` (array): Array of similar images, sorted by similarity (top 3)
  - `image_name` (string): Name of the similar image
  - `distance` (float): Similarity distance (lower = more similar, 0 = identical)
  - `data` (string, optional): Base64-encoded image data (only if `with_image: true`)

**Example Request:**
```bash
curl -X POST http://localhost:3000/diff \
  -H "Content-Type: application/json" \
  -d '{
    "project_name": "my_project",
    "data": "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==",
    "with_image": true
  }'
```

**Example Response:**
```json
{
  "success": true,
  "message": "success",
  "project_name": "my_project",
  "compare_result": [
    {
      "image_name": "similar_image.jpg",
      "distance": 2.5,
      "data": "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJ..."
    },
    {
      "image_name": "another_image.png",
      "distance": 5.3,
      "data": "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJ..."
    }
  ]
}
```

**Error Responses:**
- `400 Bad Request`: Invalid request data or project not found
- `500 Internal Server Error`: Server-side processing error

---

### 2. Upload Image

Upload an image to a project database.

**Endpoint:** `POST /upload`

**Request Body:**
```json
{
  "project_name": "string",
  "image_name": "string",
  "data": "string (base64 encoded image)"
}
```

**Parameters:**
- `project_name` (string, required): Name of the project (will be created if it doesn't exist)
- `image_name` (string, required): Name to save the image as
- `data` (string, required): Base64-encoded image data

**Response:**
```json
{
  "success": true,
  "message": "image uploaded and indexed successfully",
  "token": "string"
}
```

**Example Request:**
```bash
curl -X POST http://localhost:3000/upload \
  -H "Content-Type: application/json" \
  -d '{
    "project_name": "my_project",
    "image_name": "photo.jpg",
    "data": "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJ..."
  }'
```

**Example Response:**
```json
{
  "success": true,
  "message": "image uploaded and indexed successfully",
  "token": "dummy-deletion-token"
}
```

**Error Responses:**
- `400 Bad Request`: Invalid image data or project name
- `500 Internal Server Error`: Failed to save or process image

---

### 3. Delete Project

Delete a project and all its images.

**Endpoint:** `DELETE /project/{project_name}`

**Path Parameters:**
- `project_name` (string, required): Name of the project to delete

**Response:**
```json
{
  "success": true,
  "message": "Project 'project_name' deleted successfully"
}
```

**Example Request:**
```bash
curl -X DELETE http://localhost:3000/project/my_project
```

**Example Response:**
```json
{
  "success": true,
  "message": "Project 'my_project' deleted successfully"
}
```

**Error Responses:**
- `400 Bad Request`: Invalid project name format
- `404 Not Found`: Project does not exist (returns success: false)
- `500 Internal Server Error`: Failed to delete project directory

**Project Name Validation:**
Project names must:
- Be non-empty
- Be 64 characters or less
- Contain only Chinese characters, letters, numbers, and underscores
- Not contain path separators (`/`, `\`) or special characters

---

## Error Format

All error responses follow this format:

```json
{
  "message": "Error description"
}
```

**HTTP Status Codes:**
- `200 OK`: Request succeeded
- `400 Bad Request`: Invalid request parameters
- `404 Not Found`: Resource not found
- `500 Internal Server Error`: Server error

---

## Rate Limits

Currently, there are no rate limits. For production deployment, consider implementing rate limiting.

## CORS

The API supports CORS and allows requests from any origin. For production, consider restricting allowed origins.

---

## Image Format Support

Supported image formats:
- JPEG/JPG
- PNG
- GIF
- WebP
- BMP

Images are automatically processed and indexed using perceptual hashing (pHash algorithm).

---

## Response Limits

- **Comparison Results**: Returns top 3 most similar images (configurable in code)
- **Image Size**: No hard limit, but very large images may take longer to process
- **Base64 Encoding**: Images should be base64-encoded in requests

---

## Best Practices

1. **Project Names**: Use descriptive, consistent naming (e.g., `vehicle_database`, `person_records`)
2. **Image Names**: Use meaningful filenames (e.g., `vehicle_001.jpg` instead of `IMG_1234.jpg`)
3. **Batch Operations**: Use the frontend's multiple upload feature for adding many images
4. **Error Handling**: Always check the `success` field in responses
5. **Distance Scores**: Lower distance = more similar (0 = identical)

---

## Examples

### Complete Workflow

1. **Upload images to a project:**
   ```bash
   curl -X POST http://localhost:3000/upload \
     -H "Content-Type: application/json" \
     -d '{
       "project_name": "test_project",
       "image_name": "test1.jpg",
       "data": "<base64_image_data>"
     }'
   ```

2. **Compare an image:**
   ```bash
   curl -X POST http://localhost:3000/diff \
     -H "Content-Type: application/json" \
     -d '{
       "project_name": "test_project",
       "data": "<base64_image_data>",
       "with_image": true
     }'
   ```

3. **Delete the project:**
   ```bash
   curl -X DELETE http://localhost:3000/project/test_project
   ```

---

## Testing

Test the API endpoints:

```bash
# Test upload
curl -X POST http://localhost:3000/upload \
  -H "Content-Type: application/json" \
  -d '{"project_name":"test","image_name":"test.jpg","data":"<base64>"}'

# Test comparison
curl -X POST http://localhost:3000/diff \
  -H "Content-Type: application/json" \
  -d '{"project_name":"test","data":"<base64>","with_image":false}'

# Test delete
curl -X DELETE http://localhost:3000/project/test
```

---

For frontend usage, see [frontend/USAGE.md](frontend/USAGE.md).

