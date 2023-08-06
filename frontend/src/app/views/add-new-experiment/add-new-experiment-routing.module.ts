import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';

import { AddNewExperimentComponent } from './add-new-experiment.component';

const routes: Routes = [
    {
        path: '',
        component: AddNewExperimentComponent,
        data: {
            title: $localize`Dashboard`
        }
    }
];

@NgModule({
    imports: [RouterModule.forChild(routes)],
    exports: [RouterModule]
})
export class AddNewExperimentRoutingModule {
}
