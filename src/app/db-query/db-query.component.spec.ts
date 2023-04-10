import { ComponentFixture, TestBed } from '@angular/core/testing';

import { DbQueryComponent } from './db-query.component';

describe('DbQueryComponent', () => {
  let component: DbQueryComponent;
  let fixture: ComponentFixture<DbQueryComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ DbQueryComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(DbQueryComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
