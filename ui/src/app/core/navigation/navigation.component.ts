import { Component } from '@angular/core';
import { Router } from '@angular/router';
import { AuthService } from 'src/app/services/auth.service';

@Component({
  selector: 'osgp-core-navigation',
  templateUrl: './navigation.component.html',
  styleUrls: ['./navigation.component.less'],
  host: { 'class': 'content-container' },
})
export class NavigationComponent {

  public fullView = false;
  private fullViewUrls = [
    '/login',
  ];

  constructor(
    public authService: AuthService,
    private router: Router,
  ) {
    this.router.events.subscribe((_event) => {
      this.update();
    });

    this.update();
  }

  update() {
    this.fullView = this.fullViewUrls.includes(this.router.url)
  }
}
