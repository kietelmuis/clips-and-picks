const baseUrl = "https://whatliked.onrender.com";

fetch(`${baseUrl}/health`).then((res) => {
    if (!res.ok) alert("server offline");

    console.log("server healthy");
});
