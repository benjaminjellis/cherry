import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';

import { ExperimentsComponent } from './experiments.component';

const routes: Routes = [
    {
        path: '',
        component: ExperimentsComponent,
        data: {
            title: $localize`Dashboard`
        }
    }
];

@NgModule({
    imports: [RouterModule.forChild(routes)],
    exports: [RouterModule]
})
export class ExperimentsRoutingModule {
}
