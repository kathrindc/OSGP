import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { DashboardPageComponent } from './pages/dashboard/dashboard.component';
import { LoginPageComponent } from './pages/login/login.component';
import { NotFoundPageComponent } from './pages/notfound/notfound.component';
import { loginGuard } from './util/login-guard';

const routes: Routes = [
  {
    path: '',
    pathMatch: 'full',
    redirectTo: '/home',
  },
  {
    path: 'home',
    pathMatch: 'full',
    canActivate: [loginGuard],
    component: DashboardPageComponent,
  },
  {
    path: 'login',
    pathMatch: 'full',
    component: LoginPageComponent,
  },
  {
    path: '**',
    pathMatch: 'full',
    component: NotFoundPageComponent,
  },
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
