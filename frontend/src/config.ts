// Environment configuration for different deployment scenarios
const getApiBaseUrl = (): string => {
  // In production (Azure Container Apps), use the backend URL
  if (import.meta.env.PROD) {
    return import.meta.env.VITE_API_BASE_URL || 'https://apronymer-backend.agreeablemushroom-284a0b84.westeurope.azurecontainerapps.io';
  }
  
  // In development, use local backend
  return 'http://localhost:3000';
};

export const API_BASE_URL = getApiBaseUrl();
export const API_KEY = import.meta.env.PROD ? 'f806867ecacc6edc2b240fa45e43b0ee6f5541d85c84bdb1efd7e4efed129f09' : 'test-api-key';
