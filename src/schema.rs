table! {
    blog_posts (id) {
        id -> Text,
        title -> Text,
        brief -> Text,
        contents -> Text,
        author_id -> Text,
        created_at -> Text,
        updated_at -> Text,
        visibility -> Text,
    }
}

table! {
    check_if_migrated (whatever) {
        whatever -> Integer,
    }
}

table! {
    comments (id) {
        id -> Text,
        contents -> Text,
        author_id -> Text,
        post_id -> Text,
        reply_to_id -> Nullable<Text>,
        created_at -> Text,
        updated_at -> Text,
        is_deleted -> Bool,
    }
}

table! {
    project_blog_post_junctions (project_id, post_id) {
        project_id -> Text,
        post_id -> Text,
    }
}

table! {
    project_editor_junctions (project_id, user_id) {
        project_id -> Text,
        user_id -> Text,
    }
}

table! {
    projects (id) {
        id -> Text,
        title -> Text,
        brief -> Text,
        author_id -> Text,
        visibility -> Text,
    }
}

table! {
    users (id) {
        id -> Text,
        name -> Text,
        email -> Text,
        created_at -> Text,
        password -> Text,
        password_salt -> Text,
        is_banned -> Bool,
        role -> Text,
    }
}

joinable!(blog_posts -> users (author_id));
joinable!(comments -> blog_posts (post_id));
joinable!(comments -> users (author_id));
joinable!(project_blog_post_junctions -> blog_posts (post_id));
joinable!(project_blog_post_junctions -> projects (project_id));
joinable!(project_editor_junctions -> projects (project_id));
joinable!(project_editor_junctions -> users (user_id));
joinable!(projects -> users (author_id));

allow_tables_to_appear_in_same_query!(
    blog_posts,
    check_if_migrated,
    comments,
    project_blog_post_junctions,
    project_editor_junctions,
    projects,
    users,
);
