# Diesel Bug Report

Versions:
- **Rust**: 1.72.0
- **Diesel**: 2.1.1
- **Database**: SQLite
- **Operating System**: Ubuntu 20.04.6 LTS

Reproduce steps:
1. clone the repository.
2. Run with `cargo run`.

```
A dummy database is included in this repository.
./src/db.sqlite
```

## Problem

Compilation failed with these error message:
```
error[E0277]: the trait bound `SelectStatement<FromClause<JoinOn<query_source::joins::Join<users::table, companies::table, Inner>, expression::grouped::Grouped<expression::operators::Eq<NullableExpression<users::columns::company_id>, NullableExpression<companies::columns::id>>>>>>: Table` is not satisfied
  --> src/main.rs:46:10
   |
46 |         .inner_join(schema::users::table.inner_join(schema::companies::table))
   |          ^^^^^^^^^^ the trait `Table` is not implemented for `SelectStatement<FromClause<JoinOn<query_source::joins::Join<users::table, companies::table, Inner>, expression::grouped::Grouped<expression::operators::Eq<NullableExpression<users::columns::company_id>, NullableExpression<companies::columns::id>>>>>>`
   |
   = help: the following other types implement trait `Table`:
             categories::table
             companies::table
             users::table
   = note: required for `Alias<category_child>` to implement `JoinTo<SelectStatement<FromClause<JoinOn<query_source::joins::Join<users::table, companies::table, Inner>, expression::grouped::Grouped<expression::operators::Eq<NullableExpression<users::columns::company_id>, NullableExpression<companies::columns::id>>>>>>>`
   = note: 1 redundant requirement hidden
   = note: required for `query_source::joins::Join<Alias<category_child>, Alias<category_parent>, Inner>` to implement `JoinTo<SelectStatement<FromClause<JoinOn<query_source::joins::Join<users::table, companies::table, Inner>, expression::grouped::Grouped<expression::operators::Eq<NullableExpression<users::columns::company_id>, NullableExpression<companies::columns::id>>>>>>>`
   = note: required for `SelectStatement<FromClause<JoinOn<Join<Alias<category_child>, Alias<category_parent>, Inner>, Grouped<...>>>>` to implement `JoinWithImplicitOnClause<SelectStatement<FromClause<JoinOn<query_source::joins::Join<users::table, companies::table, Inner>, expression::grouped::Grouped<expression::operators::Eq<NullableExpression<users::columns::company_id>, NullableExpression<companies::columns::id>>>>>>, Inner>`
```
```
error[E0277]: the trait bound `SelectStatement<FromClause<JoinOn<query_source::joins::Join<users::table, companies::table, Inner>, expression::grouped::Grouped<expression::operators::Eq<NullableExpression<users::columns::company_id>, NullableExpression<companies::columns::id>>>>>>: Table` is not satisfied
  --> src/main.rs:46:10
   |
46 |         .inner_join(schema::users::table.inner_join(schema::companies::table))
   |          ^^^^^^^^^^ the trait `Table` is not implemented for `SelectStatement<FromClause<JoinOn<query_source::joins::Join<users::table, companies::table, Inner>, expression::grouped::Grouped<expression::operators::Eq<NullableExpression<users::columns::company_id>, NullableExpression<companies::columns::id>>>>>>`
   |
   = help: the following other types implement trait `Table`:
             categories::table
             companies::table
             users::table
   = note: required for `Alias<category_child>` to implement `JoinTo<SelectStatement<FromClause<JoinOn<query_source::joins::Join<users::table, companies::table, Inner>, expression::grouped::Grouped<expression::operators::Eq<NullableExpression<users::columns::company_id>, NullableExpression<companies::columns::id>>>>>>>`
   = note: 1 redundant requirement hidden
   = note: required for `query_source::joins::Join<Alias<category_child>, Alias<category_parent>, Inner>` to implement `JoinTo<SelectStatement<FromClause<JoinOn<query_source::joins::Join<users::table, companies::table, Inner>, expression::grouped::Grouped<expression::operators::Eq<NullableExpression<users::columns::company_id>, NullableExpression<companies::columns::id>>>>>>>`
   = note: required for `SelectStatement<FromClause<JoinOn<Join<Alias<category_child>, Alias<category_parent>, Inner>, Grouped<...>>>>` to implement `JoinWithImplicitOnClause<SelectStatement<FromClause<JoinOn<query_source::joins::Join<users::table, companies::table, Inner>, expression::grouped::Grouped<expression::operators::Eq<NullableExpression<users::columns::company_id>, NullableExpression<companies::columns::id>>>>>>, Inner>`
```

## Expected Output

The program should print out the output:
```
(
    Category {
        id: 1,
        user_id: 1,
        name: "Category A",
        parent_id: None,
    },
    None,
    (
        User {
            id: 1,
            company_id: 1,
            name: "John Smith",
        },
        Company {
            id: 1,
            name: "Company A",
        },
    ),
)
```

## Alternative Solution

[See this answer](https://github.com/diesel-rs/diesel/discussions/3785#discussioncomment-6981859)