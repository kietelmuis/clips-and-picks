const baseUrl = "https://clipspicks.onrender.com";

fetch(`${baseUrl}/health`).then((res) => {
    if (!res.ok) alert("server offline");

    console.log("server healthy");
});
