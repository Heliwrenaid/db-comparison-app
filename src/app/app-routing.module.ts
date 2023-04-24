import { DbTestComponent } from './db-test/db-test.component';
import { DbQueryComponent } from './db-query/db-query.component';
import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';

const routes: Routes = [
  { path: 'db/query', component: DbQueryComponent },
  { path: 'db/test', component: DbTestComponent }
]; 

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }