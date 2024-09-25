document.getElementById("loginForm").addEventListener("submit", async function(event) {
    event.preventDefault();  // 防止表单提交刷新页面

    // 获取用户输入的用户名和密码
    const username = document.getElementById("username").value;
    const password = document.getElementById("password").value;

    try {
        const response = await fetch("/login", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ username, password })
        });

        const result = await response.json();
        if (response.ok) {
            alert("Login successful! Token: " + result.token);
            localStorage.setItem("token", result.token);  // 存储 JWT 到本地
        } else {
            alert("Login failed: " + result.message);  // 显示失败信息
        }
    } catch (error) {
        console.error("Error during login:", error);
        alert("An error occurred during login. Check the console for details.");
    }
});
