#!/bin/bash

# Azure Container Apps Deployment Script
# This script deploys the apronymer application to Azure Container Apps

set -e

# Configuration
RESOURCE_GROUP="rg-apronymer"
LOCATION="eastus"
ENVIRONMENT_NAME="apronymer-env"
BACKEND_APP_NAME="apronymer-backend"
FRONTEND_APP_NAME="apronymer-frontend"
CONTAINER_REGISTRY_NAME="crapronymer"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}🚀 Deploying Apronymer to Azure Container Apps${NC}"

# Create resource group
echo -e "${YELLOW}📦 Creating resource group...${NC}"
az group create \
  --name $RESOURCE_GROUP \
  --location $LOCATION

# Create Container Registry
echo -e "${YELLOW}🏗️  Creating Azure Container Registry...${NC}"
az acr create \
  --resource-group $RESOURCE_GROUP \
  --name $CONTAINER_REGISTRY_NAME \
  --sku Basic \
  --admin-enabled true

# Get ACR login server
ACR_LOGIN_SERVER=$(az acr show --name $CONTAINER_REGISTRY_NAME --resource-group $RESOURCE_GROUP --query loginServer --output tsv)

# Build and push backend image
echo -e "${YELLOW}🔨 Building and pushing backend image...${NC}"
cd backend
az acr build \
  --registry $CONTAINER_REGISTRY_NAME \
  --image apronymer-backend:latest \
  .
cd ..

# Build and push frontend image
echo -e "${YELLOW}🔨 Building and pushing frontend image...${NC}"
cd frontend
az acr build \
  --registry $CONTAINER_REGISTRY_NAME \
  --image apronymer-frontend:latest \
  .
cd ..

# Create Container Apps Environment
echo -e "${YELLOW}🌍 Creating Container Apps Environment...${NC}"
az containerapp env create \
  --name $ENVIRONMENT_NAME \
  --resource-group $RESOURCE_GROUP \
  --location $LOCATION

# Deploy Backend Container App
echo -e "${YELLOW}🚀 Deploying backend container app...${NC}"
az containerapp create \
  --name $BACKEND_APP_NAME \
  --resource-group $RESOURCE_GROUP \
  --environment $ENVIRONMENT_NAME \
  --image $ACR_LOGIN_SERVER/apronymer-backend:latest \
  --registry-server $ACR_LOGIN_SERVER \
  --cpu 0.25 \
  --memory 0.5Gi \
  --min-replicas 0 \
  --max-replicas 3 \
  --target-port 3000 \
  --ingress external \
  --env-vars HOST=0.0.0.0 PORT=3000 RUST_LOG=info

# Get backend URL
BACKEND_URL=$(az containerapp show --name $BACKEND_APP_NAME --resource-group $RESOURCE_GROUP --query properties.configuration.ingress.fqdn --output tsv)

# Deploy Frontend Container App
echo -e "${YELLOW}🚀 Deploying frontend container app...${NC}"
az containerapp create \
  --name $FRONTEND_APP_NAME \
  --resource-group $RESOURCE_GROUP \
  --environment $ENVIRONMENT_NAME \
  --image $ACR_LOGIN_SERVER/apronymer-frontend:latest \
  --registry-server $ACR_LOGIN_SERVER \
  --cpu 0.25 \
  --memory 0.5Gi \
  --min-replicas 0 \
  --max-replicas 3 \
  --target-port 80 \
  --ingress external

# Get frontend URL
FRONTEND_URL=$(az containerapp show --name $FRONTEND_APP_NAME --resource-group $RESOURCE_GROUP --query properties.configuration.ingress.fqdn --output tsv)

echo -e "${GREEN}✅ Deployment completed successfully!${NC}"
echo -e "${GREEN}🌐 Frontend URL: https://$FRONTEND_URL${NC}"
echo -e "${GREEN}🔗 Backend URL: https://$BACKEND_URL${NC}"
echo -e "${YELLOW}📝 Note: You may need to update the frontend to point to the backend URL${NC}"
