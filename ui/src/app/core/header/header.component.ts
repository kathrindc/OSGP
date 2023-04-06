import { Component } from '@angular/core';
import { AuthService } from 'src/app/services/auth.service';

@Component({
  selector: 'osgp-core-header',
  templateUrl: './header.component.html',
  styleUrls: ['./header.component.less'],
  host: { 'class': 'header header-6' }
})
export class HeaderComponent {
  public user: string = 'Admin McAdminface';

  constructor(
    public authService: AuthService,
  ) {
  }
}
