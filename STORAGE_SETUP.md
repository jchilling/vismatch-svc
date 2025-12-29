# Firebase Storage Setup Guide

This guide explains how to configure the backend to use Firebase Storage (Google Cloud Storage) instead of local filesystem storage.

## Overview

The backend now supports two storage backends:
1. **Local Storage** (default) - Files stored in `./image_root/` directory
2. **Google Cloud Storage** - Files stored in a GCS bucket (Firebase Storage)

## Why Use Firebase Storage?

- **Persistent**: Data survives container restarts (Cloud Run is stateless)
- **Scalable**: Handles large amounts of data
- **Integrated**: Works seamlessly with Firebase projects
- **Production-ready**: Suitable for production deployments

## Setup Instructions

### Step 1: Create a GCS Bucket

1. Go to [Google Cloud Console](https://console.cloud.google.com/storage)
2. Click "Create Bucket"
3. Configure:
   - **Name**: `vismatch-svc-images` (or your preferred name)
   - **Location**: Choose a region (e.g., `asia-east1`)
   - **Storage class**: Standard
   - **Access control**: Uniform
4. Click "Create"

### Step 2: Set Up Authentication

The backend uses Application Default Credentials (ADC) for authentication.

**For Cloud Run deployment:**
- ADC is automatically available - no setup needed!

**For local development:**
```bash
gcloud auth application-default login
```

### Step 3: Configure Environment Variable

Set the `GCS_BUCKET_NAME` environment variable to your bucket name.

**For Cloud Run:**
```bash
gcloud run services update vismatch-svc \
  --update-env-vars GCS_BUCKET_NAME=vismatch-svc-images \
  --region asia-east1
```

**For Docker Compose:**
Add to your `compose.yml`:
```yaml
services:
  image-compare-srv:
    environment:
      - GCS_BUCKET_NAME=vismatch-svc-images
```

**For local development:**
```bash
export GCS_BUCKET_NAME=vismatch-svc-images
```

### Step 4: Deploy

The backend will automatically detect the `GCS_BUCKET_NAME` environment variable and use GCS instead of local storage.

## Storage Structure

Images are stored in GCS with the following structure:
```
bucket-name/
├── project_name_1/
│   ├── image1.jpg
│   ├── image2.png
│   └── ...
└── project_name_2/
    └── photo.jpg
```

## Migration from Local to GCS

If you have existing images in local storage:

1. **Option 1: Manual Upload**
   ```bash
   gsutil -m cp -r ./image_root/* gs://your-bucket-name/
   ```

2. **Option 2: Re-upload via API**
   - Use the frontend to re-upload images
   - They will be stored in GCS automatically

## Cost Estimation

**GCS Pricing (Standard Storage in Asia):**
- Storage: $0.020 per GB/month
- Operations: $0.05 per 10,000 operations
- Network egress: $0.12 per GB (first 1TB/month free)

**Example for 10,000 images (~5GB):**
- Storage: ~$0.10/month
- Operations: ~$0.50/month (assuming 100k operations)
- **Total: ~$0.60/month**

## Troubleshooting

### Error: "Failed to get access token"

**Solution:**
- For Cloud Run: Ensure the service account has Storage permissions
- For local: Run `gcloud auth application-default login`

### Error: "GCS upload failed: 403"

**Solution:**
- Check bucket permissions
- Ensure the service account has "Storage Object Admin" role

### Error: "GCS download failed: 404"

**Solution:**
- Verify the image exists in the bucket
- Check the project name and image name are correct

## Permissions Required

The Cloud Run service account needs:
- `storage.objects.create` - Upload images
- `storage.objects.get` - Download images
- `storage.objects.delete` - Delete images
- `storage.objects.list` - List images in project

**Grant permissions:**
```bash
gcloud projects add-iam-policy-binding YOUR_PROJECT_ID \
  --member="serviceAccount:YOUR_SERVICE_ACCOUNT@YOUR_PROJECT_ID.iam.gserviceaccount.com" \
  --role="roles/storage.objectAdmin"
```

## Fallback to Local Storage

If `GCS_BUCKET_NAME` is not set, the backend automatically uses local filesystem storage (`./image_root/`).

This is useful for:
- Local development
- Testing
- Small deployments where GCS isn't needed

## Next Steps

1. Create your GCS bucket
2. Set the `GCS_BUCKET_NAME` environment variable
3. Deploy your backend
4. Test image uploads - they should now persist in GCS!

---

For API documentation, see [API.md](API.md)
For deployment instructions, see [FIREBASE_DEPLOY.md](FIREBASE_DEPLOY.md)

