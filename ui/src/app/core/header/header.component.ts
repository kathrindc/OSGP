import { Component } from '@angular/core';

@Component({
  selector: 'osgp-core-header',
  templateUrl: './header.component.html',
  styleUrls: ['./header.component.less'],
  host: { 'class': 'header header-6' }
})
export class HeaderComponent {
  public loggedIn: boolean = true;
  public user: string = 'Admin McAdminface';
}
