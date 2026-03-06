import axios from 'axios';
import { getApiBaseUrl } from '@/lib/platform';

const client = axios.create({
  baseURL: '/api/v1',
  headers: { 'Content-Type': 'application/json' },
});

// Bare client for refresh requests — no interceptors to avoid deadlock
const refreshClient = axios.create({
  baseURL: '/api/v1',
  headers: { 'Content-Type': 'application/json' },
});

// Initialize dynamic base URL for Tauri mode
let baseUrlInitialized = false;

async function ensureBaseUrl() {
  if (!baseUrlInitialized) {
    baseUrlInitialized = true;
    const baseURL = await getApiBaseUrl();
    client.defaults.baseURL = baseURL;
    refreshClient.defaults.baseURL = baseURL;
  }
}

// Initialize on module load (non-blocking)
ensureBaseUrl();

let isRefreshing = false;
let failedQueue: Array<{
  resolve: (token: string) => void;
  reject: (err: unknown) => void;
}> = [];

function processQueue(error: unknown, token: string | null) {
  failedQueue.forEach((prom) => {
    if (error) {
      prom.reject(error);
    } else {
      prom.resolve(token!);
    }
  });
  failedQueue = [];
}

client.interceptors.request.use(async (config) => {
  // Ensure base URL is set before any request
  await ensureBaseUrl();

  const tokens = localStorage.getItem('konto_tokens');
  if (tokens) {
    try {
      const { access_token } = JSON.parse(tokens);
      config.headers.Authorization = `Bearer ${access_token}`;
    } catch {
      localStorage.removeItem('konto_tokens');
    }
  }
  return config;
});

client.interceptors.response.use(
  (response) => response,
  async (error) => {
    const originalRequest = error.config;

    if (error.response?.status !== 401 || originalRequest._retry) {
      return Promise.reject(error);
    }

    if (isRefreshing) {
      return new Promise((resolve, reject) => {
        failedQueue.push({ resolve, reject });
      }).then((token) => {
        originalRequest.headers.Authorization = `Bearer ${token}`;
        return client(originalRequest);
      });
    }

    originalRequest._retry = true;
    isRefreshing = true;

    try {
      const tokens = localStorage.getItem('konto_tokens');
      if (!tokens) throw new Error('No tokens');

      let refresh_token: string;
      try {
        ({ refresh_token } = JSON.parse(tokens));
      } catch {
        localStorage.removeItem('konto_tokens');
        throw new Error('Corrupted tokens');
      }
      const { data } = await refreshClient.post('/auth/refresh', { refresh_token });

      localStorage.setItem(
        'konto_tokens',
        JSON.stringify({
          access_token: data.access_token,
          refresh_token: data.refresh_token,
        })
      );

      processQueue(null, data.access_token);
      originalRequest.headers.Authorization = `Bearer ${data.access_token}`;
      return client(originalRequest);
    } catch (refreshError) {
      processQueue(refreshError, null);
      localStorage.removeItem('konto_tokens');
      localStorage.removeItem('konto_user');
      window.location.href = '/login';
      return Promise.reject(refreshError);
    } finally {
      isRefreshing = false;
    }
  }
);

/**
 * Extract a human-readable error message from an API error.
 * The backend returns { error: "message" } for AppError responses.
 */
export function extractErrorMessage(error: unknown): string {
  if (axios.isAxiosError(error)) {
    const data = error.response?.data;
    if (data && typeof data === 'object') {
      if ('error' in data && typeof data.error === 'string') return data.error;
      if ('message' in data && typeof data.message === 'string') return data.message;
    }
    if (error.response?.status === 413) return 'File too large';
    if (error.response?.status === 422) return 'Validation error';
    if (error.message) return error.message;
  }
  if (error instanceof Error) return error.message;
  return 'An unexpected error occurred';
}

export default client;
