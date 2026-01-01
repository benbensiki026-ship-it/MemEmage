// API Configuration
const API_BASE_URL = 'http://localhost:8080/api';

// State Management
let currentUser = null;
let authToken = null;
let selectedTemplate = null;

// Initialize app
document.addEventListener('DOMContentLoaded', () => {
    checkAuth();
    loadMemes();
});

// Page Navigation
function showPage(pageName) {
    document.querySelectorAll('.page').forEach(page => {
        page.classList.remove('active');
    });
    
    const pageElement = document.getElementById(`${pageName}Page`);
    if (pageElement) {
        pageElement.classList.add('active');
    }
    
    // Load data for specific pages
    if (pageName === 'myMemes' && currentUser) {
        loadUserMemes();
    } else if (pageName === 'home') {
        loadMemes();
    }
}

// Authentication
function checkAuth() {
    authToken = localStorage.getItem('authToken');
    const userData = localStorage.getItem('userData');
    
    if (authToken && userData) {
        currentUser = JSON.parse(userData);
        updateUIForAuthenticatedUser();
    }
}

function updateUIForAuthenticatedUser() {
    document.getElementById('authButtons').style.display = 'none';
    document.getElementById('userMenu').style.display = 'flex';
    document.getElementById('myMemesLink').style.display = 'block';
    document.getElementById('username').textContent = currentUser.username;
}

function updateUIForUnauthenticatedUser() {
    document.getElementById('authButtons').style.display = 'flex';
    document.getElementById('userMenu').style.display = 'none';
    document.getElementById('myMemesLink').style.display = 'none';
}

async function handleSignup(event) {
    event.preventDefault();
    
    const username = document.getElementById('signupUsername').value;
    const email = document.getElementById('signupEmail').value;
    const password = document.getElementById('signupPassword').value;
    
    try {
        const response = await fetch(`${API_BASE_URL}/auth/signup`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ username, email, password }),
        });
        
        const result = await response.json();
        
        if (result.success && result.data) {
            authToken = result.data.token;
            currentUser = result.data.user;
            
            localStorage.setItem('authToken', authToken);
            localStorage.setItem('userData', JSON.stringify(currentUser));
            
            updateUIForAuthenticatedUser();
            showPage('home');
            alert('Welcome to MemEmage! 🎉');
        } else {
            alert(result.error || 'Signup failed');
        }
    } catch (error) {
        console.error('Signup error:', error);
        alert('Network error. Please try again.');
    }
}

async function handleLogin(event) {
    event.preventDefault();
    
    const username = document.getElementById('loginUsername').value;
    const password = document.getElementById('loginPassword').value;
    
    try {
        const response = await fetch(`${API_BASE_URL}/auth/login`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ username, password }),
        });
        
        const result = await response.json();
        
        if (result.success && result.data) {
            authToken = result.data.token;
            currentUser = result.data.user;
            
            localStorage.setItem('authToken', authToken);
            localStorage.setItem('userData', JSON.stringify(currentUser));
            
            updateUIForAuthenticatedUser();
            showPage('home');
            alert('Welcome back! 👋');
        } else {
            alert(result.error || 'Login failed');
        }
    } catch (error) {
        console.error('Login error:', error);
        alert('Network error. Please try again.');
    }
}

function logout() {
    localStorage.removeItem('authToken');
    localStorage.removeItem('userData');
    authToken = null;
    currentUser = null;
    updateUIForUnauthenticatedUser();
    showPage('home');
}

// Meme Functions
function updatePreview() {
    const topText = document.getElementById('topText').value;
    const bottomText = document.getElementById('bottomText').value;
    
    document.getElementById('previewTopText').textContent = topText;
    document.getElementById('previewBottomText').textContent = bottomText;
}

function selectTemplate(templateName) {
    selectedTemplate = templateName;
    
    // Update UI
    document.querySelectorAll('.template-option').forEach(option => {
        option.classList.remove('selected');
    });
    event.target.closest('.template-option').classList.add('selected');
}

async function createMeme(event) {
    event.preventDefault();
    
    if (!authToken) {
        alert('Please login to create memes');
        showPage('login');
        return;
    }
    
    const title = document.getElementById('memeTitle').value;
    const topText = document.getElementById('topText').value;
    const bottomText = document.getElementById('bottomText').value;
    
    try {
        const response = await fetch(`${API_BASE_URL}/memes`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${authToken}`,
            },
            body: JSON.stringify({
                title,
                top_text: topText || null,
                bottom_text: bottomText || null,
                template_name: selectedTemplate,
            }),
        });
        
        const result = await response.json();
        
        if (result.success) {
            alert('Meme created successfully! 🎉');
            document.getElementById('memeForm').reset();
            updatePreview();
            showPage('myMemes');
        } else {
            alert(result.error || 'Failed to create meme');
        }
    } catch (error) {
        console.error('Create meme error:', error);
        alert('Network error. Please try again.');
    }
}

async function loadMemes() {
    const container = document.getElementById('memesContainer');
    container.innerHTML = '<div class="loading">Loading memes...</div>';
    
    try {
        const response = await fetch(`${API_BASE_URL}/memes?limit=20`);
        const result = await response.json();
        
        if (result.success && result.data) {
            displayMemes(result.data, container);
        } else {
            container.innerHTML = '<div class="loading">No memes found</div>';
        }
    } catch (error) {
        console.error('Load memes error:', error);
        container.innerHTML = '<div class="loading">Error loading memes</div>';
    }
}

async function loadUserMemes() {
    if (!authToken) {
        showPage('login');
        return;
    }
    
    const container = document.getElementById('myMemesContainer');
    container.innerHTML = '<div class="loading">Loading your memes...</div>';
    
    try {
        const response = await fetch(`${API_BASE_URL}/memes/user/my-memes`, {
            headers: {
                'Authorization': `Bearer ${authToken}`,
            },
        });
        
        const result = await response.json();
        
        if (result.success && result.data) {
            displayMemes(result.data, container);
        } else {
            container.innerHTML = '<div class="loading">You haven\'t created any memes yet</div>';
        }
    } catch (error) {
        console.error('Load user memes error:', error);
        container.innerHTML = '<div class="loading">Error loading memes</div>';
    }
}

function displayMemes(memes, container) {
    if (memes.length === 0) {
        container.innerHTML = '<div class="loading">No memes found</div>';
        return;
    }
    
    container.innerHTML = memes.map(meme => `
        <div class="meme-card" onclick="viewMeme('${meme.id}')">
            <img src="${meme.image_url}" alt="${meme.title}" onerror="this.src='data:image/svg+xml,%3Csvg xmlns=\\'http://www.w3.org/2000/svg\\' width=\\'300\\' height=\\'250\\'%3E%3Crect fill=\\'%23334155\\' width=\\'300\\' height=\\'250\\'/%3E%3Ctext fill=\\'%23f1f5f9\\' font-family=\\'Arial\\' font-size=\\'20\\' x=\\'50%25\\' y=\\'50%25\\' text-anchor=\\'middle\\' dy=\\'.3em\\'%3E${meme.title}%3C/text%3E%3C/svg%3E'">
            <div class="meme-info">
                <h4>${meme.title}</h4>
                <div class="meme-meta">
                    <span>👁️ ${meme.views}</span>
                    <span>❤️ ${meme.likes}</span>
                </div>
            </div>
        </div>
    `).join('');
}

async function viewMeme(memeId) {
    try {
        const response = await fetch(`${API_BASE_URL}/memes/${memeId}`);
        const result = await response.json();
        
        if (result.success && result.data) {
            // In a real app, open modal or navigate to detail page
            alert(`Viewing: ${result.data.title}\nViews: ${result.data.views}\nLikes: ${result.data.likes}`);
        }
    } catch (error) {
        console.error('View meme error:', error);
    }
}

async function likeMeme(memeId) {
    try {
        const response = await fetch(`${API_BASE_URL}/memes/${memeId}/like`, {
            method: 'POST',
        });
        
        const result = await response.json();
        
        if (result.success) {
            loadMemes(); // Refresh the list
        }
    } catch (error) {
        console.error('Like meme error:', error);
    }
}
