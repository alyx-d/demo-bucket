fetch("http://localhost:8000")
    .then(res => res.text())
    .then(text => console.log(text));