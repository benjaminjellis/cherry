import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';

import { AllCoffeesComponent } from './all-coffees.component';

const routes: Routes = [
    {
        path: '',
        component: AllCoffeesComponent,
        data: {
            title: $localize`Dashboard`
        }
    }
];

@NgModule({
    imports: [RouterModule.forChild(routes)],
    exports: [RouterModule]
})
export class AllCoffeesRoutingModule {
}
