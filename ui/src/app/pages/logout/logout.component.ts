import { Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';
import { AuthService } from 'src/app/services/auth.service';

@Component({
  selector: 'osgp-pages-logout',
  templateUrl: './logout.component.html',
  styleUrls: ['./logout.component.less']
})
export class LogoutPageComponent implements OnInit {

  constructor(
    private authService: AuthService,
    private router: Router
  ) {
  }

  ngOnInit(): void {
    this.authService.logout();
    this.router.navigate(['/login']);
  }
}
