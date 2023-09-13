// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Integer,
        user_id -> Integer,
        name -> Text,
        parent_id -> Nullable<Integer>,
    }
}

diesel::table! {
    companies (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        company_id -> Integer,
        name -> Text,
    }
}

diesel::joinable!(categories -> users (user_id));
diesel::joinable!(users -> companies (company_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    companies,
    users,
);
