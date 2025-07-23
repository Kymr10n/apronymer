#!/bin/bash

# Azure Container Apps Deployment Script
# This script deploys the apronymer application to Azure Container Apps
# Usage: ./deploy-to-aca.sh [--clean] [--frontend] [--backend] [--both]
#   --clean: Delete existing container apps before deployment for a fresh start
#   --frontend: Deploy only the frontend component
#   --backend: Deploy only the backend component  
#   --both: Deploy both frontend and backend (default)

# Enable error handling
trap 'echo -e "${RED}❌ Deployment failed at line $LINENO${NC}"; exit 1' ERR

# Parse command line arguments
CLEAN_DEPLOYMENT=false
DEPLOY_FRONTEND=false
DEPLOY_BACKEND=false
DEPLOY_BOTH=true

for arg in "$@"; do
  case $arg in
    --clean)
      CLEAN_DEPLOYMENT=true
      shift
      ;;
    --frontend)
      DEPLOY_FRONTEND=true
      DEPLOY_BOTH=false
      shift
      ;;
    --backend)
      DEPLOY_BACKEND=true
      DEPLOY_BOTH=false
      shift
      ;;
    --both)
      DEPLOY_BOTH=true
      DEPLOY_FRONTEND=false
      DEPLOY_BACKEND=false
      shift
      ;;
    --help|-h)
      echo "Usage: $0 [--clean] [--frontend] [--backend] [--both]"
      echo "  --clean: Delete existing container apps before deployment"
      echo "  --frontend: Deploy only the frontend component"
      echo "  --backend: Deploy only the backend component"
      echo "  --both: Deploy both frontend and backend (default)"
      exit 0
      ;;
    *)
      echo "Unknown argument: $arg"
      echo "Use --help for usage information"
      exit 1
      ;;
  esac
done

# Configuration
RESOURCE_GROUP="rg-apronymer"
LOCATION="westeurope"
ENVIRONMENT_NAME="apronymer-env"
BACKEND_APP_NAME="apronymer-backend"
FRONTEND_APP_NAME="apronymer-frontend"
CONTAINER_REGISTRY_NAME="crapronymer"
# Generate or use existing API key
API_KEY="${API_KEY:-f806867ecacc6edc2b240fa45e43b0ee6f5541d85c84bdb1efd7e4efed129f09}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}🚀 Deploying Apronymer to Azure Container Apps${NC}"

# Check if Azure CLI is logged in
if ! az account show &>/dev/null; then
  echo -e "${RED}❌ You are not logged in to Azure CLI. Please run 'az login' first.${NC}"
  exit 1
fi

echo -e "${GREEN}✅ Azure CLI authentication verified${NC}"

# Determine what components to deploy
if [[ "$DEPLOY_BOTH" == "true" ]]; then
  DEPLOY_FRONTEND=true
  DEPLOY_BACKEND=true
  echo -e "${GREEN}📦 Deploying both frontend and backend${NC}"
elif [[ "$DEPLOY_FRONTEND" == "true" && "$DEPLOY_BACKEND" == "true" ]]; then
  DEPLOY_BOTH=true
  echo -e "${GREEN}📦 Deploying both frontend and backend${NC}"
elif [[ "$DEPLOY_FRONTEND" == "true" ]]; then
  echo -e "${GREEN}🎨 Deploying frontend only${NC}"
elif [[ "$DEPLOY_BACKEND" == "true" ]]; then
  echo -e "${GREEN}⚙️  Deploying backend only${NC}"
else
  echo -e "${RED}❌ No deployment target specified. Use --frontend, --backend, or --both${NC}"
  exit 1
fi

# Function to check if resource exists
resource_exists() {
  local resource_type=$1
  local resource_name=$2
  local resource_group=$3
  
  case $resource_type in
    "group")
      az group show --name $resource_name &>/dev/null
      ;;
    "acr")
      az acr show --name $resource_name --resource-group $resource_group &>/dev/null
      ;;
    "containerapp-env")
      az containerapp env show --name $resource_name --resource-group $resource_group &>/dev/null
      ;;
    "containerapp")
      az containerapp show --name $resource_name --resource-group $resource_group &>/dev/null
      ;;
  esac
}

# Function to force a new revision by updating with a timestamp environment variable
force_new_revision() {
  local app_name=$1
  local resource_group=$2
  local timestamp=$(date +%s)
  
  echo -e "${YELLOW}🔄 Forcing new revision for $app_name with timestamp $timestamp...${NC}"
  az containerapp update \
    --name $app_name \
    --resource-group $resource_group \
    --set-env-vars DEPLOY_TIMESTAMP="$timestamp" \
    --output none
}

# Function to verify new revision was created
verify_new_revision() {
  local app_name=$1
  local resource_group=$2
  
  echo -e "${YELLOW}🔍 Verifying new revision for $app_name...${NC}"
  
  # Get the latest revision info with a simpler query
  local latest_revision=$(az containerapp revision list \
    --name $app_name \
    --resource-group $resource_group \
    --query "[?properties.active].name | [0]" \
    --output tsv)
  
  local revision_time=$(az containerapp revision list \
    --name $app_name \
    --resource-group $resource_group \
    --query "[?properties.active].properties.createdTime | [0]" \
    --output tsv)
  
  echo -e "${GREEN}✅ Active revision: $latest_revision (created: $revision_time)${NC}"
}

# Function to clean up existing container apps
cleanup_container_apps() {
  local cleanup_frontend=$1
  local cleanup_backend=$2
  
  echo -e "${YELLOW}🧹 Cleaning up existing container apps...${NC}"
  
  # Delete backend container app if requested and exists
  if [[ "$cleanup_backend" == "true" ]]; then
    if resource_exists "containerapp" $BACKEND_APP_NAME $RESOURCE_GROUP; then
      echo -e "${YELLOW}🗑️  Deleting existing backend container app...${NC}"
      az containerapp delete --name $BACKEND_APP_NAME --resource-group $RESOURCE_GROUP --yes &>/dev/null
      echo -e "${GREEN}✅ Backend container app deleted${NC}"
    else
      echo -e "${YELLOW}ℹ️  Backend container app does not exist, skipping deletion${NC}"
    fi
  else
    echo -e "${YELLOW}ℹ️  Skipping backend cleanup (not requested)${NC}"
  fi
  
  # Delete frontend container app if requested and exists
  if [[ "$cleanup_frontend" == "true" ]]; then
    if resource_exists "containerapp" $FRONTEND_APP_NAME $RESOURCE_GROUP; then
      echo -e "${YELLOW}🗑️  Deleting existing frontend container app...${NC}"
      az containerapp delete --name $FRONTEND_APP_NAME --resource-group $RESOURCE_GROUP --yes &>/dev/null
      echo -e "${GREEN}✅ Frontend container app deleted${NC}"
    else
      echo -e "${YELLOW}ℹ️  Frontend container app does not exist, skipping deletion${NC}"
    fi
  else
    echo -e "${YELLOW}ℹ️  Skipping frontend cleanup (not requested)${NC}"
  fi
  
  # Wait a moment for cleanup to complete
  if [[ "$cleanup_frontend" == "true" || "$cleanup_backend" == "true" ]]; then
    echo -e "${YELLOW}⏳ Waiting for cleanup to complete...${NC}"
    sleep 10
    echo -e "${GREEN}✅ Cleanup completed${NC}"
  fi
}

# Create or check resource group
if resource_exists "group" $RESOURCE_GROUP; then
  echo -e "${GREEN}✅ Resource group '$RESOURCE_GROUP' already exists${NC}"
else
  echo -e "${YELLOW}📦 Creating resource group...${NC}"
  az group create \
    --name $RESOURCE_GROUP \
    --location $LOCATION
fi

# Create or check Container Registry
if resource_exists "acr" $CONTAINER_REGISTRY_NAME $RESOURCE_GROUP; then
  echo -e "${GREEN}✅ Container Registry '$CONTAINER_REGISTRY_NAME' already exists${NC}"
else
  echo -e "${YELLOW}🏗️  Creating Azure Container Registry...${NC}"
  az acr create \
    --resource-group $RESOURCE_GROUP \
    --name $CONTAINER_REGISTRY_NAME \
    --sku Basic \
    --admin-enabled true
fi

# Get ACR login server
ACR_LOGIN_SERVER=$(az acr show --name $CONTAINER_REGISTRY_NAME --resource-group $RESOURCE_GROUP --query loginServer --output tsv)

# Ensure we're logged into the container registry
echo -e "${YELLOW}🔐 Logging into container registry...${NC}"
az acr login --name $CONTAINER_REGISTRY_NAME

# Get the absolute path to the project root
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BACKEND_DIR="$PROJECT_ROOT/backend"
FRONTEND_DIR="$PROJECT_ROOT/frontend"

echo -e "${GREEN}✅ Project structure detected:${NC}"
echo -e "  Backend: $BACKEND_DIR"
echo -e "  Frontend: $FRONTEND_DIR"

# Verify directories exist
if [[ ! -d "$BACKEND_DIR" ]]; then
  echo -e "${RED}❌ Backend directory not found: $BACKEND_DIR${NC}"
  exit 1
fi

if [[ ! -d "$FRONTEND_DIR" ]]; then
  echo -e "${RED}❌ Frontend directory not found: $FRONTEND_DIR${NC}"
  exit 1
fi

if [[ ! -f "$BACKEND_DIR/Dockerfile" ]]; then
  echo -e "${RED}❌ Backend Dockerfile not found: $BACKEND_DIR/Dockerfile${NC}"
  exit 1
fi

if [[ ! -f "$FRONTEND_DIR/Dockerfile" ]]; then
  echo -e "${RED}❌ Frontend Dockerfile not found: $FRONTEND_DIR/Dockerfile${NC}"
  exit 1
fi

# Build and push backend image
if [[ "$DEPLOY_BACKEND" == "true" ]]; then
  echo -e "${YELLOW}🔨 Building and pushing backend image...${NC}"
  cd "$BACKEND_DIR"
  az acr build \
    --registry $CONTAINER_REGISTRY_NAME \
    --image apronymer-backend:latest \
    .
fi

# Build and push frontend image  
if [[ "$DEPLOY_FRONTEND" == "true" ]]; then
  echo -e "${YELLOW}🔨 Building and pushing frontend image...${NC}"
  cd "$FRONTEND_DIR"
  az acr build \
    --registry $CONTAINER_REGISTRY_NAME \
    --image apronymer-frontend:latest \
    .
fi

# Return to backend directory
cd "$BACKEND_DIR"

# Clean up existing container apps if requested
if [[ "$CLEAN_DEPLOYMENT" == "true" ]]; then
  cleanup_container_apps "$DEPLOY_FRONTEND" "$DEPLOY_BACKEND"
fi

# Create or check Container Apps Environment
if resource_exists "containerapp-env" $ENVIRONMENT_NAME $RESOURCE_GROUP; then
  echo -e "${GREEN}✅ Container Apps Environment '$ENVIRONMENT_NAME' already exists${NC}"
else
  echo -e "${YELLOW}🌍 Creating Container Apps Environment...${NC}"
  az containerapp env create \
    --name $ENVIRONMENT_NAME \
    --resource-group $RESOURCE_GROUP \
    --location $LOCATION
fi

# Deploy or update Backend Container App
if [[ "$DEPLOY_BACKEND" == "true" ]]; then
  if resource_exists "containerapp" $BACKEND_APP_NAME $RESOURCE_GROUP; then
    echo -e "${YELLOW}🔄 Updating backend container app...${NC}"
    az containerapp update \
      --name $BACKEND_APP_NAME \
      --resource-group $RESOURCE_GROUP \
      --image $ACR_LOGIN_SERVER/apronymer-backend:latest \
      --cpu 0.5 \
      --memory 1Gi
    
    # Force a new revision to ensure the latest image is used
    force_new_revision $BACKEND_APP_NAME $RESOURCE_GROUP
  else
    echo -e "${YELLOW}🚀 Deploying backend container app with enhanced resources...${NC}"
    az containerapp create \
      --name $BACKEND_APP_NAME \
      --resource-group $RESOURCE_GROUP \
      --environment $ENVIRONMENT_NAME \
      --image $ACR_LOGIN_SERVER/apronymer-backend:latest \
      --registry-server $ACR_LOGIN_SERVER \
      --cpu 0.5 \
      --memory 1Gi \
      --min-replicas 1 \
      --max-replicas 3 \
      --target-port 3000 \
      --ingress external \
      --env-vars HOST=0.0.0.0 PORT=3000 RUST_LOG=debug API_KEY="${API_KEY:-missing}" DEPLOY_TIMESTAMP="$(date +%s)"
  fi
fi

# Get backend URL
if [[ "$DEPLOY_BACKEND" == "true" ]]; then
  BACKEND_URL=$(az containerapp show --name $BACKEND_APP_NAME --resource-group $RESOURCE_GROUP --query properties.configuration.ingress.fqdn --output tsv)
fi

# Deploy or update Frontend Container App
if [[ "$DEPLOY_FRONTEND" == "true" ]]; then
  if resource_exists "containerapp" $FRONTEND_APP_NAME $RESOURCE_GROUP; then
    echo -e "${YELLOW}🔄 Updating frontend container app...${NC}"
    az containerapp update \
      --name $FRONTEND_APP_NAME \
      --resource-group $RESOURCE_GROUP \
      --image $ACR_LOGIN_SERVER/apronymer-frontend:latest
    
    # Force a new revision to ensure the latest image is used
    force_new_revision $FRONTEND_APP_NAME $RESOURCE_GROUP
  else
    echo -e "${YELLOW}🚀 Deploying frontend container app...${NC}"
    az containerapp create \
      --name $FRONTEND_APP_NAME \
      --resource-group $RESOURCE_GROUP \
      --environment $ENVIRONMENT_NAME \
      --image $ACR_LOGIN_SERVER/apronymer-frontend:latest \
      --registry-server $ACR_LOGIN_SERVER \
      --cpu 0.25 \
      --memory 0.5Gi \
      --min-replicas 1 \
      --max-replicas 3 \
      --target-port 8080 \
      --ingress external \
      --env-vars DEPLOY_TIMESTAMP="$(date +%s)"
  fi
fi

# Get frontend URL
if [[ "$DEPLOY_FRONTEND" == "true" ]]; then
  FRONTEND_URL=$(az containerapp show --name $FRONTEND_APP_NAME --resource-group $RESOURCE_GROUP --query properties.configuration.ingress.fqdn --output tsv)
fi

echo -e "${GREEN}✅ Deployment completed successfully!${NC}"

# Verify new revisions were created
echo -e "${YELLOW}🔍 Verifying deployments...${NC}"
if [[ "$DEPLOY_BACKEND" == "true" ]]; then
  verify_new_revision $BACKEND_APP_NAME $RESOURCE_GROUP
fi
if [[ "$DEPLOY_FRONTEND" == "true" ]]; then
  verify_new_revision $FRONTEND_APP_NAME $RESOURCE_GROUP
fi

# Display URLs only for deployed components
if [[ "$DEPLOY_FRONTEND" == "true" ]]; then
  echo -e "${GREEN}🌐 Frontend URL: https://$FRONTEND_URL${NC}"
fi

if [[ "$DEPLOY_BACKEND" == "true" ]]; then
  echo -e "${GREEN}🔗 Backend URL: https://$BACKEND_URL${NC}"
  echo -e "${GREEN}🔑 API Key: $API_KEY${NC}"
fi

echo -e "${YELLOW}📝 Note: Frontend is configured to use the backend API${NC}"
echo -e "${YELLOW}💡 To redeploy: Just run this script again - it will update existing resources${NC}"

# Display deployment summary
if [[ "$DEPLOY_BOTH" == "true" ]]; then
  echo -e "${GREEN}📦 Both frontend and backend deployed successfully${NC}"
elif [[ "$DEPLOY_FRONTEND" == "true" ]]; then
  echo -e "${GREEN}🎨 Frontend deployed successfully${NC}"
elif [[ "$DEPLOY_BACKEND" == "true" ]]; then
  echo -e "${GREEN}⚙️  Backend deployed successfully${NC}"
fi
