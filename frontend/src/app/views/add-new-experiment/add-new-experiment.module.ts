import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';

import {
    AvatarModule,
    ButtonGroupModule,
    ButtonModule,
    CardModule,
    FormModule,
    GridModule,
    NavModule,
    ProgressModule,
    TableModule,
    TabsModule
} from '@coreui/angular';
import { IconModule } from '@coreui/icons-angular';
import { ChartjsModule } from '@coreui/angular-chartjs';

import { AddNewExperimentRoutingModule } from './add-new-experiment-routing.module';
import { AddNewExperimentComponent } from './add-new-experiment.component';

import { WidgetsModule } from '../widgets/widgets.module';

@NgModule({
    imports: [
        AddNewExperimentRoutingModule,
        CardModule,
        NavModule,
        IconModule,
        TabsModule,
        CommonModule,
        GridModule,
        ProgressModule,
        ReactiveFormsModule,
        ButtonModule,
        FormModule,
        ButtonModule,
        ButtonGroupModule,
        ChartjsModule,
        AvatarModule,
        TableModule,
        WidgetsModule,
        FormsModule
    ],
    declarations: [AddNewExperimentComponent]
})
export class AddNewExperimentModule {

}