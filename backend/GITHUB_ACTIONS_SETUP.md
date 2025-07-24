# GitHub Actions Deployment Setup

This guide explains how to set up automated deployment to Azure Container Apps using GitHub Actions.

## Prerequisites

1. **Azure Service Principal**: Create a service principal with appropriate permissions
2. **GitHub Secrets**: Configure required secrets in your repository
3. **Azure Resources**: Ensure your Azure Container Apps environment exists

## Step 1: Create Azure Service Principal

```bash
# Create service principal
az ad sp create-for-rbac --name "apronymer-github-actions" \
  --role contributor \
  --scopes /subscriptions/<SUBSCRIPTION_ID>/resourceGroups/rg-apronymer \
  --sdk-auth

# Grant ACR permissions
az role assignment create \
  --assignee <SERVICE_PRINCIPAL_ID> \
  --role AcrPush \
  --scope /subscriptions/<SUBSCRIPTION_ID>/resourceGroups/rg-apronymer/providers/Microsoft.ContainerRegistry/registries/crapronymer
```

## Step 2: Configure GitHub Secrets

Add these secrets to your GitHub repository (Settings → Secrets and variables → Actions):

### Required Secrets:
- `AZURE_CLIENT_ID`: Service principal client ID
- `AZURE_TENANT_ID`: Azure tenant ID  
- `AZURE_SUBSCRIPTION_ID`: Azure subscription ID
- `API_KEY`: Your application's API key

### How to get the values:
```bash
# Get subscription and tenant info
az account show --query '{subscriptionId:id, tenantId:tenantId}' -o table

# Get service principal client ID from the output of the sp create command above
```

## Step 3: Workflow Features

The GitHub Actions workflow (`.github/workflows/deploy.yml`) provides:

### Automatic Deployment:
- **Triggers**: Pushes to `main` branch
- **Manual**: Workflow dispatch with options

### Deployment Options:
- **Target**: Choose backend, frontend, or both
- **Clean**: Option to clean old revisions
- **Environment**: Production deployment with proper authentication

### Security Features:
- **OIDC Authentication**: No service principal secrets stored
- **Scoped Permissions**: Minimal required access
- **Secret Management**: API keys from GitHub secrets

## Step 4: Local vs GitHub Actions Differences

### Authentication:
- **Local**: Uses `az login` (user authentication)
- **GitHub Actions**: Uses `azure/login@v1` (service principal)

### Docker Building:
- **Both**: Build locally and push to ACR (avoids Azure ACR build issues)
- **GitHub Actions**: Uses Docker Buildx for enhanced capabilities

### Environment Detection:
The script automatically detects GitHub Actions environment via `$GITHUB_ACTIONS` variable.

## Step 5: Testing the Setup

1. **Manual Test**: Use workflow dispatch to test deployment
2. **Automatic Test**: Push to main branch
3. **Monitor**: Check Actions tab for deployment status

## Troubleshooting

### Common Issues:

1. **ACR Authentication Failed**:
   - Ensure service principal has `AcrPush` role
   - Verify `AZURE_CLIENT_ID` is correct

2. **Container App Update Failed**:
   - Check service principal has `Contributor` role on resource group
   - Verify Azure resources exist

3. **Docker Build Failed**:
   - Check Dockerfile syntax
   - Verify build context includes required files

### Debug Steps:
```bash
# Test service principal locally
az login --service-principal \
  --username <AZURE_CLIENT_ID> \
  --password <AZURE_CLIENT_SECRET> \
  --tenant <AZURE_TENANT_ID>

# Test ACR access
az acr login --name crapronymer
```

## Security Best Practices

1. **Least Privilege**: Service principal only has required permissions
2. **Secret Rotation**: Rotate API keys and service principal credentials regularly
3. **Branch Protection**: Protect main branch to control deployments
4. **Environment Separation**: Use different service principals for different environments

## Workflow Customization

To modify the workflow:
- **Change triggers**: Edit the `on:` section
- **Add environments**: Use GitHub environments for approval workflows
- **Add notifications**: Integrate with Slack/Teams
- **Add tests**: Run tests before deployment
