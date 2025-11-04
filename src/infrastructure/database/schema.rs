// @generated automatically by Diesel CLI.

diesel::table! {
    menus (id) {
        id -> Bigint,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 255]
        path -> Varchar,
        #[max_length = 255]
        component -> Varchar,
        #[max_length = 100]
        icon -> Nullable<Varchar>,
        parent_id -> Nullable<Bigint>,
        order_num -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    role_menus (role_id, menu_id) {
        role_id -> Bigint,
        menu_id -> Bigint,
    }
}

diesel::table! {
    roles (id) {
        id -> Bigint,
        #[max_length = 50]
        name -> Varchar,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    system_menus (system_id, menu_id) {
        system_id -> Bigint,
        menu_id -> Bigint,
    }
}

diesel::table! {
    systems (id) {
        id -> Bigint,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 50]
        code -> Varchar,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    user_roles (user_id, role_id) {
        user_id -> Bigint,
        role_id -> Bigint,
    }
}

diesel::table! {
    users (id) {
        id -> Bigint,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::joinable!(role_menus -> menus (menu_id));
diesel::joinable!(role_menus -> roles (role_id));
diesel::joinable!(system_menus -> menus (menu_id));
diesel::joinable!(system_menus -> systems (system_id));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    menus,
    role_menus,
    roles,
    system_menus,
    systems,
    user_roles,
    users,
);
