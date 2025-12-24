# vismatch-svc

相似影像辨識服務 | Image Similarity Matching Service

A high-performance Rust-based microservice for image similarity matching with a modern React + TypeScript frontend. Designed for internal government use.

## Quick Start

```bash
# Build and start services
docker compose build
docker compose up -d

# Access the services
# Backend API: http://localhost:3000
# Frontend UI: http://localhost:8080
```

## Features

- **Image Comparison**: Find similar images using perceptual hashing
- **Batch Upload**: Upload multiple images at once
- **Project Management**: Create and delete project databases
- **Modern Web UI**: Professional React frontend
- **Docker Ready**: One-command deployment

## Installation

### Prerequisites

- Docker and Docker Compose
- 4GB+ RAM recommended

### Setup

1. Clone the repository
2. Build containers: `docker compose build`
3. Start services: `docker compose up -d`

Images are stored in `./image_root/` organized by project name.

## API Documentation

See [API.md](API.md) for complete API documentation.

## Frontend

The frontend is a React + TypeScript application in the `frontend/` directory.

**Development:**
```bash
cd frontend
npm install
npm run dev
```

**Production Build:**
```bash
cd frontend
npm run build
```

See [frontend/USAGE.md](frontend/USAGE.md) for detailed usage instructions.

## Deployment

### Docker Compose

```bash
docker compose build
docker compose up -d
```

### Production

1. Update `VITE_API_URL` in `compose.yml` to your API domain
2. Rebuild: `docker compose build frontend`
3. Restart: `docker compose up -d`

## GitHub Setup

### Initial Setup

1. Create a new repository on GitHub
2. Initialize git (if not done):
   ```bash
   git init
   git add .
   git commit -m "Initial commit"
   ```
3. Add remote and push:
   ```bash
   git remote add origin https://github.com/YOUR_USERNAME/vismatch-svc.git
   git branch -M main
   git push -u origin main
   ```

### .gitignore

Create `.gitignore`:
```gitignore
# Rust
target/
Cargo.lock

# Node
node_modules/
frontend/dist/
frontend/.env

# Docker
.env

# IDE
.vscode/
.idea/
*.swp

# Logs
*.log
.cursor/

# OS
.DS_Store
Thumbs.db
```

### GitHub Pages (Frontend) - Optional

To host frontend on GitHub Pages:

1. Update `frontend/vite.config.ts`:
   ```typescript
   export default defineConfig({
     plugins: [react()],
     base: '/vismatch-svc/', // Your repo name
   })
   ```

2. Create `.github/workflows/deploy-frontend.yml`:
   ```yaml
   name: Deploy Frontend
   
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
         - uses: actions/setup-node@v3
           with:
             node-version: '20'
         - run: |
             cd frontend
             npm install
             npm run build
         - uses: peaceiris/actions-gh-pages@v3
           with:
             github_token: ${{ secrets.GITHUB_TOKEN }}
             publish_dir: ./frontend/dist
   ```

3. Enable GitHub Pages in repository Settings → Pages (Source: GitHub Actions)

## Project Structure

```
vismatch-svc/
├── src/              # Rust backend
├── frontend/         # React frontend
├── image_root/       # Image storage
├── Dockerfile        # Backend build
├── compose.yml       # Docker Compose config
└── API.md           # API documentation
```

## Troubleshooting

**Backend keeps restarting:**
- Check logs: `docker compose logs image-compare-srv`
- Verify route syntax uses `{param}` not `:param` (Axum 0.8)

**CORS errors:**
- Verify CORS is configured in `src/main.rs`
- Check backend is running: `docker compose ps`

**Route returns 404:**
- Rebuild backend: `docker compose build image-compare-srv`
- Restart: `docker compose restart image-compare-srv`

## License

See [LICENSE](LICENSE) file.
