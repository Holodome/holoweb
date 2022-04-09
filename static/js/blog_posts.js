pagination_function = n => {
    return () => {
        let query = get_query();
        query["size"] = n;
        redirect_on_same_page_with_query(query);
    };
}

window.onload = () => {
    $("#paginate_10").onclick = pagination_function(10);
    $("#paginate_20").onclick = pagination_function(20);
    $("#paginate_40").onclick = pagination_function(40);
}
