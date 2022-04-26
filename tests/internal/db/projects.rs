use crate::common::{TestBlogPost, TestDB, TestProject, TestUser};
use claim::{assert_err, assert_ok, assert_some};
use holosite::domain::projects::{NewProject, ProjectID, ProjectVisibility, UpdateProject};
use holosite::services::{
    add_project_blog_post, add_project_editor, get_all_projects, get_project_blog_post_ids,
    get_project_by_id, get_project_by_title, get_project_editor_ids, insert_new_project,
    remove_project_editor, update_project, ProjectError,
};

#[test]
fn add_new_project_works() {
    let db = TestDB::spawn();
    let test_project = TestProject::generate();
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(db.pool());

    let res = insert_new_project(
        db.pool(),
        &NewProject {
            title: &test_project.title,
            brief: &test_project.brief,
            author_id: &user_id,
            visibility: ProjectVisibility::All,
        },
    );
    assert_ok!(&res);
    let res = res.unwrap();
    assert_eq!(res.title, test_project.title);
    assert_eq!(res.brief, test_project.brief);
}

#[test]
fn add_project_and_get_it_by_id_works() {
    let db = TestDB::spawn();
    let test_project = TestProject::generate();
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(db.pool());
    let project_id = test_project.register_internally(db.pool(), &user_id);

    let res = get_project_by_id(db.pool(), &project_id);
    assert_ok!(&res);
    let res = res.unwrap();
    assert_some!(&res);
    let res = res.unwrap();
    assert_eq!(res.title, test_project.title);
    assert_eq!(res.brief, test_project.brief);
}

#[test]
fn add_project_and_get_it_by_title_works() {
    let db = TestDB::spawn();
    let test_project = TestProject::generate();
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(db.pool());
    test_project.register_internally(db.pool(), &user_id);

    let res = get_project_by_title(db.pool(), &test_project.title);
    assert_ok!(&res);
    let res = res.unwrap();
    assert_some!(&res);
    let res = res.unwrap();
    assert_eq!(res.title, test_project.title);
    assert_eq!(res.brief, test_project.brief);
}

#[test]
fn update_project_title_works() {
    let db = TestDB::spawn();
    let test_project = TestProject::generate();
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(db.pool());
    let post_id = test_project.register_internally(db.pool(), &user_id);

    let changeset = UpdateProject {
        id: &post_id,
        title: Some("New Title"),
        brief: None,
        visibility: None,
    };
    let res = update_project(db.pool(), &changeset);
    assert_ok!(res);

    let res = get_project_by_id(db.pool(), &post_id);
    assert_ok!(&res);
    let res = res.unwrap();
    assert_some!(&res);
    let res = res.unwrap();
    assert_eq!(res.title, "New Title");
    assert_eq!(res.brief, test_project.brief);
}

#[test]
fn cant_change_project_title_to_taken_title() {
    let db = TestDB::spawn();
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(db.pool());

    let test_project = TestProject::generate();
    let post_id = test_project.register_internally(db.pool(), &user_id);
    let other_post = TestProject::generate();
    other_post.register_internally(db.pool(), &user_id);

    let res = update_project(
        db.pool(),
        &UpdateProject {
            id: &post_id,
            title: Some(&other_post.title),
            brief: None,
            visibility: None,
        },
    );
    assert_err!(&res);
    let res = res.unwrap_err();
    match res {
        ProjectError::TakenTitle => {}
        _ => panic!("Incorrect error type: got {:?}", res),
    };
}

#[test]
fn change_project_brief_works() {
    let db = TestDB::spawn();
    let test_project = TestProject::generate();
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(db.pool());
    let post_id = test_project.register_internally(db.pool(), &user_id);

    let changeset = UpdateProject {
        id: &post_id,
        title: None,
        brief: Some("New Brief"),
        visibility: None,
    };
    let res = update_project(db.pool(), &changeset);
    assert_ok!(res);

    let res = get_project_by_id(db.pool(), &post_id);
    assert_ok!(&res);
    let res = res.unwrap();
    assert_some!(&res);
    let res = res.unwrap();
    assert_eq!(res.title, test_project.title);
    assert_eq!(res.brief, "New Brief");
}

#[test]
fn cant_add_new_project_with_taken_title() {
    let db = TestDB::spawn();
    let test_project = TestProject::generate();
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(db.pool());
    test_project.register_internally(db.pool(), &user_id);

    let res = insert_new_project(
        db.pool(),
        &NewProject {
            title: &test_project.title,
            brief: &test_project.brief,
            author_id: &user_id,
            visibility: ProjectVisibility::All,
        },
    );
    assert_err!(&res);
    let res = res.unwrap_err();
    match res {
        ProjectError::TakenTitle => {}
        _ => panic!("Incorrect error type: got {:?}", res),
    };
}

#[test]
fn get_all_projects_works() {
    let db = TestDB::spawn();
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(db.pool());
    let post_ids: Vec<ProjectID> = (0..100)
        .map(|_| {
            let test_project = TestProject::generate();
            test_project.register_internally(db.pool(), &user_id)
        })
        .collect();

    let res = get_all_projects(db.pool());
    assert_ok!(&res);
    let res = res.unwrap();
    assert_eq!(res.len(), post_ids.len());
    for i in 0..res.len() {
        assert_eq!(res[i].id, post_ids[i]);
    }
}

#[test]
fn test_project_editors_workflow() {
    let db = TestDB::spawn();
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(db.pool());
    let test_project = TestProject::generate();
    let project_id = test_project.register_internally(db.pool(), &user_id);

    let project_editors = get_project_editor_ids(db.pool(), &project_id);
    assert_ok!(&project_editors);
    let project_editors = project_editors.unwrap();
    assert_eq!(project_editors.len(), 1);
    assert_eq!(project_editors[0], user_id);

    let other_user = TestUser::generate();
    let other_user_id = other_user.register_internally(db.pool());
    add_project_editor(db.pool(), &project_id, &other_user_id).unwrap();

    let project_editors = get_project_editor_ids(db.pool(), &project_id);
    assert_ok!(&project_editors);
    let project_editors = project_editors.unwrap();
    assert_eq!(project_editors.len(), 2);

    remove_project_editor(db.pool(), &project_id, &other_user_id).unwrap();
    let project_editors = get_project_editor_ids(db.pool(), &project_id);
    assert_ok!(&project_editors);
    let project_editors = project_editors.unwrap();
    assert_eq!(project_editors.len(), 1);
    assert_eq!(project_editors[0], user_id);
}

#[test]
fn test_project_blog_posts_workflow() {
    let db = TestDB::spawn();
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(db.pool());
    let test_project = TestProject::generate();
    let project_id = test_project.register_internally(db.pool(), &user_id);
    let blog_post = TestBlogPost::generate();
    let blog_post_id = blog_post.register_internally(db.pool(), &user_id);

    add_project_blog_post(db.pool(), &project_id, &blog_post_id).unwrap();

    let blog_posts = get_project_blog_post_ids(db.pool(), &project_id).unwrap();
    assert_eq!(blog_posts.len(), 1);
    assert_eq!(blog_posts[0], blog_post_id);
}
