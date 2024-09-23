// static/login.js

const form = document.getElementById("login-form");
form.addEventListener("submit", async (event) => {
    event.preventDefault();
    const username = document.getElementById("username").value;
    const password = document.getElementById("password").value;

    try {
        const response = await fetch("/login", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({ username, password }),
        });

        const data = await response.json();

        if (response.ok) {
            // 保存 JWT token 到 localStorage
            localStorage.setItem("token", data.token);
            document.getElementById("message").textContent = "Login successful!";
        } else {
            document.getElementById("message").textContent = "Login failed: " + data.message;
        }
    } catch (error) {
        console.error("Error:", error);
        document.getElementById("message").textContent = "An error occurred during login.";
    }
});
