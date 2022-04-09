get_query = () => {
    const urlSearchParams = new URLSearchParams(window.location.search);
    return Object.fromEntries(urlSearchParams.entries());
}

redirect_on_same_page_with_query = query => {
    window.location.href = window.location.origin + window.location.pathname +
        "?" + new URLSearchParams(query).toString();
}

change_password_visibility = (obj, input) => {
    obj.click(() => {
        console.log("Hello");
        if (input.attr("type") === "password") {
            input.prop("type", "text");
        } else {
            input.prop("type", "password");
        }
    });
}