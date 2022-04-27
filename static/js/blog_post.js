get_comment_id_from_reply = reply_id => {
    return reply_id.replace("reply-comment-", "");
}

$(document).ready(() => {
    $( ".comment-reply-button" ).click(e => {
        e.preventDefault();
        let anchor_id = e.target.id;
        let comment_id = get_comment_id_from_reply(anchor_id);
        // Add form to current comment
        let form = $("#comment-reply-form");
        form.show();
        $( "#comment-" + comment_id ).after(form);
        // Set id of reply
        $( "#comment-reply-form-id" ).val(comment_id);
    })
    ;

    $('.ui.dropdown')
        .dropdown()
    ;

    $( ".edit-comment-button" ).click(e => {
        e.preventDefault();

        let comment_id = e.target.id.replace("edit-comment-", "");

        let form = $( "#edit-comment-form" );
        let form_action = form.attr("action");
        if (form_action !== undefined && form_action !== "") {
            let regex = /comments\/(.*)\/edit/;
            let old_comment_id = form_action.match(regex)[1];
            $( "#comment-contents-paragraph-" + old_comment_id ).show();
            if (old_comment_id === comment_id && form.is(":visible")) {
                form.attr("action", "");
                form.hide();
                return;
            }
        }

        form.show();
        let current_path = window.location.pathname.replace("/view", "");
        form.attr("action", current_path + "/comments/" + comment_id + "/edit")

        let paragraph = $( "#comment-contents-paragraph-" + comment_id );
        paragraph.hide();

        $( "#edit-comment-form-contents" ).val(paragraph.text().trim());
        $( "#comment-contents-" + comment_id ).after(form);
    })
    ;

    $( ".delete-comment-button" ).click(e => {
        e.preventDefault();

        let comment_id = e.target.id.replace("delete-comment-", "");

        let current_path = window.location.pathname;
        window.location.href = window.location.origin + current_path.replace("/view", "/comments/")
            + comment_id + "/delete";
    })
    ;
});
