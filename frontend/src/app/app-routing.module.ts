import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';

import { DefaultLayoutComponent } from './containers';
import { Page404Component } from './views/pages/page404/page404.component';
import { Page500Component } from './views/pages/page500/page500.component';
import { LoginComponent } from './views/pages/login/login.component';
import { RegisterComponent } from './views/pages/register/register.component';

const routes: Routes = [
  {
    path: '',
    redirectTo: 'currentRotation',
    pathMatch: 'full'
  },
  {
    path: '',
    component: DefaultLayoutComponent,
    data: {
      title: 'Home'
    },
    children: [
      {
        path: 'currentRotation',
        loadChildren: () =>
          import('./views/current-rotation/current-rotation.module').then((m) => m.CurrentRotationModule)
      },
      {
        path: 'allCoffees',
        loadChildren: () =>
          import('./views/all-coffees/all-coffees.module').then((m) => m.AllCoffeesModule)
      },
      {
        path: 'experiments',
        loadChildren: () =>
          import('./views/experiments/experiments.module').then((m) => m.ExperimentsModule)
      },
      {
        path: 'addNewCoffee',
        loadChildren: () =>
          import('./views/add-new-coffee/add-new-coffee.module').then((m) => m.AddNewCoffeeModule)
      },
      {
        path: 'addNewExperiment',
        loadChildren: () =>
          import('./views/add-new-experiment/add-new-experiment.module').then((m) => m.AddNewExperimentModule)
      },
      {
        path: 'pages',
        loadChildren: () =>
          import('./views/pages/pages.module').then((m) => m.PagesModule)
      },
    ]
  },
  {
    path: '404',
    component: Page404Component,
    data: {
      title: 'Page 404'
    }
  },
  {
    path: '500',
    component: Page500Component,
    data: {
      title: 'Page 500'
    }
  },
  {
    path: 'login',
    component: LoginComponent,
    data: {
      title: 'Login Page'
    }
  },
  {
    path: 'register',
    component: RegisterComponent,
    data: {
      title: 'Register Page'
    }
  },
  { path: '**', redirectTo: 'dashboard' }
];

@NgModule({
  imports: [
    RouterModule.forRoot(routes, {
      scrollPositionRestoration: 'top',
      anchorScrolling: 'enabled',
      initialNavigation: 'enabledBlocking'
      // relativeLinkResolution: 'legacy'
    })
  ],
  exports: [RouterModule]
})
export class AppRoutingModule {
}
