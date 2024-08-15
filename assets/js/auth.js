function showErrorPopup(message) {
    document.getElementById('error-message').textContent = message;
    document.getElementById('error-popup').style.display = 'block';
}

function closeErrorPopup() {
    document.getElementById('error-popup').style.display = 'none';
}