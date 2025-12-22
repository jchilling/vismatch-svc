# Vismatch Frontend

相似影像辨識服務前端介面

A modern React + TypeScript frontend for the image similarity matching service.

## Features

- **Image Comparison**: Upload an image and find similar images from a project database
- **Image Upload**: Add new images to project databases
- **Professional UI**: Clean, modern interface suitable for government/internal use
- **Responsive Design**: Works on desktop and mobile devices

## Development

### Prerequisites

- Node.js 18+ and npm

### Setup

1. Install dependencies:
```bash
npm install
```

2. Create a `.env` file (copy from `.env.example`):
```bash
cp .env.example .env
```

3. Update `.env` with your API URL:
```
VITE_API_URL=http://localhost:3000
```

4. Start development server:
```bash
npm run dev
```

The frontend will be available at `http://localhost:5173`

## Building for Production

```bash
npm run build
```

The production build will be in the `dist/` directory.

## Deployment

For production deployment on your domain (域名):

1. Update `.env` with your production API URL:
```
VITE_API_URL=https://your-api-domain.com
```

2. Build the project:
```bash
npm run build
```

3. Serve the `dist/` directory using a web server (nginx, Apache, etc.)

## Project Structure

```
src/
├── components/       # React components
│   ├── ImageCompare.tsx
│   └── ImageUpload.tsx
├── services/         # API client
│   └── api.ts
├── types/           # TypeScript types
│   └── api.ts
├── App.tsx          # Main app component
└── main.tsx         # Entry point
```
