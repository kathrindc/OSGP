import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { ClarityModule } from '@clr/angular';
import { NavigationComponent } from './core/navigation/navigation.component';
import { HeaderComponent } from './core/header/header.component';
import { DashboardPageComponent } from './pages/dashboard/dashboard.component';
import './app-icons';
import { NotFoundPageComponent } from './pages/notfound/notfound.component';
import { HttpClientModule } from '@angular/common/http';
import { JwtModule } from '@auth0/angular-jwt';
import { AuthService } from './services/auth.service';
import { LoginPageComponent } from './pages/login/login.component';
import { FormsModule } from '@angular/forms';
import { LogoutPageComponent } from './pages/logout/logout.component';
import { ApiDomain } from './api-host';

@NgModule({
  declarations: [
    AppComponent,
    NavigationComponent,
    HeaderComponent,
    DashboardPageComponent,
    NotFoundPageComponent,
    LoginPageComponent,
    LogoutPageComponent
  ],
  imports: [
    BrowserModule,
    BrowserAnimationsModule,

    AppRoutingModule,
    FormsModule,

    HttpClientModule,
    JwtModule.forRoot({
      config: {
        allowedDomains: [ApiDomain],
        tokenGetter: () => AuthService.CurrentToken,
      }
    }),

    ClarityModule,
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }
