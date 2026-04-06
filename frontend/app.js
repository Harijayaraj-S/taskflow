// ═══════════════════════════════════════════════════════════════
// Taskflow — Frontend Application
// ═══════════════════════════════════════════════════════════════

const API = window.location.origin;

// ── State ──
let token = localStorage.getItem('tf_token') || null;
let currentProjectId = localStorage.getItem('tf_project') || null;
let projects = [];
let tasks = [];
let eventSource = null;

// ── DOM Cache ──
const $ = (id) => document.getElementById(id);

// ── Boot ──
document.addEventListener('DOMContentLoaded', init);

function init() {
  bindEvents();
  if (token) {
    showApp();
  } else {
    showAuth();
  }
}

// ═══════════════════════════════════════════════════════════════
// EVENT BINDING
// ═══════════════════════════════════════════════════════════════

function bindEvents() {
  // Auth
  $('auth-form').addEventListener('submit', handleAuth);
  $('auth-toggle-link').addEventListener('click', toggleAuthMode);

  // Sidebar
  $('new-project-btn').addEventListener('click', openProjectModal);
  $('sidebar-toggle').addEventListener('click', toggleSidebar);

  // Project Modal
  $('project-form').addEventListener('submit', handleCreateProject);
  $('modal-close-btn').addEventListener('click', closeProjectModal);
  $('modal-cancel-btn').addEventListener('click', closeProjectModal);
  $('project-modal').addEventListener('click', (e) => {
    if (e.target === $('project-modal')) closeProjectModal();
  });

  // Tasks
  $('add-task-btn').addEventListener('click', createTask);
  $('task-input').addEventListener('keydown', (e) => {
    if (e.key === 'Enter') createTask();
  });

  // Logout
  $('logout-btn').addEventListener('click', logout);

  // Keyboard — ESC to close modal
  document.addEventListener('keydown', (e) => {
    if (e.key === 'Escape') closeProjectModal();
  });
}

// ═══════════════════════════════════════════════════════════════
// AUTH
// ═══════════════════════════════════════════════════════════════

let authMode = 'login';

function showAuth() {
  $('auth-screen').style.display = '';
  $('app-shell').style.display = 'none';
  disconnectSSE();
}

function showApp() {
  $('auth-screen').style.display = 'none';
  $('app-shell').style.display = '';
  loadProjects();
  connectSSE();
}

function toggleAuthMode() {
  authMode = authMode === 'login' ? 'signup' : 'login';

  $('auth-title').textContent =
    authMode === 'login' ? 'Sign in to your account' : 'Create a new account';
  $('auth-submit-text').textContent =
    authMode === 'login' ? 'Sign In' : 'Sign Up';
  $('auth-toggle-text').textContent =
    authMode === 'login' ? "Don't have an account?" : 'Already have an account?';
  $('auth-toggle-link').textContent =
    authMode === 'login' ? 'Sign Up' : 'Sign In';
}

async function handleAuth(e) {
  e.preventDefault();
  const email = $('auth-email').value.trim();
  const password = $('auth-password').value;
  const endpoint = authMode === 'login' ? '/auth/login' : '/auth/signup';

  $('auth-submit-text').style.display = 'none';
  $('auth-spinner').style.display = '';

  try {
    const res = await fetch(`${API}${endpoint}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ email, password }),
    });

    if (!res.ok) {
      const err = await res.text();
      throw new Error(err || 'Authentication failed');
    }

    const data = await res.json();
    token = data.access_token;
    localStorage.setItem('tf_token', token);
    toast(authMode === 'login' ? 'Welcome back!' : 'Account created!');
    showApp();
  } catch (err) {
    toast(err.message, 'error');
  } finally {
    $('auth-submit-text').style.display = '';
    $('auth-spinner').style.display = 'none';
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

// ═══════════════════════════════════════════════════════════════
// API HELPERS
// ═══════════════════════════════════════════════════════════════

function authHeaders() {
  return {
    'Content-Type': 'application/json',
    Authorization: `Bearer ${token}`,
  };
}

async function apiFetch(path, opts = {}) {
  const res = await fetch(`${API}${path}`, {
    ...opts,
    headers: { ...authHeaders(), ...(opts.headers || {}) },
  });

  if (res.status === 401) {
    logout();
    throw new Error('Session expired');
  }

  if (!res.ok) {
    const text = await res.text();
    throw new Error(text || `Request failed: ${res.status}`);
  }

  const ct = res.headers.get('content-type');
  return ct && ct.includes('application/json') ? res.json() : res.text();
}

// ═══════════════════════════════════════════════════════════════
// PROJECTS
// ═══════════════════════════════════════════════════════════════

async function loadProjects() {
  try {
    projects = await apiFetch('/projects');
    renderProjects();
  } catch (err) {
    toast('Failed to load projects', 'error');
  }
}

function renderProjects() {
  const list = $('project-list');

  if (!projects.length) {
    list.innerHTML = `
      <li class="sidebar-empty">
        Create your first project<br />to get started.
      </li>
    `;
    showEmptyMain();
    return;
  }

  list.innerHTML = projects
    .map((p) => {
      const initial = p.name.charAt(0).toUpperCase();
      const isActive = p.id === currentProjectId;
      return `
        <li class="project-item${isActive ? ' active' : ''}"
            data-id="${p.id}"
            role="button"
            tabindex="0">
          <span class="project-icon">${escapeHtml(initial)}</span>
          <span class="project-name">${escapeHtml(p.name)}</span>
        </li>
      `;
    })
    .join('');

  // Bind click events
  list.querySelectorAll('.project-item').forEach((el) => {
    el.addEventListener('click', () => selectProject(el.dataset.id));
    el.addEventListener('keydown', (e) => {
      if (e.key === 'Enter') selectProject(el.dataset.id);
    });
  });

  // Restore selection
  if (currentProjectId && projects.some((p) => p.id === currentProjectId)) {
    selectProject(currentProjectId, false);
  } else {
    showEmptyMain();
  }
}

function selectProject(id, closeMobile = true) {
  currentProjectId = id;
  localStorage.setItem('tf_project', id);

  // Update active state in sidebar
  document.querySelectorAll('.project-item').forEach((el) => {
    el.classList.toggle('active', el.dataset.id === id);
  });

  // Show project view
  $('empty-main').style.display = 'none';
  $('project-view').style.display = '';

  // Set project title
  const project = projects.find((p) => p.id === id);
  if (project) {
    $('project-title').textContent = project.name;
  }

  loadTasks();

  // Close mobile sidebar
  if (closeMobile && window.innerWidth <= 768) {
    closeSidebar();
  }
}

function showEmptyMain() {
  $('empty-main').style.display = '';
  $('project-view').style.display = 'none';
}

// ── Project Modal ──

function openProjectModal() {
  $('project-modal').style.display = '';
  $('project-name-input').value = '';
  setTimeout(() => $('project-name-input').focus(), 100);
}

function closeProjectModal() {
  $('project-modal').style.display = 'none';
}

async function handleCreateProject(e) {
  e.preventDefault();
  const name = $('project-name-input').value.trim();
  if (!name) return;

  try {
    const p = await apiFetch('/projects', {
      method: 'POST',
      body: JSON.stringify({ name }),
    });
    toast(`Project "${p.name}" created`);
    closeProjectModal();
    currentProjectId = p.id;
    localStorage.setItem('tf_project', p.id);
    await loadProjects();
  } catch (err) {
    toast(err.message, 'error');
  }
}

// ═══════════════════════════════════════════════════════════════
// TASKS
// ═══════════════════════════════════════════════════════════════

async function loadTasks() {
  if (!currentProjectId) return;

  const list = $('task-list');
  list.innerHTML = `
    <div class="loading-state">
      <span class="spinner"></span>
      <span>Loading tasks…</span>
    </div>
  `;

  try {
    tasks = await apiFetch(`/tasks?project_id=${currentProjectId}`);
    renderTasks();
  } catch (err) {
    toast('Failed to load tasks', 'error');
    list.innerHTML = '';
  }
}

function renderTasks() {
  const list = $('task-list');

  if (!tasks.length) {
    list.innerHTML = `
      <div class="task-empty-state">
        <div class="task-empty-icon">📋</div>
        <h3>No tasks yet</h3>
        <p>Add a task above to get started.</p>
      </div>
    `;
    return;
  }

  // Group by status
  const groups = [
    { key: 'in_progress', label: 'In Progress', items: [] },
    { key: 'todo',        label: 'To Do',       items: [] },
    { key: 'done',        label: 'Done',        items: [] },
  ];

  const groupMap = Object.fromEntries(groups.map((g) => [g.key, g]));

  tasks.forEach((t) => {
    const group = groupMap[t.status];
    if (group) group.items.push(t);
    else groupMap.todo.items.push(t);
  });

  const activeGroups = groups.filter((g) => g.items.length > 0);

  list.innerHTML = activeGroups
    .map(
      (group) => `
      <div class="task-group">
        <div class="task-group-label">
          ${group.label}
          <span class="task-group-count">${group.items.length}</span>
        </div>
        ${group.items.map((t) => renderTaskItem(t)).join('')}
      </div>
    `
    )
    .join('');

  // Bind task events
  list.querySelectorAll('[data-action="toggle"]').forEach((btn) => {
    btn.addEventListener('click', () => {
      const id = btn.closest('.task-item').dataset.id;
      const status = btn.dataset.nextStatus;
      updateTaskStatus(id, status);
    });
  });

  list.querySelectorAll('[data-action="delete"]').forEach((btn) => {
    btn.addEventListener('click', () => {
      const id = btn.closest('.task-item').dataset.id;
      deleteTask(id);
    });
  });
}

function renderTaskItem(task) {
  const nextStatus = getNextStatus(task.status);
  const statusLabel = {
    todo: 'To Do',
    in_progress: 'In Progress',
    done: 'Done',
  }[task.status] || task.status;

  const nextIcon = {
    todo: '▶',       // Start
    in_progress: '✓', // Complete
    done: '↺',        // Reopen
  }[task.status] || '▶';

  const nextTooltip = {
    todo: 'Start',
    in_progress: 'Mark Done',
    done: 'Reopen',
  }[task.status] || 'Next';

  return `
    <div class="task-item status-${task.status}" data-id="${task.id}">
      <div class="task-checkbox ${task.status}"
           data-action="toggle"
           data-next-status="${nextStatus}"
           title="${nextTooltip}"
           role="button"
           tabindex="0"></div>
      <div class="task-body">
        <div class="task-title">${escapeHtml(task.title)}</div>
        <span class="task-status-label ${task.status}">${statusLabel}</span>
      </div>
      <div class="task-actions">
        <button class="task-action-btn" data-action="toggle" data-next-status="${nextStatus}" title="${nextTooltip}">
          ${nextIcon}
        </button>
        <button class="task-action-btn delete" data-action="delete" title="Delete">
          ✕
        </button>
      </div>
    </div>
  `;
}

function getNextStatus(current) {
  const cycle = { todo: 'in_progress', in_progress: 'done', done: 'todo' };
  return cycle[current] || 'todo';
}

async function createTask() {
  const input = $('task-input');
  const title = input.value.trim();
  if (!title || !currentProjectId) return;

  try {
    await apiFetch('/tasks', {
      method: 'POST',
      body: JSON.stringify({ project_id: currentProjectId, title }),
    });
    input.value = '';
    toast('Task added');
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

// ═══════════════════════════════════════════════════════════════
// SSE
// ═══════════════════════════════════════════════════════════════

function connectSSE() {
  disconnectSSE();

  eventSource = new EventSource(`${API}/events`);

  eventSource.onopen = () => {
    const el = $('sse-indicator');
    if (el) {
      el.className = 'sse-badge connected';
      $('sse-label').textContent = 'Live';
    }
  };

  eventSource.onmessage = () => {
    if (currentProjectId) loadTasks();
  };

  eventSource.onerror = () => {
    const el = $('sse-indicator');
    if (el) {
      el.className = 'sse-badge disconnected';
      $('sse-label').textContent = 'Disconnected';
    }
  };
}

function disconnectSSE() {
  if (eventSource) {
    eventSource.close();
    eventSource = null;
  }
}

// ═══════════════════════════════════════════════════════════════
// SIDEBAR — MOBILE
// ═══════════════════════════════════════════════════════════════

function toggleSidebar() {
  const sidebar = $('sidebar');
  sidebar.classList.toggle('open');

  if (sidebar.classList.contains('open')) {
    const backdrop = document.createElement('div');
    backdrop.className = 'sidebar-backdrop';
    backdrop.id = 'sidebar-backdrop';
    backdrop.addEventListener('click', closeSidebar);
    document.body.appendChild(backdrop);
  } else {
    removeBackdrop();
  }
}

function closeSidebar() {
  $('sidebar').classList.remove('open');
  removeBackdrop();
}

function removeBackdrop() {
  const bd = $('sidebar-backdrop');
  if (bd) bd.remove();
}

// ═══════════════════════════════════════════════════════════════
// TOAST
// ═══════════════════════════════════════════════════════════════

function toast(message, type = 'success') {
  const container = $('toast-container');
  const el = document.createElement('div');
  el.className = `toast toast-${type}`;
  el.textContent = message;
  container.appendChild(el);
  setTimeout(() => el.remove(), 3200);
}

// ═══════════════════════════════════════════════════════════════
// UTILITIES
// ═══════════════════════════════════════════════════════════════

function escapeHtml(str) {
  const div = document.createElement('div');
  div.textContent = str;
  return div.innerHTML;
}
