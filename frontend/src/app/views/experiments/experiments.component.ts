import { Component, OnInit } from '@angular/core';
import { ActivatedRoute, Router } from '@angular/router';
// note the import from apollo/client/core not to avoid a stupid fucking react dep 
import { Apollo, gql, Subscription, } from 'apollo-angular';


const GET_EXPERIMENTS_FOR_COFFEE = gql`
query GetExperimentsForCoffee($coffeeId: String){
    coffee(id: $coffeeId){
        id
        name,
        process,
        grower,
        experiments{
            id,
            date,
            dose, 
            waterDose,
            brewer,
            filter,
            water,
            waterTemp,
            grinder, 
            grindSetting,
            pourStructure,
            notes
        }
    }
}`;

const MARK_COFFEE_AS_FINISHED = gql`
mutation MarkCoffeeAsFinished($coffeeId: String!){ 
    markCoffeeAsFinished(
        coffeeId: $coffeeId
    )
}
`;

const DELETE_COFFEE = gql`
mutation DeleteCoffee($coffeeID: String!){
    deleteCoffee(coffeeId: $coffeeID)
}`;


@Component({
    selector: 'app-tables',
    templateUrl: './experiments.component.html',
    styleUrls: ['./experiments.component.scss']
})
export class ExperimentsComponent implements OnInit {
    coffeeData: any;
    coffeeId: string

    constructor(private apollo: Apollo, private route: ActivatedRoute, private router: Router) {
    }

    ngOnInit() {
        this.route.queryParams
            .subscribe(params => {
                this.coffeeId = params["coffeeId"];
            }
            );
        this.apollo
            .watchQuery<any>({
                query: GET_EXPERIMENTS_FOR_COFFEE,
                variables: { coffeeId: this.coffeeId },
                fetchPolicy: "no-cache"
            })
            .valueChanges.subscribe(({ data }) => {
                this.coffeeData = data.coffee;
            });
    }

    markCoffeeAsFinished() {
        this.apollo.mutate({
            mutation: MARK_COFFEE_AS_FINISHED,
            variables: { coffeeId: this.coffeeId }
        }).subscribe()
    }

    deleteCoffee() {
        this.apollo.mutate({
            mutation: DELETE_COFFEE,
            variables: { coffeeID: this.coffeeId }
        }).subscribe();
        this.router.navigate(['/currentRotation']);
    }

}
