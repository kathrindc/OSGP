import { Component } from '@angular/core';
import { Router } from '@angular/router';
import { AuthService } from 'src/app/services/auth.service';

@Component({
  selector: 'osgp-pages-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.less'],
  host: { 'class': 'login-wrapper' }
})
export class LoginPageComponent {

  public form = {
    role: 'customer',
    email: '',
    password: '',
    remember: false,
  };
  public errorMessage = '';

  constructor(
    public authService: AuthService,
    public router: Router,
  ) {
  }

  async submit() {
    try {
      await this.authService.login(this.form.role, this.form.email, this.form.password);
      await this.router.navigate(['/home']);
    } catch (error: any) {
      this.errorMessage = error.message;
    }
  }
}
