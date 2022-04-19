use crate::common::{TestDB, TestProject, TestUser};
use claim::{assert_err, assert_ok, assert_some};
use holosite::domain::projects::{NewProject, ProjectID, ProjectVisibility, UpdateProject};
use holosite::services::{
    get_all_projects, get_project_by_id, get_project_by_title, insert_new_project, update_project,
    BlogPostError, ProjectError,
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
    let post_id = test_project.register_internally(db.pool(), &user_id);

    let res = get_project_by_id(db.pool(), &post_id);
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
        title: Some(&"New Title"),
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
        brief: Some(&"New Brief"),
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
