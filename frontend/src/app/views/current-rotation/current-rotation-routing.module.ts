import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';

import { CurrentRotationComponent } from './current-rotation.component';

const routes: Routes = [
    {
        path: '',
        component: CurrentRotationComponent,
        data: {
            title: $localize`Dashboard`
        }
    }
];

@NgModule({
    imports: [RouterModule.forChild(routes)],
    exports: [RouterModule]
})
export class CurrentRotationRoutingModule {
}
