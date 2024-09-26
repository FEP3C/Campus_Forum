async function fetchUsers() {
    const token = localStorage.getItem('token');

    const response = await fetch('/admin/users', {
        headers: {
            'Authorization': `Bearer ${token}`,
        },
    });

    const users = await response.json();
    const tableBody = document.getElementById('user-table-body');

    users.forEach(user => {
        const row = `<tr>
            <td>${user.username}</td>
            <td>${user.email}</td>
            <td>${user.role}</td>
            <td><button>Delete</button></td>
        </tr>`;
        tableBody.innerHTML += row;
    });
}

fetchUsers();