# Firebase Deployment Guide

This guide explains how to deploy the vismatch-svc frontend to Firebase Hosting.

**Note:** The Rust backend cannot run on Firebase Functions (Firebase only supports Node.js, Python, Go). You'll need to deploy the backend separately to a service that supports Rust (see [Backend Deployment Options](#backend-deployment-options)).

## Prerequisites

1. **Firebase CLI** installed:
   ```bash
   npm install -g firebase-tools
   ```

2. **Firebase account** and project created:
   - Go to [Firebase Console](https://console.firebase.google.com/)
   - Create a new project or use an existing one

3. **Backend deployed** to a service that supports Rust (see options below)

## Frontend Deployment to Firebase

### Step 1: Install Firebase CLI

```bash
npm install -g firebase-tools
```

### Step 2: Login to Firebase

```bash
firebase login
```

This will open a browser for authentication.

### Step 3: Initialize Firebase in Project

```bash
# From project root
firebase init hosting
```

When prompted:
- **Select an existing project** or create a new one
- **Public directory**: `frontend/dist`
- **Single-page app**: Yes (configure as a single-page app)
- **Overwrite index.html**: No (we'll build it)

### Step 4: Build Frontend

```bash
cd frontend

# Create .env.production with your backend URL
echo "VITE_API_URL=https://your-backend-url.com" > .env.production

# Build for production
npm run build
```

**Important:** Update `VITE_API_URL` in `.env.production` to point to your deployed backend URL.

### Step 5: Deploy

```bash
# From project root
firebase deploy --only hosting
```

Your frontend will be available at: `https://YOUR_PROJECT_ID.web.app`

### Step 6: Custom Domain (Optional)

1. Go to Firebase Console → Hosting
2. Click "Add custom domain"
3. Follow the instructions to verify your domain
4. Update DNS records as instructed

## Backend Deployment Options

Since Firebase Functions don't support Rust, deploy the backend to one of these services:

### Option 1: Google Cloud Run (Recommended)

**Why:** Same ecosystem as Firebase, easy integration

**Steps:**
1. Build Docker image:
   ```bash
   docker build -t gcr.io/YOUR_PROJECT_ID/vismatch-svc .
   ```

2. Push to Google Container Registry:
   ```bash
   gcloud auth configure-docker
   docker push gcr.io/YOUR_PROJECT_ID/vismatch-svc
   ```

3. Deploy to Cloud Run:
   ```bash
   gcloud run deploy vismatch-svc \
     --image gcr.io/YOUR_PROJECT_ID/vismatch-svc \
     --platform managed \
     --region us-central1 \
     --allow-unauthenticated \
     --port 3000
   ```

4. Get the Cloud Run URL and update frontend `VITE_API_URL`

### Option 2: Railway

**Why:** Simple, supports Docker, good for Rust

**Steps:**
1. Sign up at [railway.app](https://railway.app)
2. Create new project
3. Connect GitHub repository
4. Add service → Deploy from GitHub
5. Railway will detect `Dockerfile` and deploy automatically
6. Get the Railway URL and update frontend `VITE_API_URL`

### Option 3: Fly.io

**Why:** Great Rust support, global deployment

**Steps:**
1. Install Fly CLI: `curl -L https://fly.io/install.sh | sh`
2. Login: `fly auth login`
3. Launch: `fly launch` (from project root)
4. Deploy: `fly deploy`
5. Get the Fly.io URL and update frontend `VITE_API_URL`

### Option 4: Render

**Why:** Simple Docker deployment

**Steps:**
1. Sign up at [render.com](https://render.com)
2. Create new Web Service
3. Connect GitHub repository
4. Configure:
   - Build Command: `docker build -t vismatch-svc .`
   - Start Command: `docker run -p 3000:3000 vismatch-svc`
5. Deploy and get URL

## Complete Deployment Workflow

### 1. Deploy Backend First

Choose one of the backend options above and deploy. Note the backend URL (e.g., `https://vismatch-svc-xxx.run.app`).

### 2. Update Frontend Configuration

```bash
cd frontend
echo "VITE_API_URL=https://your-backend-url.com" > .env.production
npm run build
```

### 3. Deploy Frontend to Firebase

```bash
# From project root
firebase deploy --only hosting
```

### 4. Update CORS (If Needed)

If you get CORS errors, update the backend CORS configuration in `src/main.rs` to allow your Firebase domain:

```rust
let cors = CorsLayer::new()
    .allow_origin("https://YOUR_PROJECT_ID.web.app".parse::<HeaderValue>().unwrap())
    .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::OPTIONS])
    .allow_headers(Any);
```

Or for custom domain:
```rust
.allow_origin("https://your-domain.com".parse::<HeaderValue>().unwrap())
```

## Environment Variables

### Frontend (.env.production)

```env
VITE_API_URL=https://your-backend-url.com
```

### Backend (Cloud Run / Railway / etc.)

Set these environment variables in your deployment platform:
- No required env vars currently, but you may want to add:
  - `RUST_LOG=info` (for logging)
  - `PORT=3000` (if your platform requires it)

## Continuous Deployment

### GitHub Actions for Firebase

Create `.github/workflows/deploy-firebase.yml`:

```yaml
name: Deploy to Firebase

on:
  push:
    branches: [ main ]
    paths:
      - 'frontend/**'

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Node
        uses: actions/setup-node@v3
        with:
          node-version: '20'
      
      - name: Install dependencies
        run: |
          cd frontend
          npm install
      
      - name: Build
        run: |
          cd frontend
          echo "VITE_API_URL=${{ secrets.VITE_API_URL }}" > .env.production
          npm run build
      
      - name: Deploy to Firebase
        uses: FirebaseExtended/action-hosting-deploy@v0
        with:
          repoToken: '${{ secrets.GITHUB_TOKEN }}'
          firebaseServiceAccount: '${{ secrets.FIREBASE_SERVICE_ACCOUNT }}'
          channelId: live
          projectId: your-project-id
```

**Setup:**
1. Get Firebase service account: Firebase Console → Project Settings → Service Accounts
2. Add secrets to GitHub: Settings → Secrets → Actions
   - `FIREBASE_SERVICE_ACCOUNT`: JSON key content
   - `VITE_API_URL`: Your backend URL

## Troubleshooting

### CORS Errors

- Verify backend CORS allows your Firebase domain
- Check backend is accessible from browser
- Verify `VITE_API_URL` is correct in production build

### Build Fails

- Check Node.js version (requires 18+)
- Verify all dependencies installed: `npm install`
- Check for TypeScript errors: `npm run build`

### Backend Not Accessible

- Verify backend is deployed and running
- Check backend logs for errors
- Test backend URL directly: `curl https://your-backend-url.com/diff`

## Cost Estimation

**Firebase Hosting:**
- Free tier: 10 GB storage, 360 MB/day transfer
- Paid: $0.026/GB storage, $0.15/GB transfer

**Backend (Cloud Run example):**
- Free tier: 2 million requests/month
- Paid: $0.40 per million requests

## Security Considerations

1. **Add authentication** to backend API for production
2. **Use HTTPS** for all connections (Firebase provides this automatically)
3. **Restrict CORS** to your Firebase domain only
4. **Validate inputs** (already implemented in backend)
5. **Rate limiting** (consider adding for production)

## Next Steps

1. Deploy backend to your chosen platform
2. Deploy frontend to Firebase
3. Configure custom domain (optional)
4. Set up CI/CD with GitHub Actions
5. Monitor usage and costs

---

For API documentation, see [API.md](API.md)

