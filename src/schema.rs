table! {
    form_buys (id) {
        id -> Integer,
        per_com -> Varchar,
        company -> Text,
        city -> Varchar,
        name -> Varchar,
        contact -> Text,
        prod -> Text,
        model -> Text,
        amount -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    form_register_bus (id) {
        id -> Integer,
        username -> Varchar,
        password -> Varchar,
        passwordq -> Nullable<Text>,
        passworda -> Nullable<Text>,
        usertype -> Text,
        name -> Text,
        address -> Text,
        postcode -> Nullable<Text>,
        personincharge -> Nullable<Text>,
        tel -> Nullable<Text>,
        cell -> Nullable<Text>,
        province -> Varchar,
        city -> Varchar,
        mainbusiness -> Nullable<Text>,
        sellingbrand -> Nullable<Text>,
        recommander -> Nullable<Varchar>,
        recommander_name -> Nullable<Varchar>,
        subcompany -> Nullable<Varchar>,
        subcompany_name -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    form_register_pss (id) {
        id -> Integer,
        username -> Varchar,
        password -> Varchar,
        passwordq -> Nullable<Text>,
        passworda -> Nullable<Text>,
        usertype -> Text,
        name -> Text,
        year -> Varchar,
        month -> Varchar,
        company -> Text,
        title -> Text,
        expertise -> Text,
        cell -> Varchar,
        identity -> Varchar,
        recommander -> Nullable<Varchar>,
        recommander_name -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    form_repairs (id) {
        id -> Integer,
        hospital_name -> Varchar,
        city -> Varchar,
        name -> Varchar,
        contact -> Text,
        prod -> Text,
        model -> Text,
        serial -> Nullable<Text>,
        year -> Nullable<Varchar>,
        month -> Nullable<Varchar>,
        question -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    form_trails (id) {
        id -> Integer,
        hospital_name -> Varchar,
        applicant -> Varchar,
        city -> Varchar,
        user -> Varchar,
        contact -> Text,
        prod -> Text,
        model -> Text,
        month -> Varchar,
        day -> Varchar,
        bmonth -> Varchar,
        bday -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    form_buys,
    form_register_bus,
    form_register_pss,
    form_repairs,
    form_trails,
);
