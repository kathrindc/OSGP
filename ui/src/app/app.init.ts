import { AuthService } from './services/auth.service';
import { APP_INITIALIZER } from '@angular/core';

export const AppInit = {
  provide: APP_INITIALIZER,
  multi: true,
  deps: [
    AuthService,
  ],

  useFactory(authService: AuthService): () => Promise<any> {
    return async (): Promise<any> => {
      await authService.initUserData();
    };
  }
};
