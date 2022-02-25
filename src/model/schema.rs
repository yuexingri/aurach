table! {
    aurach_user (id) {
        id -> Nullable<Integer>,
        name -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(aurach_user,);