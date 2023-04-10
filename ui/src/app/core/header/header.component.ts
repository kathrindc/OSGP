import { Component, OnInit } from '@angular/core';
import { AuthService } from 'src/app/services/auth.service';
import User from 'src/app/types/user';

@Component({
  selector: 'osgp-core-header',
  templateUrl: './header.component.html',
  styleUrls: ['./header.component.less'],
  host: { 'class': 'header header-6' }
})
export class HeaderComponent implements OnInit {
  public user: User | null = null;

  constructor(
    public authService: AuthService,
  ) {
  }

  ngOnInit(): void {
    AuthService.User.subscribe(user => { this.user = user });
  }
}
