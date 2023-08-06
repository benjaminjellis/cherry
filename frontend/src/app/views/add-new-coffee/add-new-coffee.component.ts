import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { ActivatedRoute, Router } from '@angular/router';
import { Apollo, gql, } from 'apollo-angular';

const ADD_NEW_COFFEE = gql`
mutation AddNewCoffee(
    $name: String!, 
    $roaster: String!,
    $process: String!,
    $varietal: String!,
    $grower: String!,
    $farm: String!,
    $country: String!,
    $tastingNotes: String!,
    $roastDate: String!
    ){
    addNewCoffee(
        name: $name,
        roaster: $roaster,
        process: $process,
        varietal: $varietal,
        grower: $grower,
        farm: $farm,
        country: $country,
        tastingNotes: $tastingNotes,
        roastDate: $roastDate
      )
}`;


@Component({
    // selector: 'app-tables',
    templateUrl: './add-new-coffee.component.html',
    styleUrls: ['./add-new-coffee.component.scss'],
})
export class AddNewCoffeeComponent {
    addCoffeeForm!: FormGroup;
    loading: boolean;
    coffees: any;
    queryParams: any
    browserDefaultsValidated: boolean;
    customStylesValidated: boolean;
    tooltipValidated: boolean;

    constructor(private apollo: Apollo, private formBuilder: FormBuilder, private router: Router) {
        this.createForm();
    }

    createForm() {
        this.addCoffeeForm = this.formBuilder.group({
            coffeeName: [""],
            roaster: [""],
            tastingNotes: [""],
            varietal: [""],
            process: [""],
            grower: [""],
            country: [""],
            farm: [""],
            roastDate: [""],
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
        const formData = this.addCoffeeForm.value;
        this.apollo.mutate(
            {
                mutation: ADD_NEW_COFFEE,
                variables: {
                    name: String(formData["coffeeName"]),
                    roaster: String(formData["roaster"]),
                    tastingNotes: String(formData["tastingNotes"]),
                    varietal: String(formData["varietal"]),
                    process: String(formData["process"]),
                    grower: String(formData["grower"]),
                    country: String(formData["country"]),
                    farm: String(formData["farm"]),
                    roastDate: String(formData["roastDate"])
                }
            }
        ).subscribe();
        this.router.navigate(['/currentRotation']);
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
