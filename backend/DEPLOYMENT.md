# Azure Container Apps Deployment Guide

This guide explains how to deploy the Apronymer application to Azure Container Apps.

## Prerequisites

1. **Azure CLI**: Install and login to Azure CLI
   ```bash
   az login
   ```

2. **Azure Subscription**: Ensure you have an active Azure subscription

3. **Docker**: Make sure Docker is installed and running locally (for testing)

## Deployment Options

### Option 1: Automated Deployment (Recommended)

Use the provided deployment script:

```bash
cd backend
./deploy-to-aca.sh
```

This script will:
- Create a resource group
- Create an Azure Container Registry (ACR)
- Build and push both backend and frontend images
- Create a Container Apps Environment
- Deploy both applications

### Option 2: Manual Deployment

#### Step 1: Create Resources

```bash
# Set variables
RESOURCE_GROUP="rg-apronymer"
LOCATION="eastus"
CONTAINER_REGISTRY_NAME="crapronymer"

# Create resource group
az group create --name $RESOURCE_GROUP --location $LOCATION

# Create Container Registry
az acr create --resource-group $RESOURCE_GROUP --name $CONTAINER_REGISTRY_NAME --sku Basic --admin-enabled true
```

#### Step 2: Build and Push Images

```bash
# Get ACR login server
ACR_LOGIN_SERVER=$(az acr show --name $CONTAINER_REGISTRY_NAME --resource-group $RESOURCE_GROUP --query loginServer --output tsv)

# Build backend
cd backend
az acr build --registry $CONTAINER_REGISTRY_NAME --image apronymer-backend:latest .

# Build frontend
cd ../frontend
az acr build --registry $CONTAINER_REGISTRY_NAME --image apronymer-frontend:latest .
```

#### Step 3: Deploy Container Apps

```bash
# Create environment
az containerapp env create --name apronymer-env --resource-group $RESOURCE_GROUP --location $LOCATION

# Deploy backend
az containerapp create \
  --name apronymer-backend \
  --resource-group $RESOURCE_GROUP \
  --environment apronymer-env \
  --image $ACR_LOGIN_SERVER/apronymer-backend:latest \
  --registry-server $ACR_LOGIN_SERVER \
  --cpu 0.25 --memory 0.5Gi \
  --min-replicas 0 --max-replicas 3 \
  --target-port 3000 --ingress external

# Deploy frontend
az containerapp create \
  --name apronymer-frontend \
  --resource-group $RESOURCE_GROUP \
  --environment apronymer-env \
  --image $ACR_LOGIN_SERVER/apronymer-frontend:latest \
  --registry-server $ACR_LOGIN_SERVER \
  --cpu 0.25 --memory 0.5Gi \
  --min-replicas 0 --max-replicas 3 \
  --target-port 80 --ingress external
```

## Local Testing with Docker

Before deploying to Azure, test locally:

```bash
# From the backend directory
docker-compose up
```

This will start both services locally:
- Frontend: http://localhost
- Backend: http://localhost:3000

## Architecture

### Backend Container
- **Base Image**: debian:bookworm-slim
- **Runtime**: Rust application
- **Port**: 3000
- **Health Check**: `/hello` endpoint
- **Features**:
  - Multi-stage build for optimized size
  - Non-root user for security
  - Comprehensive health checks

### Frontend Container
- **Base Image**: nginx:alpine
- **Content**: React SPA built with Vite
- **Port**: 80
- **Features**:
  - Optimized nginx configuration
  - Gzip compression
  - Security headers
  - SPA routing support
  - Static asset caching

## Configuration

### Environment Variables

#### Backend
- `HOST=0.0.0.0`
- `PORT=3000`
- `RUST_LOG=info`

#### Frontend
- Nginx configuration handles routing and proxying

### Scaling
- **Auto-scaling**: 0-3 replicas based on demand
- **Resources**: 0.25 CPU, 0.5GB memory per container
- **Cost-effective**: Scales to zero when not in use

## Security Features

1. **Container Security**:
   - Non-root users
   - Minimal base images
   - Health checks

2. **Web Security**:
   - Security headers (X-Frame-Options, etc.)
   - HTTPS termination at ACA level
   - Content Security Policy

3. **Network Security**:
   - Internal communication between containers
   - External ingress only where needed

## Monitoring

Azure Container Apps provides built-in monitoring:
- Application logs
- Metrics and performance
- Health status
- Auto-scaling events

Access via Azure Portal → Container Apps → your app → Monitoring

## Troubleshooting

### Common Issues

1. **Build Failures**:
   ```bash
   # Check ACR build logs
   az acr task logs --registry $CONTAINER_REGISTRY_NAME
   ```

2. **Deployment Issues**:
   ```bash
   # Check container app logs
   az containerapp logs show --name apronymer-backend --resource-group $RESOURCE_GROUP
   ```

3. **Networking**:
   - Ensure both apps are in the same Container Apps Environment
   - Check ingress settings

### Cleanup

To remove all resources:
```bash
az group delete --name rg-apronymer --yes
```

## Cost Optimization

- **Scale to Zero**: Enabled for cost savings
- **Right-sizing**: 0.25 CPU / 0.5GB memory
- **Shared Environment**: Both apps use same environment
- **Basic ACR**: Sufficient for this use case

## Updates and CI/CD

For production deployments, consider:
1. GitHub Actions integration
2. Staged deployments (dev/staging/prod)
3. Blue-green deployments
4. Automated testing
