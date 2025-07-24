#!/bin/bash

# Colors for output
RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; NC='\033[0m'
trap 'echo -e "${RED}❌ Deployment failed at line $LINENO${NC}"; exit 1' ERR

# Azure Container Apps Deployment Script
# Usage: ./deploy-to-aca.sh [--clean] [--frontend] [--backend] [--both]

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
readonly RESOURCE_GROUP="rg-apronymer"
readonly LOCATION="westeurope"
readonly ENVIRONMENT_NAME="apronymer-env"
readonly BACKEND_APP_NAME="apronymer-backend"
readonly FRONTEND_APP_NAME="apronymer-frontend"
readonly CONTAINER_REGISTRY_NAME="crapronymer"
readonly ACR_LOGIN_SERVER="${CONTAINER_REGISTRY_NAME}.azurecr.io"
readonly IMAGE_TAG="latest"
readonly REV_SUFFIX="$(date +%s)"

# Application configuration
readonly BACKEND_HOST="0.0.0.0"
readonly BACKEND_PORT="3000"
readonly FRONTEND_PORT="8080"

# Load .env if present
if [ -f .env ]; then
  set -o allexport
  source .env
  set +o allexport
fi

# Require API_KEY
if [ -z "$API_KEY" ]; then
  echo -e "${RED}❌ API_KEY is not set. Please set it in your environment or in a .env file.${NC}"
  exit 1
fi

# Ensure Azure CLI login
if ! az account show --query id -o tsv &>/dev/null; then
  echo -e "${RED}❌ You are not logged in to Azure CLI. Please run 'az login' first.${NC}"
  exit 1
fi

echo -e "${GREEN}✅ Azure CLI authentication verified${NC}"

# Determine components to deploy
if [[ "$DEPLOY_BOTH" == "true" ]]; then
  DEPLOY_FRONTEND=true
  DEPLOY_BACKEND=true
  echo -e "${GREEN}📦 Deploying both frontend and backend${NC}"
elif [[ "$DEPLOY_FRONTEND" == "true" ]]; then
  echo -e "${GREEN}🎨 Deploying frontend only${NC}"
elif [[ "$DEPLOY_BACKEND" == "true" ]]; then
  echo -e "${GREEN}⚙️  Deploying backend only${NC}"
else
  echo -e "${RED}❌ No deployment target specified. Use --frontend, --backend, or --both${NC}"
  exit 1
fi

# Set backend and frontend directories relative to script location
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BACKEND_DIR="$SCRIPT_DIR"
FRONTEND_DIR="$SCRIPT_DIR/../frontend"

# Helper function: check if a resource exists
resource_exists() {
  local type=$1
  local name=$2
  local group=$3
  az $type show --name "$name" --resource-group "$group" &>/dev/null
}

# Helper function: clean inactive revisions
cleanup_old_revisions() {
  local app=$1
  echo -e "${YELLOW}🧽 Cleaning up old revisions for $app...${NC}"
  az containerapp revision list \
    --name "$app" --resource-group "$RESOURCE_GROUP" \
    --query "[?active==\`false\`].name" -o tsv |
  while read rev; do
    echo -e "${YELLOW}🗑 Deleting old revision: $rev${NC}"
    az containerapp revision delete \
      --name "$app" --resource-group "$RESOURCE_GROUP" \
      --revision "$rev" >/dev/null
  done
}

# Build images
if [[ "$DEPLOY_BACKEND" == "true" ]]; then
  echo -e "${YELLOW}🔨 Building backend image...${NC}"
  pushd "$BACKEND_DIR" > /dev/null
  az acr build --registry "$CONTAINER_REGISTRY_NAME" --image apronymer-backend:$IMAGE_TAG . || exit 1
  popd > /dev/null
fi

if [[ "$DEPLOY_FRONTEND" == "true" ]]; then
  echo -e "${YELLOW}🎨 Building frontend image...${NC}"
  pushd "$FRONTEND_DIR" > /dev/null
  az acr build --registry "$CONTAINER_REGISTRY_NAME" --image apronymer-frontend:$IMAGE_TAG . || exit 1
  popd > /dev/null
fi

# Clean up old revisions if requested
if [[ "$CLEAN_DEPLOYMENT" == "true" ]]; then
  echo -e "${YELLOW}🧹 Clean deployment requested${NC}"
  [[ "$DEPLOY_BACKEND" == "true" ]] && cleanup_old_revisions "$BACKEND_APP_NAME"
  [[ "$DEPLOY_FRONTEND" == "true" ]] && cleanup_old_revisions "$FRONTEND_APP_NAME"
fi

# Deploy backend
if [[ "$DEPLOY_BACKEND" == "true" ]]; then
  echo -e "${GREEN}🚀 Deploying backend container app...${NC}"
  if resource_exists "containerapp" "$BACKEND_APP_NAME" "$RESOURCE_GROUP"; then
    az containerapp update \
      --name "$BACKEND_APP_NAME" \
      --resource-group "$RESOURCE_GROUP" \
      --image "$ACR_LOGIN_SERVER/apronymer-backend:$IMAGE_TAG" \
      --revision-suffix "$REV_SUFFIX" \
      --cpu 0.5 --memory 1Gi \
      --set-env-vars HOST="$BACKEND_HOST" PORT="$BACKEND_PORT" RUST_LOG=debug API_KEY="$API_KEY" DEPLOY_TIMESTAMP="$REV_SUFFIX"
  else
    az containerapp create \
      --name "$BACKEND_APP_NAME" \
      --resource-group "$RESOURCE_GROUP" \
      --environment "$ENVIRONMENT_NAME" \
      --image "$ACR_LOGIN_SERVER/apronymer-backend:$IMAGE_TAG" \
      --registry-server "$ACR_LOGIN_SERVER" \
      --cpu 0.5 --memory 1Gi \
      --min-replicas 1 --max-replicas 3 \
      --target-port "$BACKEND_PORT" --ingress external \
      --env-vars HOST="$BACKEND_HOST" PORT="$BACKEND_PORT" RUST_LOG=debug API_KEY="$API_KEY" DEPLOY_TIMESTAMP="$REV_SUFFIX"
  fi
fi

# Deploy frontend
if [[ "$DEPLOY_FRONTEND" == "true" ]]; then
  echo -e "${GREEN}🚀 Deploying frontend container app...${NC}"
  if resource_exists "containerapp" "$FRONTEND_APP_NAME" "$RESOURCE_GROUP"; then
    az containerapp update \
      --name "$FRONTEND_APP_NAME" \
      --resource-group "$RESOURCE_GROUP" \
      --image "$ACR_LOGIN_SERVER/apronymer-frontend:$IMAGE_TAG" \
      --revision-suffix "$REV_SUFFIX" \
      --set-env-vars VITE_API_KEY="$API_KEY" DEPLOY_TIMESTAMP="$REV_SUFFIX"
  else
    az containerapp create \
      --name "$FRONTEND_APP_NAME" \
      --resource-group "$RESOURCE_GROUP" \
      --environment "$ENVIRONMENT_NAME" \
      --image "$ACR_LOGIN_SERVER/apronymer-frontend:$IMAGE_TAG" \
      --registry-server "$ACR_LOGIN_SERVER" \
      --cpu 0.25 --memory 0.5Gi \
      --min-replicas 1 --max-replicas 3 \
      --target-port "$FRONTEND_PORT" --ingress external \
      --env-vars VITE_API_KEY="$API_KEY" DEPLOY_TIMESTAMP="$REV_SUFFIX"
  fi
fi

# Final output
echo -e "${GREEN}✅ Deployment completed. Revisions tagged with: $REV_SUFFIX${NC}"
echo -e "${YELLOW}🔐 API_KEY was used securely and not printed${NC}"

# Print deployed URLs
if [[ "$DEPLOY_BACKEND" == "true" ]]; then
  BACKEND_URL=$(az containerapp show --name "$BACKEND_APP_NAME" --resource-group "$RESOURCE_GROUP" --query "properties.configuration.ingress.fqdn" -o tsv)
  REV_NAME=$(az containerapp show --name "$BACKEND_APP_NAME" --resource-group "$RESOURCE_GROUP" --query "properties.latestRevisionName" -o tsv)
  echo -e "${GREEN}🔗 Backend URL: https://$BACKEND_URL  (rev: $REV_NAME)${NC}"
fi

if [[ "$DEPLOY_FRONTEND" == "true" ]]; then
  FRONTEND_URL=$(az containerapp show --name "$FRONTEND_APP_NAME" --resource-group "$RESOURCE_GROUP" --query "properties.configuration.ingress.fqdn" -o tsv)
  REV_NAME=$(az containerapp show --name "$FRONTEND_APP_NAME" --resource-group "$RESOURCE_GROUP" --query "properties.latestRevisionName" -o tsv)
  echo -e "${GREEN}🌐 Frontend URL: https://$FRONTEND_URL  (rev: $REV_NAME)${NC}"
fi

echo -e "${YELLOW}📎 All apps available via Azure Portal.${NC}"
# End of script