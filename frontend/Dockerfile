# Multi-stage build for React frontend
# Stage 1: Build the application
FROM node:20-alpine as builder

# Set working directory
WORKDIR /app

# Copy package files first for better caching
COPY package*.json ./

# Install dependencies
RUN npm ci

# Copy source code
COPY . .

# Accept build argument for API key
ARG VITE_API_KEY
ENV VITE_API_KEY=$VITE_API_KEY

# Build the application (Vite will embed VITE_API_KEY into the bundle)
RUN npm run build

# Stage 2: Serve with Nginx
FROM nginx:alpine

COPY nginx-aca.conf /etc/nginx/nginx.conf
COPY --from=builder /app/dist /usr/share/nginx/html
EXPOSE 80
USER nginx

# Switch to nginx user for security
USER nginx

# Expose port 8080 (non-privileged port)
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD wget --no-verbose --tries=1 --spider http://localhost:8080/ || exit 1

# Start nginx
CMD ["nginx", "-g", "daemon off;"]
