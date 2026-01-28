const baseUrl = "https://clipspicks.onrender.com";

fetch(`${baseUrl}/health`).then((res) => {
    if (!res.ok) alert("server offline");

    console.log("server healthy");
});

document.addEventListener("DOMContentLoaded", () => {
    let oauth = document.getElementById("tiktok-oauth");
    oauth.onclick = () => {
        window.location.replace(`${baseUrl}/oauth`);
    };
});
