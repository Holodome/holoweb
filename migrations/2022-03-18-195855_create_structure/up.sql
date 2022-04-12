-- Your SQL goes here
-- This file should undo anything in `up.sql`
create table users (
    id varchar primary key not null,
    name text unique not null,
    email text unique not null,
    password varchar not null,
    password_salt varchar not null,
    created_at text not null,
    is_banned boolean not null
);

create table blog_posts (
    id varchar primary key not null,
    title text unique not null,
    brief text not null,
    contents text not null,
    author_id varchar not null,
    created_at text not null,

    foreign key (author_id) references users(id)
);

create table comments (
    id varchar primary key not null,
    author_id varchar not null,
    post_id varchar not null,

    parent_id varchar,
    contents text not null,
    created_at text not null,
    is_deleted boolean not null,
    main_parent_id varchar,
    depth integer not null,

    foreign key (author_id) references users(id),
    foreign key (post_id) references blog_posts(id),
    foreign key (parent_id) references comments(id),
    foreign key (main_parent_id) references comments(id)
);

create table projects (
    id varchar primary key not null,
    title text unique not null,
    brief text not null,

    author_id varchar not null,

    foreign key (author_id) references users(id)
);

create table project_blog_post_junctions (
    project_id varchar not null,
    post_id varchar not null,

    constraint pk primary key (
        project_id, post_id
    ),

    foreign key (project_id) references projects(id),
    foreign key (post_id) references blog_posts(id)
);