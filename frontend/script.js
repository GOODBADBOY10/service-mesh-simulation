const GATEWAY_URL = 'http://localhost:3002';
let token = localStorage.getItem('token');
let currentUser = JSON.parse(localStorage.getItem('currentUser') || 'null');
let userProfile = null;

document.addEventListener('DOMContentLoaded', () => {
    checkServiceStatus();
    if (token && currentUser) {
        showDashboard();
        loadProfile();
    }
    setInterval(checkServiceStatus, 30000);
});

async function checkServiceStatus() {
    try {
        const response = await fetch(`${GATEWAY_URL}/`);
        document.getElementById('gatewayStatus').classList.remove('offline');
        document.getElementById('authStatus').classList.remove('offline');
        document.getElementById('userStatus').classList.remove('offline');
    } catch (e) {
        document.getElementById('gatewayStatus').classList.add('offline');
        document.getElementById('authStatus').classList.add('offline');
        document.getElementById('userStatus').classList.add('offline');
    }
}

function showAlert(message, type = 'info') {
    const container = document.getElementById('alertContainer');
    const alert = document.createElement('div');
    alert.className = `alert alert-${type}`;
    alert.innerHTML = `<span>${message}</span>`;
    container.appendChild(alert);
    setTimeout(() => alert.remove(), 5000);
}

function showLogin() {
    document.getElementById('loginCard').classList.remove('hidden');
    document.getElementById('registerCard').classList.add('hidden');
}

function showRegister() {
    document.getElementById('loginCard').classList.add('hidden');
    document.getElementById('registerCard').classList.remove('hidden');
}

async function handleRegister(e) {
    e.preventDefault();
    const username = document.getElementById('regUsername').value;
    const email = document.getElementById('regEmail').value;
    const password = document.getElementById('regPassword').value;
    try {
        const response = await fetch(`${GATEWAY_URL}/api/register`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ username, email, password })
        });
        const data = await response.json();
        if (response.ok) {
            showAlert('Registration successful! Please login.', 'success');
            showLogin();
            document.getElementById('loginUsername').value = username;
        } else {
            showAlert(data.message || 'Registration failed', 'error');
        }
    } catch (e) {
        showAlert('Failed to connect to server', 'error');
    }
}

async function handleLogin(e) {
    e.preventDefault();
    const username = document.getElementById('loginUsername').value;
    const password = document.getElementById('loginPassword').value;
    try {
        const response = await fetch(`${GATEWAY_URL}/api/login`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ username, password })
        });
        const data = await response.json();
        if (response.ok) {
            token = data.token;
            localStorage.setItem('token', token);
            const payload = JSON.parse(atob(token.split('.')[1]));
            currentUser = { user_id: payload.sub, username: payload.username };
            localStorage.setItem('currentUser', JSON.stringify(currentUser));
            showAlert('Login successful!', 'success');
            showDashboard();
            loadProfile();
        } else {
            showAlert(data.message || 'Login failed', 'error');
        }
    } catch (e) {
        showAlert('Failed to connect to server', 'error');
    }
}

function logout() {
    token = null;
    currentUser = null;
    userProfile = null;
    localStorage.removeItem('token');
    localStorage.removeItem('currentUser');
    showAuth();
    showAlert('Logged out successfully', 'info');
}

function showDashboard() {
    document.getElementById('authSection').classList.add('hidden');
    document.getElementById('dashboardSection').classList.remove('hidden');
    document.getElementById('userInfo').classList.remove('hidden');
    document.getElementById('userBadge').textContent = `👤 ${currentUser.username}`;
}

function showAuth() {
    document.getElementById('authSection').classList.remove('hidden');
    document.getElementById('dashboardSection').classList.add('hidden');
    document.getElementById('userInfo').classList.add('hidden');
}

function showPanel(panel, btn) {
    document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
    document.querySelectorAll('.panel').forEach(p => p.classList.remove('active'));
    btn.classList.add('active');
    document.getElementById(`${panel}Panel`).classList.add('active');
    if (panel === 'users') loadUsers();
}

async function loadProfile() {
    try {
        const response = await fetch(`${GATEWAY_URL}/api/users/${currentUser.user_id}`, {
            headers: { 'Authorization': `Bearer ${token}` }
        });
        if (response.ok) {
            userProfile = await response.json();
            renderProfile();
        } else {
            renderNoProfile();
        }
    } catch (e) {
        renderNoProfile();
    }
}

function renderProfile() {
    document.getElementById('profileContent').innerHTML = `
                <div style="display: grid; gap: 1rem;">
                    <div style="display: flex; justify-content: space-between; align-items: start;">
                        <div>
                            <h3 style="font-size: 1.5rem; margin-bottom: 0.5rem;">${userProfile.full_name}</h3>
                            <p style="color: var(--muted);">@${userProfile.username}</p>
                        </div>
                        <div style="display: flex; gap: 0.5rem;">
                            <button class="btn btn-secondary" onclick="showEditModal()">✏️ Edit</button>
                            <button class="btn btn-danger" onclick="showDeleteModal()">🗑️ Delete</button>
                        </div>
                    </div>
                    <div style="background: var(--dark); padding: 1rem; border-radius: 8px;">
                        <p style="color: var(--muted); font-size: 0.85rem; margin-bottom: 0.25rem;">Email</p>
                        <p>${userProfile.email}</p>
                    </div>
                    <div style="background: var(--dark); padding: 1rem; border-radius: 8px;">
                        <p style="color: var(--muted); font-size: 0.85rem; margin-bottom: 0.25rem;">Bio</p>
                        <p>${userProfile.bio || 'No bio provided'}</p>
                    </div>
                    <div style="background: var(--dark); padding: 1rem; border-radius: 8px;">
                        <p style="color: var(--muted); font-size: 0.85rem; margin-bottom: 0.25rem;">Member Since</p>
                        <p>${new Date(userProfile.created_at).toLocaleDateString()}</p>
                    </div>
                </div>`;
    document.getElementById('profileFormCard').classList.add('hidden');
}

function renderNoProfile() {
    document.getElementById('profileContent').innerHTML = `
                <div class="empty-state">
                    <div class="icon">📋</div>
                    <p>No profile created yet</p>
                    <button class="btn btn-primary" style="margin-top: 1rem" onclick="showCreateProfile()">Create Profile</button>
                </div>`;
}

function showCreateProfile() {
    document.getElementById('profileFormCard').classList.remove('hidden');
    document.getElementById('profileFormTitle').textContent = '✨ Create Profile';
    document.getElementById('profileEmail').value = '';
    document.getElementById('profileFullName').value = '';
    document.getElementById('profileBio').value = '';
}

function hideProfileForm() {
    document.getElementById('profileFormCard').classList.add('hidden');
}

async function handleSaveProfile(e) {
    e.preventDefault();
    const profileData = {
        user_id: currentUser.user_id,
        username: currentUser.username,
        email: document.getElementById('profileEmail').value,
        full_name: document.getElementById('profileFullName').value,
        bio: document.getElementById('profileBio').value
    };
    try {
        const response = await fetch(`${GATEWAY_URL}/api/users`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json', 'Authorization': `Bearer ${token}` },
            body: JSON.stringify(profileData)
        });
        if (response.ok) {
            showAlert('Profile created successfully!', 'success');
            loadProfile();
        } else {
            const data = await response.json();
            showAlert(data.message || 'Failed to create profile', 'error');
        }
    } catch (e) {
        showAlert('Failed to connect to server', 'error');
    }
}

function showEditModal() {
    document.getElementById('editFullName').value = userProfile.full_name;
    document.getElementById('editBio').value = userProfile.bio || '';
    document.getElementById('editModal').classList.add('active');
}

function closeModal() {
    document.getElementById('editModal').classList.remove('active');
}

async function handleUpdateProfile(e) {
    e.preventDefault();
    const updateData = {
        full_name: document.getElementById('editFullName').value,
        bio: document.getElementById('editBio').value
    };
    try {
        const response = await fetch(`${GATEWAY_URL}/api/users/${currentUser.user_id}`, {
            method: 'PUT',
            headers: { 'Content-Type': 'application/json', 'Authorization': `Bearer ${token}` },
            body: JSON.stringify(updateData)
        });
        if (response.ok) {
            showAlert('Profile updated successfully!', 'success');
            closeModal();
            loadProfile();
        } else {
            const data = await response.json();
            showAlert(data.message || 'Failed to update profile', 'error');
        }
    } catch (e) {
        showAlert('Failed to connect to server', 'error');
    }
}

function showDeleteModal() {
    document.getElementById('deleteModal').classList.add('active');
}

function closeDeleteModal() {
    document.getElementById('deleteModal').classList.remove('active');
}

async function confirmDelete() {
    try {
        const response = await fetch(`${GATEWAY_URL}/api/users/${currentUser.user_id}`, {
            method: 'DELETE',
            headers: { 'Authorization': `Bearer ${token}` }
        });
        if (response.ok) {
            showAlert('Profile deleted successfully!', 'success');
            closeDeleteModal();
            userProfile = null;
            renderNoProfile();
        } else {
            const data = await response.json();
            showAlert(data.message || 'Failed to delete profile', 'error');
        }
    } catch (e) {
        showAlert('Failed to connect to server', 'error');
    }
}

async function loadUsers() {
    const container = document.getElementById('usersContent');
    container.innerHTML = '<div class="loading"></div>';
    try {
        const response = await fetch(`${GATEWAY_URL}/api/users`, {
            headers: { 'Authorization': `Bearer ${token}` }
        });
        if (response.ok) {
            const users = await response.json();
            if (users.length === 0) {
                container.innerHTML = `<div class="empty-state"><div class="icon">👥</div><p>No users found</p></div>`;
                return;
            }
            container.innerHTML = `
                        <table class="users-table">
                            <thead><tr><th>Username</th><th>Full Name</th><th>Email</th><th>Bio</th><th>Joined</th></tr></thead>
                            <tbody>${users.map(user => `
                                <tr>
                                    <td><strong>@${user.username}</strong></td>
                                    <td>${user.full_name}</td>
                                    <td>${user.email}</td>
                                    <td style="max-width: 200px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">${user.bio || '-'}</td>
                                    <td>${new Date(user.created_at).toLocaleDateString()}</td>
                                </tr>`).join('')}
                            </tbody>
                        </table>`;
        } else {
            const data = await response.json();
            container.innerHTML = `<div class="alert alert-error">${data.message || 'Failed to load users'}</div>`;
        }
    } catch (e) {
        container.innerHTML = `<div class="alert alert-error">Failed to connect to server</div>`;
    }
}

async function testApi() {
    const endpoint = document.getElementById('apiEndpoint').value;
    const responseViewer = document.getElementById('apiResponse');
    responseViewer.textContent = 'Loading...';
    const [method, path] = endpoint.split(' ');
    try {
        let options = { method: method, headers: { 'Authorization': `Bearer ${token}` } };
        if (method === 'POST' && path === '/api/validate') {
            options.headers['Content-Type'] = 'application/json';
            options.body = JSON.stringify({ token: token });
        }
        const response = await fetch(`${GATEWAY_URL}${path}`, options);
        const data = await response.json();
        responseViewer.textContent = JSON.stringify({ status: response.status, statusText: response.statusText, data: data }, null, 2);
    } catch (e) {
        responseViewer.textContent = JSON.stringify({ error: 'Failed to connect to server', message: e.message }, null, 2);
    }
}