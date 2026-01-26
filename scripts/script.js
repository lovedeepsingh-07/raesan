window.send_api_request = function () {
  fetch("/api")
    .then((res) => res.text())
    .then((data) => console.log(data));
};
