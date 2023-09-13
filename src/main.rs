pub mod schema;

use diesel::prelude::*;
use diesel::JoinOnDsl;

#[derive(Identifiable, Queryable, Selectable, Associations, Debug)]
#[diesel(belongs_to(Category, foreign_key = parent_id))]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(table_name = schema::categories)]
#[allow(unused)]
struct Category {
    id: i32,
    user_id: i32,
    name: String,
    parent_id: Option<i32>
}

#[derive(Identifiable, Queryable, Selectable, Associations, Debug)]
#[diesel(belongs_to(Company, foreign_key = company_id))]
#[diesel(table_name = schema::users)]
#[allow(unused)]
struct User {
    id: i32,
    company_id: i32,
    name: String
}

#[derive(Identifiable, Queryable, Selectable, Debug)]
#[diesel(table_name = schema::companies)]
#[allow(unused)]
struct Company {
    id: i32,
    name: String
}

fn main(){
    let mut conn = SqliteConnection::establish("sqlite://src/db.sqlite").unwrap();

    let (category_child, category_parent) = diesel::alias!(
        schema::categories as category_child,
        schema::categories as category_parent,
    );

    let data : (Category, Option<Category>, (User, Company)) = category_child
        .left_join(category_parent.on(
            category_child.field(schema::categories::parent_id).eq(category_parent.field(schema::categories::id).nullable())
        ))
        // Error
        .inner_join(schema::users::table.inner_join(schema::companies::table))
        .select((
            category_child.fields((
                schema::categories::id,
                schema::categories::user_id,
                schema::categories::name,
                schema::categories::parent_id,
            )),
            category_parent.fields((
                schema::categories::id,
                schema::categories::user_id,
                schema::categories::name,
                schema::categories::parent_id,
            )).nullable(),
            (User::as_select(), Company::as_select())
        ))
        .filter(category_child.field(schema::categories::id).eq(1))
        .first(&mut conn).unwrap();

    // Alternative solution: https://github.com/diesel-rs/diesel/discussions/3785#discussioncomment-6981859
    // let data : (Category, Option<Category>, (User, Company)) = schema::users::table.inner_join(
    //         category_child
    //             .left_join(category_parent.on(
    //                 category_child.field(schema::categories::parent_id).eq(category_parent.field(schema::categories::id).nullable())
    //             ))
    //     )
    //     .inner_join(schema::companies::table)
    //     .select((
    //         category_child.fields((
    //             schema::categories::id,
    //             schema::categories::user_id,
    //             schema::categories::name,
    //             schema::categories::parent_id,
    //         )),
    //         category_parent.fields((
    //             schema::categories::id,
    //             schema::categories::user_id,
    //             schema::categories::name,
    //             schema::categories::parent_id,
    //         )).nullable(),
    //         (User::as_select(), Company::as_select())
    //     ))
    //     .filter(category_child.field(schema::categories::id).eq(1))
    //     .first(&mut conn).unwrap();

    println!("{:#?}", data);
}