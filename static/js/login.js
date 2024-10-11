document.getElementById('login-form').addEventListener('submit', async function (e) {
    e.preventDefault();

    const username = document.getElementById('username').value;
    const password = document.getElementById('password').value;

    try {
        const response = await fetch('/login', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ username, password }),
        });

        // 检查响应状态是否为成功
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        const data = await response.json();

        // 假设这里不需要对data.token进行额外的操作，直接删除该行
        // data.token = ;

        if (data.token) {
            localStorage.setItem('token', data.token);
            localStorage.setItem('username', data.username);
            window.location.href = '/home';
        } else {
            // 可以替换这里的alert为其他用户反馈机制
            alert('Login failed. Please try again.');
        }
    } catch (error) {
        console.error('Error during login:', error);
        alert('An unexpected error occurred. Please try again later.');
    }
});
