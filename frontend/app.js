// ─── Taskflow — Frontend App ────────────────────────────────────

const API = window.location.origin;

// ─── State ───
let token = localStorage.getItem('tf_token') || null;
let currentProjectId = localStorage.getItem('tf_project') || null;
let projects = [];
let tasks = [];
let eventSource = null;

// ─── Boot ───
document.addEventListener('DOMContentLoaded', init);

function init() {
  if (token) {
    showApp();
  } else {
    showAuth();
  }
}

// ─── Auth Helpers ───
function authHeaders() {
  return {
    'Content-Type': 'application/json',
    'Authorization': `Bearer ${token}`,
  };
}

async function apiFetch(path, opts = {}) {
  const res = await fetch(`${API}${path}`, {
    ...opts,
    headers: { ...authHeaders(), ...(opts.headers || {}) },
  });

  if (res.status === 401) {
    logout();
    throw new Error('Unauthorized');
  }

  if (!res.ok) {
    const text = await res.text();
    throw new Error(text || `Request failed: ${res.status}`);
  }

  const contentType = res.headers.get('content-type');
  if (contentType && contentType.includes('application/json')) {
    return res.json();
  }
  return res.text();
}

// ─── Toast ───
function toast(message, type = 'success') {
  let container = document.getElementById('toast-container');
  if (!container) {
    container = document.createElement('div');
    container.id = 'toast-container';
    container.className = 'toast-container';
    document.body.appendChild(container);
  }

  const el = document.createElement('div');
  el.className = `toast toast-${type}`;
  el.textContent = message;
  container.appendChild(el);

  setTimeout(() => el.remove(), 3000);
}

// ─── Auth UI ───
function showAuth() {
  const app = document.getElementById('app');
  app.innerHTML = `
    <header class="header">
      <h1>Taskflow</h1>
      <p>Organize. Track. Ship.</p>
    </header>
    <div class="auth-section" id="auth-card">
      <h2 id="auth-title">Sign In</h2>
      <form class="auth-form" id="auth-form" onsubmit="handleAuth(event)">
        <input type="email" id="auth-email" placeholder="Email" required autocomplete="email" />
        <input type="password" id="auth-password" placeholder="Password" required autocomplete="current-password" />
        <button type="submit" class="btn btn-primary" id="auth-submit">Sign In</button>
      </form>
      <div class="auth-toggle">
        <span id="auth-toggle-text">Don't have an account?</span>
        <a id="auth-toggle-link" onclick="toggleAuthMode()">Sign Up</a>
      </div>
    </div>
  `;
  window._authMode = 'login';
}

function toggleAuthMode() {
  const mode = window._authMode === 'login' ? 'signup' : 'login';
  window._authMode = mode;

  document.getElementById('auth-title').textContent = mode === 'login' ? 'Sign In' : 'Create Account';
  document.getElementById('auth-submit').textContent = mode === 'login' ? 'Sign In' : 'Sign Up';
  document.getElementById('auth-toggle-text').textContent =
    mode === 'login' ? "Don't have an account?" : 'Already have an account?';
  document.getElementById('auth-toggle-link').textContent =
    mode === 'login' ? 'Sign Up' : 'Sign In';
}

async function handleAuth(e) {
  e.preventDefault();
  const email = document.getElementById('auth-email').value.trim();
  const password = document.getElementById('auth-password').value;
  const mode = window._authMode;
  const endpoint = mode === 'login' ? '/auth/login' : '/auth/signup';

  try {
    const data = await fetch(`${API}${endpoint}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ email, password }),
    });

    if (!data.ok) {
      const err = await data.text();
      throw new Error(err || 'Authentication failed');
    }

    const result = await data.json();
    token = result.access_token;
    localStorage.setItem('tf_token', token);
    toast(mode === 'login' ? 'Welcome back!' : 'Account created!');
    showApp();
  } catch (err) {
    toast(err.message, 'error');
  }
}

function logout() {
  token = null;
  currentProjectId = null;
  localStorage.removeItem('tf_token');
  localStorage.removeItem('tf_project');
  disconnectSSE();
  showAuth();
}

// ─── Main App UI ───
function showApp() {
  const app = document.getElementById('app');
  app.innerHTML = `
    <header class="header">
      <h1>Taskflow</h1>
      <p>Organize. Track. Ship.</p>
      <div class="sse-status disconnected" id="sse-indicator">
        <span class="sse-dot"></span>
        <span id="sse-label">Disconnected</span>
      </div>
    </header>

    <div class="user-bar" id="user-bar">
      <span>Signed in</span>
      <button onclick="logout()">Sign Out</button>
    </div>

    <!-- Projects Section -->
    <section class="section" id="project-section">
      <div class="section-title">Project</div>
      <div class="project-row">
        <select id="project-select" onchange="onProjectChange()">
          <option value="">— Select a project —</option>
        </select>
        <button class="btn btn-ghost btn-sm" onclick="toggleNewProject()" id="new-project-toggle">+ New</button>
      </div>
      <div class="new-project-row" id="new-project-row">
        <input type="text" id="new-project-name" placeholder="Project name" />
        <button class="btn btn-primary btn-sm" onclick="createProject()">Create</button>
        <button class="btn btn-ghost btn-sm" onclick="toggleNewProject()">Cancel</button>
      </div>
    </section>

    <!-- Tasks Section -->
    <section class="section" id="task-section" style="display:none;">
      <div class="section-title">Tasks</div>
      <div class="input-row">
        <input type="text" id="task-title" placeholder="What needs to be done?" onkeydown="if(event.key==='Enter') createTask()" />
        <button class="btn btn-primary" onclick="createTask()">Add</button>
      </div>
      <div class="task-list" id="task-list">
        <div class="loading-bar"><span class="spinner"></span></div>
      </div>
    </section>
  `;

  loadProjects();
  connectSSE();
}

// ─── Projects ───
async function loadProjects() {
  try {
    projects = await apiFetch('/projects');
    renderProjectDropdown();
  } catch (err) {
    toast('Failed to load projects', 'error');
  }
}

function renderProjectDropdown() {
  const select = document.getElementById('project-select');
  select.innerHTML = '<option value="">— Select a project —</option>';

  projects.forEach((p) => {
    const opt = document.createElement('option');
    opt.value = p.id;
    opt.textContent = p.name;
    if (p.id === currentProjectId) opt.selected = true;
    select.appendChild(opt);
  });

  // Auto-select if stored project exists
  if (currentProjectId && projects.some((p) => p.id === currentProjectId)) {
    onProjectChange();
  }
}

function onProjectChange() {
  const select = document.getElementById('project-select');
  currentProjectId = select.value || null;

  if (currentProjectId) {
    localStorage.setItem('tf_project', currentProjectId);
    document.getElementById('task-section').style.display = '';
    loadTasks();
  } else {
    localStorage.removeItem('tf_project');
    document.getElementById('task-section').style.display = 'none';
  }
}

function toggleNewProject() {
  const row = document.getElementById('new-project-row');
  row.classList.toggle('active');
  if (row.classList.contains('active')) {
    document.getElementById('new-project-name').focus();
  }
}

async function createProject() {
  const nameInput = document.getElementById('new-project-name');
  const name = nameInput.value.trim();
  if (!name) return;

  try {
    const p = await apiFetch('/projects', {
      method: 'POST',
      body: JSON.stringify({ name }),
    });

    toast(`Project "${p.name}" created`);
    nameInput.value = '';
    toggleNewProject();

    currentProjectId = p.id;
    localStorage.setItem('tf_project', p.id);
    await loadProjects();
    document.getElementById('task-section').style.display = '';
    loadTasks();
  } catch (err) {
    toast(err.message, 'error');
  }
}

// ─── Tasks ───
async function loadTasks() {
  if (!currentProjectId) return;

  try {
    tasks = await apiFetch(`/tasks?project_id=${currentProjectId}`);
    renderTasks();
  } catch (err) {
    toast('Failed to load tasks', 'error');
  }
}

function renderTasks() {
  const list = document.getElementById('task-list');

  if (!tasks.length) {
    list.innerHTML = `
      <div class="empty-state">
        <div class="icon">📋</div>
        <div>No tasks yet. Add one above!</div>
      </div>
    `;
    return;
  }

  // Sort: todo → in_progress → done
  const order = { todo: 0, in_progress: 1, done: 2 };
  const sorted = [...tasks].sort((a, b) => (order[a.status] ?? 9) - (order[b.status] ?? 9));

  list.innerHTML = sorted.map((t) => {
    const nextStatus = getNextStatus(t.status);
    const statusIcon = {
      todo: '○',
      in_progress: '◐',
      done: '●',
    }[t.status] || '○';

    const statusLabel = {
      todo: 'To Do',
      in_progress: 'In Progress',
      done: 'Done',
    }[t.status] || t.status;

    const nextLabel = {
      todo: 'Start',
      in_progress: 'Complete',
      done: 'Reopen',
    }[t.status] || 'Next';

    return `
      <div class="task-item ${t.status}" data-id="${t.id}">
        <div class="task-info">
          <div class="task-title">${escapeHtml(t.title)}</div>
          <span class="badge badge-${t.status}">${statusIcon} ${statusLabel}</span>
        </div>
        <div class="task-actions">
          <button class="btn btn-ghost btn-sm" onclick="updateTaskStatus('${t.id}', '${nextStatus}')" title="Set to ${nextStatus}">
            ${nextLabel}
          </button>
          <button class="btn btn-danger btn-sm" onclick="deleteTask('${t.id}')" title="Delete task">
            ✕
          </button>
        </div>
      </div>
    `;
  }).join('');
}

function getNextStatus(current) {
  switch (current) {
    case 'todo': return 'in_progress';
    case 'in_progress': return 'done';
    case 'done': return 'todo';
    default: return 'todo';
  }
}

async function createTask() {
  const input = document.getElementById('task-title');
  const title = input.value.trim();
  if (!title || !currentProjectId) return;

  try {
    await apiFetch('/tasks', {
      method: 'POST',
      body: JSON.stringify({ project_id: currentProjectId, title }),
    });
    input.value = '';
    toast('Task added');
    // SSE will trigger reload, but also load here for snappiness
    loadTasks();
  } catch (err) {
    toast(err.message, 'error');
  }
}

async function updateTaskStatus(taskId, status) {
  try {
    await apiFetch(`/tasks/${taskId}`, {
      method: 'PATCH',
      body: JSON.stringify({ status }),
    });
    loadTasks();
  } catch (err) {
    toast(err.message, 'error');
  }
}

async function deleteTask(taskId) {
  try {
    await apiFetch(`/tasks/${taskId}`, { method: 'DELETE' });
    toast('Task deleted');
    loadTasks();
  } catch (err) {
    toast(err.message, 'error');
  }
}

// ─── SSE ───
function connectSSE() {
  disconnectSSE();

  eventSource = new EventSource(`${API}/events`);

  eventSource.onopen = () => {
    const el = document.getElementById('sse-indicator');
    if (el) {
      el.className = 'sse-status connected';
      document.getElementById('sse-label').textContent = 'Live';
    }
  };

  eventSource.onmessage = () => {
    // Reload tasks on any event
    if (currentProjectId) loadTasks();
  };

  eventSource.onerror = () => {
    const el = document.getElementById('sse-indicator');
    if (el) {
      el.className = 'sse-status disconnected';
      document.getElementById('sse-label').textContent = 'Reconnecting…';
    }
  };
}

function disconnectSSE() {
  if (eventSource) {
    eventSource.close();
    eventSource = null;
  }
}

// ─── Utilities ───
function escapeHtml(str) {
  const div = document.createElement('div');
  div.textContent = str;
  return div.innerHTML;
}
