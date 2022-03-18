table! {
    blog_posts (id) {
        id -> Text,
        title -> Text,
        brief -> Nullable<Text>,
        contents -> Text,
        author_id -> Text,
    }
}

table! {
    comments (id) {
        id -> Text,
        author_id -> Text,
        post_id -> Text,
        parent_id -> Text,
        contents -> Text,
    }
}

table! {
    project_blog_post_junctions (project_id, post_id) {
        project_id -> Text,
        post_id -> Text,
    }
}

table! {
    projects (id) {
        id -> Text,
        name -> Text,
        brief -> Text,
        author_id -> Text,
    }
}

table! {
    users (id) {
        id -> Text,
        name -> Text,
        email -> Text,
        password -> Text,
        created_at -> Integer,
        role -> Integer,
        status -> Integer,
    }
}

joinable!(blog_posts -> users (author_id));
joinable!(comments -> blog_posts (post_id));
joinable!(comments -> users (author_id));
joinable!(project_blog_post_junctions -> blog_posts (post_id));
joinable!(project_blog_post_junctions -> projects (project_id));
joinable!(projects -> users (author_id));

allow_tables_to_appear_in_same_query!(
    blog_posts,
    comments,
    project_blog_post_junctions,
    projects,
    users,
);
