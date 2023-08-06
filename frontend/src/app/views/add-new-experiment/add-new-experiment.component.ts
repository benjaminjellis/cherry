import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { ActivatedRoute, Router } from '@angular/router';
import { Apollo, gql, } from 'apollo-angular';

const ADD_NEW_EXPERIMENT = gql`
mutation AddNewExperiment(
    $coffeeId: String!,
    $date: String!,
    $brewer: String!,
    $filter: String!,
    $water: String!,
    $waterTemp: Int!,
    $dose: Int!,
    $waterDose: Int!,
    $grinder: String!,
    $grindSetting: String!,
    $pourStructure: String!,
    $notes: String!
){
    addNewExperiment(
        coffeeId: $coffeeId,
        experimentDate: $date,
        brewer: $brewer,
        filter: $filter,
        water: $water,
        waterTemp: $waterTemp,
        dose: $dose,
        waterDose: $waterDose,
        grinder: $grinder,
        grindSetting: $grindSetting,
        pourStructure: $pourStructure,
        notes: $notes
    )
}`;


@Component({
    templateUrl: './add-new-experiment.component.html',
    styleUrls: ['./add-new-experiment.component.scss'],
})
export class AddNewExperimentComponent implements OnInit {
    addExperimentForm!: FormGroup;
    loading: boolean;
    coffees: any;
    browserDefaultsValidated: boolean;
    customStylesValidated: boolean;
    tooltipValidated: boolean;
    coffeeId: String;
    coffeeName: String

    constructor(private apollo: Apollo, private formBuilder: FormBuilder, private router: Router, private route: ActivatedRoute) {
        this.createForm();
    }

    ngOnInit() {
        this.route.queryParams
            .subscribe(params => {
                this.coffeeId = params["coffeeId"];
                this.coffeeName = params["coffeeName"]
            }
            );
    }

    createForm() {
        this.addExperimentForm = this.formBuilder.group({
            dose: [""],
            waterDose: [],
            waterTemp: [""],
            water: [""],
            brewer: [""],
            filter: [""],
            grinder: [""],
            grindSetting: [""],
            pourStructure: [""],
            notes: [""],
            date: [""]
        })
    }

    onSubmit1() {
        this.customStylesValidated = true;
        console.log('Submit... 1');
    }

    onReset1() {
        this.customStylesValidated = false;
        console.log('Reset... 1');
    }

    onSubmit2() {
        this.browserDefaultsValidated = true;
        const formData = this.addExperimentForm.value;
        this.apollo.mutate(
            {
                mutation: ADD_NEW_EXPERIMENT,
                variables: {
                    coffeeId: String(this.coffeeId),
                    date: String(formData["date"]),
                    dose: Number(formData["dose"]),
                    waterDose: Number(formData["waterDose"]),
                    brewer: String(formData["brewer"]),
                    filter: String(formData["filter"]),
                    water: String(formData["water"]),
                    waterTemp: Number(formData["waterTemp"]),
                    grinder: String(formData["grinder"]),
                    grindSetting: String(formData["grindSetting"]),
                    pourStructure: String(formData["pourStructure"]),
                    notes: String(formData["notes"])
                }
            }
        ).subscribe();
        this.router.navigate(['/experiments'],
            {
                queryParams: {
                    coffeeId: this.coffeeId
                }
            });
    }

    onReset2() {
        this.browserDefaultsValidated = false;
        console.log('Reset... 3');
    }

    onSubmit3() {
        this.tooltipValidated = true;
        console.log('Submit... 3');
    }

    onReset3() {
        this.tooltipValidated = false;
        console.log('Reset... 3');
    }


}
