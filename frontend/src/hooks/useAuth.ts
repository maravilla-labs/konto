import { useAuthStore } from '@/stores/authStore';

export function useAuth() {
  const {
    user,
    isAuthenticated,
    isLoading,
    login,
    logout,
    initialize,
    setLanguagePreference,
    updateProfile,
    uploadAvatar,
  } =
    useAuthStore();

  return {
    user,
    isAuthenticated,
    isLoading,
    login,
    logout,
    initialize,
    setLanguagePreference,
    updateProfile,
    uploadAvatar,
  };
}
