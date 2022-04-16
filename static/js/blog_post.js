get_comment_id_from_reply = reply_id => {
    return reply_id.replace("reply-", "");
}

$(() => {
    const FORM = $("#reply-form");

    $( ".comment-reply-button" ).each(() => {
        let it = $(this);
        it.click(e => {
            e.preventDefault();
            console.log(it.attr("class"), it.attr("id"));
            $( get_comment_id_from_reply(it.id) ).after(FORM)
        })
    });
});