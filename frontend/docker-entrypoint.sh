#!/bin/sh
# Docker entrypoint script to inject environment variables at runtime

# Create env-config.js with runtime API URL
cat > /usr/share/nginx/html/env-config.js <<EOF
window.ENV_API_URL = '${VITE_API_URL:-http://localhost:3000}';
EOF

# Start nginx
exec nginx -g 'daemon off;'

