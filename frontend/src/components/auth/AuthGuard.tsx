import { Navigate, Outlet } from 'react-router-dom';
import { useAuth } from '@/hooks/useAuth';
import { useSetupStatus } from '@/hooks/useSetup';
import { Loader2 } from 'lucide-react';

export function AuthGuard() {
  const { isAuthenticated, isLoading } = useAuth();
  const { data: setupStatus, isLoading: setupLoading } = useSetupStatus();

  if (isLoading || setupLoading) {
    return (
      <div className="flex h-screen items-center justify-center">
        <Loader2 className="h-8 w-8 animate-spin text-primary" />
      </div>
    );
  }

  if (setupStatus?.setup_needed) {
    return <Navigate to="/setup" replace />;
  }

  if (!isAuthenticated) {
    return <Navigate to="/login" replace />;
  }

  return <Outlet />;
}
