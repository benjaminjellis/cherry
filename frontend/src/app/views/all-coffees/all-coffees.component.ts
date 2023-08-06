import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { Apollo, gql, } from 'apollo-angular';

const GET_ALL_COFFEES = gql`
query GetAllCoffees{
    coffees{
        id,
        name, 
        roaster, 
        process, 
        grower,
        roastDate,
        farm,
        varietal,
        tastingNotes,
    }
}`;


@Component({
    selector: 'app-tables',
    templateUrl: './all-coffees.component.html',
    styleUrls: ['./all-coffees.component.scss']
})
export class AllCoffeesComponent implements OnInit {
    loading: boolean;
    coffees: any;
    queryParams: any

    constructor(private apollo: Apollo, private route: ActivatedRoute) {
    }

    ngOnInit() {
        this.route.queryParams
            .subscribe(params => {
                console.log(params);
            }
            );
        this.apollo
            .watchQuery<any>({
                query: GET_ALL_COFFEES,
                fetchPolicy: "no-cache"
            })
            .valueChanges.subscribe(({ data, loading }) => {
                this.loading = loading;
                this.coffees = data.coffees;
            });
    }

}
