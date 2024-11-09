fetch("https://www.acfun.cn").then((resp) => {
    return resp.text();
}).then((body) => {
    console.log(body);
});
