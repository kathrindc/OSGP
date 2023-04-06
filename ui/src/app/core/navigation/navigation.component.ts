import { Component } from '@angular/core';
import { AuthService } from 'src/app/services/auth.service';

@Component({
  selector: 'osgp-core-navigation',
  templateUrl: './navigation.component.html',
  styleUrls: ['./navigation.component.less'],
  host: { 'class': 'content-container' },
})
export class NavigationComponent {

  constructor(
    public authService: AuthService,
  ) {
  }
}
