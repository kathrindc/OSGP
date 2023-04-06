import { inject } from '@angular/core';
import { Router } from '@angular/router';
import { AuthService } from '../services/auth.service';

export const loginGuard = () => {
  const router = inject(Router);
  const service = inject(AuthService);

  if (!service?.isAuthenticated?.()) {
    router.navigate(['/login']);

    return false;
  }

  return true;
};
