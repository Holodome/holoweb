$(() => {
    let checkbox = $("#show_password_checkbox");
    change_password_visibility(checkbox, $("#password_input"))
    change_password_visibility(checkbox, $("#repeat_password_input"));
});
