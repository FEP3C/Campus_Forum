document.getElementById("loginForm").addEventListener("submit", async function(event) {
    event.preventDefault();  // 防止表单提交刷新页面

    // 获取用户输入的用户名和密码
    const username = document.getElementById("username").value;
    const password = document.getElementById("password").value;

    try {
        // 向后端发送 POST 请求，携带用户名和密码
        const response = await fetch("/login", {
            method: "POST",
            headers: { 
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ username, password })  // 转换为 JSON 格式
        });

        // 解析服务器的响应
        const result = await response.json();

        if (response.ok) {
            // 登录成功，显示 token 并保存到浏览器的 localStorage
            alert("Login successful! Token: " + result.token);
            localStorage.setItem("token", result.token);  // 存储 JWT 到本地
        } else {
            // 登录失败，显示错误信息
            alert("Login failed: " + result.message);
        }
    } catch (error) {
        // 处理请求失败的情况
        console.error("Error during login:", error);
        alert("An error occurred during login.");
    }
});
