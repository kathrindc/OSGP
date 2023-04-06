import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Router } from '@angular/router';
import { JwtHelperService } from '@auth0/angular-jwt';
import { firstValueFrom, lastValueFrom, map, Observable, pluck } from 'rxjs';
import { ApiHost } from '../api-host';

@Injectable({
  providedIn: 'root'
})
export class AuthService {

  private static AuthKeyName = window.btoa('osgp-akey');

  constructor(
    public jwtHelper: JwtHelperService,
    public httpClient: HttpClient,
    public router: Router,
  ) {
  }

  static get CurrentToken() {
    return localStorage.getItem(AuthService.AuthKeyName);
  }

  public async login(
    mode: string,
    email: string,
    password: string,
  ): Promise<boolean> {
    const observable = this.httpClient.post(
      `${ApiHost}/api/v1/login`,
      { mode, email, password }
    );
    const { token } = (
      await lastValueFrom(observable)
    ) as { token: string };

    if (token) {
      localStorage.setItem(AuthService.AuthKeyName, token);

      return true;
    }

    return false;
  }

  public logout() {
    localStorage.removeItem(AuthService.AuthKeyName);

    this.router.navigate(['/login']);
  }

  public isAuthenticated(): boolean {
    return !this.jwtHelper.isTokenExpired(AuthService.CurrentToken);
  }

  public getRole(): Observable<string> {
    const headers: HttpHeaders = new HttpHeaders();

    headers.append('Accept', 'application/json');
    headers.append('Authorization', `Bearer ${AuthService.CurrentToken}`);

    const observable = this.httpClient.get(
      `${ApiHost}/api/v1/my/account`,
      { headers }
    );

    return observable.pipe(map((x: any) => x.role));
  }
}
