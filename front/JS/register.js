document.addEventListener('DOMContentLoaded', () => {
    const registerForm = document.getElementById('register-form');
    const message = document.getElementById('message');

    registerForm.addEventListener('submit', (event) => {
        event.preventDefault();

        const email = document.getElementById('email').value;
        const password = document.getElementById('password').value;
        const confirmPassword = document.getElementById('confirm-password').value;

        if (password !== confirmPassword) {
            message.textContent = '密码不匹配!';
            return;
        }

        // 模拟注册成功
        message.textContent = '注册成功!';

        // 清空表单字段
        registerForm.reset();
    });
});