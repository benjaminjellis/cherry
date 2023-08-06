import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';

import { AddNewCoffeeComponent } from './add-new-coffee.component';

const routes: Routes = [
    {
        path: '',
        component: AddNewCoffeeComponent,
        data: {
            title: $localize`Dashboard`
        }
    }
];

@NgModule({
    imports: [RouterModule.forChild(routes)],
    exports: [RouterModule]
})
export class AddNewCoffeeRoutingModule {
}
