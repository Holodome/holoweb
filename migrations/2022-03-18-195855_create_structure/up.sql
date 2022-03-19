-- Your SQL goes here
-- This file should undo anything in `up.sql`
create table users (
    id varchar primary key not null,
    name text not null,
    email text not null,
    password varchar not null,
    created_at text not null,
    role text not null,
    is_banned boolean not null
);

create table blog_posts (
    id varchar primary key not null,
    title text unique not null,
    brief text unique,
    contents text not null,
    author_id varchar not null,

    foreign key (author_id) references users(id)
);

create table comments (
    id varchar primary key not null,
    author_id varchar not null,
    post_id varchar not null,

    parent_id varchar,
    contents text not null,

    foreign key (author_id) references users(id),
    foreign key (post_id) references blog_posts(id)
);

create table projects (
    id varchar primary key not null,
    title text not null,
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