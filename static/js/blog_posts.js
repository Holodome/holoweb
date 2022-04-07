get_query = () => {
    const urlSearchParams = new URLSearchParams(window.location.search);
    return Object.fromEntries(urlSearchParams.entries());
}

redirect_on_same_page_with_query = query => {
    window.location.href = window.location.origin + window.location.pathname +
        "?" + new URLSearchParams(query).toString();
}

pagination_function = n => {
    return () => {
        let query = get_query();
        query["size"] = n;
        redirect_on_same_page_with_query(query);
    };
}

window.onload = () => {
    document.getElementById("paginate_10").onclick = pagination_function(10);
    document.getElementById("paginate_20").onclick = pagination_function(20);
    document.getElementById("paginate_40").onclick = pagination_function(40);
}
