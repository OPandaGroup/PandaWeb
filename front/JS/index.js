document.addEventListener('DOMContentLoaded', () => {
    const loginBtn = document.getElementById('login-btn');
    const registerBtn = document.getElementById('register-btn');
    const modal = document.getElementById('modal');
    const closeBtn = document.querySelector('.close-btn');
    const modalBody = document.getElementById('modal-body');

    const showModal = (content) => {
        modalBody.innerHTML = content;
        modal.style.display = 'flex';
    };

    loginBtn.addEventListener('click', () => {
        showModal(`
            <h2>登录</h2>
            <form>
                <label for="login-email">邮箱:</label>
                <input type="email" id="login-email" required>
                <label for="login-password">密码:</label>
                <input type="password" id="login-password" required>
                <button type="submit">登录</button>
            </form>
        `);
    });

    registerBtn.addEventListener('click', () => {
        showModal(`
            <h2>注册</h2>
            <form>
                <label for="register-email">邮箱:</label>
                <input type="email" id="register-email" required>
                <label for="register-password">密码:</label>
                <input type="password" id="register-password" required>
                <button type="submit">注册</button>
            </form>
        `);
    });

    closeBtn.addEventListener('click', () => {
        modal.style.display = 'none';
    });

    window.addEventListener('click', (event) => {
        if (event.target === modal) {
            modal.style.display = 'none';
        }
    });
});
