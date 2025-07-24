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
export const API_KEY = import.meta.env.VITE_API_KEY;
