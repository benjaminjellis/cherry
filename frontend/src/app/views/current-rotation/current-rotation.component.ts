import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
// note the import from apollo/client/core not to avoid a stupid fucking react dep 
import { Apollo, gql, Subscription, } from 'apollo-angular';

const GET_CURRENT_ROTATION = gql`
query GetCurrentRotation{
    coffees (isFinished: false){
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
    templateUrl: './current-rotation.component.html',
    styleUrls: ['./current-rotation.component.scss']
})
export class CurrentRotationComponent implements OnInit {
    loading: boolean;
    coffees: any;
    queryParams: any

    constructor(private apollo: Apollo, private route: ActivatedRoute) {
    }

    async ngOnInit() {
        this.route.queryParams
            .subscribe(params => {
                console.log(params);
            }
            );
        this.apollo
            .query<any>({
                query: GET_CURRENT_ROTATION,
                fetchPolicy: "no-cache"
            }).subscribe(({ data, loading }) => {
                this.loading = loading;
                this.coffees = data.coffees;
            });
    }

}
