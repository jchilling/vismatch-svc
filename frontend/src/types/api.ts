// API types matching the backend Rust structures

export interface SimilarImageEntry {
  image_name: string;
  distance: number;
  data?: string; // base64 image data
}

export interface CompareImageReq {
  project_name: string;
  data: string; // base64 image data
  with_image: boolean;
}

export interface CompareImageResp {
  success: boolean;
  message: string;
  project_name: string;
  compare_result: SimilarImageEntry[];
}

export interface UploadImageReq {
  project_name: string;
  image_name: string;
  data: string; // base64 image data
}

export interface UploadImageResp {
  success: boolean;
  message: string;
  token: string;
}

export interface ApiError {
  message: string;
}

export interface DeleteProjectReq {
  project_name: string;
}

export interface DeleteProjectResp {
  success: boolean;
  message: string;
}

