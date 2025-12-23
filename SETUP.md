# Setup Guide - Running Without Docker

Since Docker requires admin permissions, here are alternative ways to run the application.

## Option 1: Install Docker Desktop Manually

1. Download Docker Desktop for Mac from: https://www.docker.com/products/docker-desktop/
2. Open the downloaded `.dmg` file
3. Drag Docker.app to Applications
4. Open Docker Desktop from Applications
5. Wait for Docker to start (whale icon in menu bar)
6. Then run:
   ```bash
   docker compose build
   docker compose up -d
   ```

## Option 2: Run Without Docker (Development Mode)

### Prerequisites

You'll need to install Rust:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Restart terminal or run:
source ~/.cargo/env

# Verify installation
cargo --version
```

### Running the Backend

```bash
# From project root
cargo build
cargo run
```

The backend will start on `http://localhost:3000`

### Running the Frontend

```bash
# In a new terminal
cd frontend
npm install
npm run dev
```

The frontend will start on `http://localhost:5173`

## Option 3: Use Docker with Sudo

If you have admin access, you can install Docker via Homebrew with sudo:

```bash
sudo brew install --cask docker
```

Then open Docker Desktop from Applications and start it.

