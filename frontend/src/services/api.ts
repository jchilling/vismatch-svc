import axios, { AxiosError } from 'axios';
import type {
  CompareImageReq,
  CompareImageResp,
  UploadImageReq,
  UploadImageResp,
  DeleteProjectReq,
  DeleteProjectResp,
  ApiError,
} from '../types/api';

// Support both build-time and runtime configuration
// For Docker deployments, use window.ENV_API_URL if available
const getApiBaseUrl = (): string => {
  // Runtime configuration (for Docker)
  if (typeof window !== 'undefined' && (window as any).ENV_API_URL) {
    return (window as any).ENV_API_URL;
  }
  // Build-time configuration (for Vite)
  return import.meta.env.VITE_API_URL || 'http://localhost:3000';
};

const API_BASE_URL = getApiBaseUrl();

const apiClient = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Helper to convert File to base64
export const fileToBase64 = (file: File): Promise<string> => {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = () => {
      const result = reader.result as string;
      // Remove data URL prefix if present
      const base64 = result.includes(',') ? result.split(',')[1] : result;
      resolve(base64);
    };
    reader.onerror = reject;
    reader.readAsDataURL(file);
  });
};

// Helper to convert image file to base64 data URL
export const fileToDataURL = (file: File): Promise<string> => {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = () => resolve(reader.result as string);
    reader.onerror = reject;
    reader.readAsDataURL(file);
  });
};

export const api = {
  /**
   * Compare an image against a project's database
   */
  async compareImage(req: CompareImageReq): Promise<CompareImageResp> {
    try {
      const response = await apiClient.post<CompareImageResp>('/diff', req);
      return response.data;
    } catch (error) {
      if (axios.isAxiosError(error)) {
        const axiosError = error as AxiosError<ApiError>;
        if (axiosError.response?.data) {
          throw new Error(axiosError.response.data.message);
        }
        throw new Error(axiosError.message || 'Failed to compare image');
      }
      throw error;
    }
  },

  /**
   * Upload an image to a project
   */
  async uploadImage(req: UploadImageReq): Promise<UploadImageResp> {
    try {
      const response = await apiClient.post<UploadImageResp>('/upload', req);
      return response.data;
    } catch (error) {
      if (axios.isAxiosError(error)) {
        const axiosError = error as AxiosError<ApiError>;
        if (axiosError.response?.data) {
          throw new Error(axiosError.response.data.message);
        }
        throw new Error(axiosError.message || 'Failed to upload image');
      }
      throw error;
    }
  },

  /**
   * Delete a project and all its images
   */
  async deleteProject(req: DeleteProjectReq): Promise<DeleteProjectResp> {
    try {
      // URL encode the project name to handle special characters
      const encodedProjectName = encodeURIComponent(req.project_name);
      const url = `/project/${encodedProjectName}`;
      console.log('[API] Deleting project:', url, req);
      const response = await apiClient.delete<DeleteProjectResp>(url);
      console.log('[API] Delete response:', response.data);
      return response.data;
    } catch (error) {
      console.error('[API] Delete project error:', error);
      if (axios.isAxiosError(error)) {
        const axiosError = error as AxiosError<ApiError>;
        console.error('[API] Axios error details:', {
          status: axiosError.response?.status,
          statusText: axiosError.response?.statusText,
          data: axiosError.response?.data,
          message: axiosError.message,
        });
        if (axiosError.response?.data) {
          throw new Error(axiosError.response.data.message);
        }
        throw new Error(axiosError.message || 'Failed to delete project');
      }
      throw error;
    }
  },
};

