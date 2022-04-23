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
    });
});