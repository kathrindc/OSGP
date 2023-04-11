import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Router } from '@angular/router';
import { JwtHelperService } from '@auth0/angular-jwt';
import { firstValueFrom, lastValueFrom, map, Subject } from 'rxjs';
import { ApiHost } from '../api-host';
import ApiResponse from '../types/response';
import User from '../types/user';
import LogonHistory from '../types/logon-history';

@Injectable({
  providedIn: 'root'
})
export class AuthService {

  private static AuthKeyName = window.btoa('osgp-akey');
  private static _userSubject = new Subject<User | null>();
  private static _user: User | null = null;

  constructor(
    public jwtHelper: JwtHelperService,
    public httpClient: HttpClient,
    public router: Router,
  ) {
  }

  static get CurrentToken() {
    return localStorage.getItem(AuthService.AuthKeyName);
  }

  static get CurrentUser() {
    return AuthService._user;
  }

  static get UserSubject() {
    return AuthService._userSubject;
  }

  public async login(
    role: string,
    email: string,
    password: string,
  ): Promise<void> {
    const observable = this.httpClient.post(
      `${ApiHost}/api/v1/logon`,
      { role, email, password }
    );
    const { ok, body } = (
      await lastValueFrom(observable)
    ) as ApiResponse<string>;

    if (!ok) {
      throw new Error(body);
    }

    localStorage.setItem(AuthService.AuthKeyName, body);

    await this.loadInfo(body as string);
  }

  public logout() {
    localStorage.removeItem(AuthService.AuthKeyName);
    AuthService._userSubject.next(null);
    AuthService._user = null;

    this.router.navigate(['/login']);
  }

  public async loadHistory(): Promise<LogonHistory[]> {
    const observable = this.httpClient.get(
      `${ApiHost}/api/v1/logon/history`
    );
    const { ok, body } = (
      await lastValueFrom(observable)
    ) as ApiResponse<LogonHistory[]>;

    if (!ok) {
      throw new Error(body as string);
    }

    return body as LogonHistory[];
  }

  public isAuthenticated(): boolean {
    return !this.jwtHelper.isTokenExpired(AuthService.CurrentToken);
  }

  public async initUserData(): Promise<void> {
    let token = AuthService.CurrentToken;

    if (token && this.isAuthenticated()) {
      await this.loadInfo(token);
    } else {
      AuthService._userSubject.next(null);
    }
  }

  private async loadInfo(token: string) {
    try {
      const observable = this.httpClient.get(
        `${ApiHost}/api/v1/logon`,
        { headers: { 'Authorization': `Bearer ${token}` } }
      );
      const { ok, body } = (
        await lastValueFrom(observable)
      ) as ApiResponse<User>;

      if (!ok) {
        throw new Error(body as string);
      }

      AuthService._userSubject.next(body as User);
      AuthService._user = body as User;
    } catch (error: any) {
      if (error.status != 401) {
        throw error;
      }

      this.logout();
    }
  }
}
