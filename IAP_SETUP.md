# Identity-Aware Proxy (IAP) Setup

This guide explains how to set up Google Cloud Identity-Aware Proxy (IAP) for the vismatch-svc service.

## Overview

The service uses IAP for authentication. When requests go through IAP:
- IAP authenticates users using Google Cloud IAM
- IAP adds headers to authenticated requests:
  - `X-Goog-Authenticated-User-Email`: User's email
  - `X-Goog-Authenticated-User-ID`: User's ID
  - `X-Goog-IAP-JWT-Assertion`: JWT assertion (optional verification)
- The backend verifies these headers to ensure requests are authenticated

## Backend Implementation

The backend automatically:
1. Checks for `X-Goog-Authenticated-User-Email` and `X-Goog-Authenticated-User-ID` headers
2. Extracts user information from these headers
3. Returns 401 Unauthorized if headers are missing

**No configuration needed** - the backend works automatically when deployed behind IAP.

## Setting Up IAP for Cloud Run

### 1. Enable IAP for Your Cloud Run Service

1. Go to [Google Cloud Console](https://console.cloud.google.com/)
2. Navigate to **Security** > **Identity-Aware Proxy**
3. Click **Enable API** if not already enabled
4. Go to **Cloud Run** services
5. Find your `vismatch-svc` service
6. Click on the service name
7. Go to the **Security** tab
8. Under **Authentication**, select **需要驗證** (Require authentication)
9. Configure IAM access:
   - Add users/groups who should have access
   - Grant them the `roles/run.invoker` role or use IAP-specific roles

### 2. Configure IAP Access

1. In the IAP page, select your Cloud Run service
2. Click **Add Principal**
3. Add users or groups who should have access:
   - For individual users: `user@example.com`
   - For groups: `group@example.com`
   - For all authenticated users: `allAuthenticatedUsers`
4. Grant role: **IAP-secured Web App User** or **Cloud Run Invoker**

### 3. Frontend Configuration

**Important**: The frontend must be accessed through IAP, not directly.

1. **Option A: Deploy frontend behind IAP** (Recommended)
   - Deploy frontend to Cloud Run or App Engine
   - Enable IAP for the frontend service
   - Users access frontend through IAP URL
   - Frontend makes requests to backend (also behind IAP)
   - IAP headers are automatically forwarded

2. **Option B: Use IAP-protected backend URL**
   - Frontend makes requests to the IAP-protected backend URL
   - IAP handles authentication before requests reach the backend
   - Users must be authenticated via IAP to access the frontend

### 4. Testing IAP

1. Access your Cloud Run service URL
2. You should be redirected to Google sign-in if not authenticated
3. After authentication, IAP adds headers to requests
4. The backend verifies these headers

### 5. Testing Locally (Development)

For local development, you can:

**Option 1: Disable IAP check in development**
```rust
// In src/auth.rs, add environment variable check
pub async fn verify_iap_auth(headers: &HeaderMap) -> Result<IapUser, StatusCode> {
    // Skip IAP check in development
    if std::env::var("SKIP_IAP_CHECK").is_ok() {
        return Ok(IapUser {
            email: "dev@example.com".to_string(),
            id: "dev-user".to_string(),
        });
    }
    // ... rest of IAP verification
}
```

**Option 2: Use IAP proxy or test with IAP headers**
```bash
# Manually add IAP headers for testing
curl -X POST http://localhost:3000/upload \
  -H "X-Goog-Authenticated-User-Email: accounts.google.com:user@example.com" \
  -H "X-Goog-Authenticated-User-ID: accounts.google.com:123456789" \
  -H "Content-Type: application/json" \
  -d '{"project_name":"test","image_name":"test.png","data":"..."}'
```

## How It Works

1. **User accesses frontend** → IAP authenticates user
2. **Frontend makes API request** → Request goes through IAP
3. **IAP adds headers**:
   ```
   X-Goog-Authenticated-User-Email: accounts.google.com:user@example.com
   X-Goog-Authenticated-User-ID: accounts.google.com:123456789
   X-Goog-IAP-JWT-Assertion: <jwt-token>
   ```
4. **Backend receives request** → Verifies IAP headers
5. **If headers valid** → Process request
6. **If headers missing** → Return 401 Unauthorized

## Security Notes

1. **IAP handles authentication**: Users authenticate through Google Cloud IAM, not your application
2. **Headers are trusted**: IAP ensures only authenticated requests reach your backend
3. **Optional JWT verification**: You can verify `X-Goog-IAP-JWT-Assertion` for additional security, but it's not required
4. **HTTPS required**: IAP only works over HTTPS in production

## Troubleshooting

### 403 Forbidden when accessing service
- Check IAP is enabled for the Cloud Run service
- Verify user has IAP access (granted in IAP settings)
- Check user is signed in with correct Google account

### 401 Unauthorized from backend
- Verify IAP headers are present (check Cloud Run logs)
- Ensure requests are going through IAP (not direct to Cloud Run)
- Check backend is reading headers correctly

### Frontend can't reach backend
- Ensure frontend is also behind IAP or has access to IAP-protected backend
- Check CORS settings if frontend and backend are on different domains
- Verify backend URL in frontend configuration

## Deployment

When deploying to Cloud Run:

```bash
# Deploy backend (IAP will be configured in Cloud Console)
gcloud run deploy vismatch-svc \
  --image gcr.io/doit-daniels-team/vismatch-svc:latest \
  --platform managed \
  --region asia-east1 \
  --allow-unauthenticated \
  --port 3000 \
  --set-env-vars GCS_BUCKET_NAME=vismatch-svc-images
```

**Note**: `--allow-unauthenticated` allows IAP to handle authentication. IAP will still require authentication before requests reach your service.

After deployment, enable IAP in the Cloud Console as described above.

