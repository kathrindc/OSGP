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

@NgModule({
  declarations: [
    AppComponent,
    NavigationComponent,
    HeaderComponent,
    DashboardPageComponent,
    NotFoundPageComponent
  ],
  imports: [
    BrowserModule,
    HttpClientModule,
    AppRoutingModule,
    BrowserAnimationsModule,
    ClarityModule,

    JwtModule.forRoot({
      config: {
        tokenGetter: () => AuthService.CurrentToken,
      }
    })
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }
