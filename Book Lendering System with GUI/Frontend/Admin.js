const socket = new WebSocket('ws://localhost:9001'); // WebSocket connection
const bookForm = document.getElementById('bookForm');
const bookList = document.getElementById('bookList');
const searchBox = document.getElementById('searchBox');

// Handle book submission
bookForm.addEventListener('submit', function (e) {
  e.preventDefault();

  const title = document.getElementById('title').value;
  const author = document.getElementById('author').value;
  const isbn = document.getElementById('isbn').value;
  const year = document.getElementById('year').value;
  const genre = document.getElementById('genre').value;
  let copies = parseInt(document.getElementById('copies').value);
  const status = copies > 0 ? 'Available' : 'Checked Out';
  const statusClass = copies > 0 ? 'available' : 'checkedout';

  const book = { title, author, isbn, year, genre, copies };

  // Send to server
  fetch('/addBook', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(book)
  }).then(() => {
    socket.send(JSON.stringify(book));
  });

  // Display locally
  const bookCard = document.createElement('div');
  bookCard.classList.add('book-card');

  bookCard.innerHTML = `
    <h4>${title}</h4>
    <p><strong>Author:</strong> ${author}</p>
    <p><strong>ISBN:</strong> ${isbn}</p>
    <p><strong>Year:</strong> ${year}</p>
    <p><strong>Genre:</strong> ${genre}</p>
    <p><strong>Copies:</strong> <span class="copy-count">${copies}</span></p>
    <div class="status-box ${statusClass}">
      <span class="status-text">${status}</span>
      ${copies <= 0 ? '<button class="notify-btn">Notify Me</button>' : ''}
      <button class="edit-btn">Edit</button>
    </div>
    <button class="delete-btn">&times;</button>
  `;

  // Delete functionality
  bookCard.querySelector('.delete-btn').addEventListener('click', () => {
    bookCard.remove();
  });

  // Edit functionality
  const editBtn = bookCard.querySelector('.edit-btn');
  editBtn.addEventListener('click', () => {
    const newCopies = prompt("Enter new number of copies:");
    if (newCopies !== null && !isNaN(newCopies)) {
      const count = parseInt(newCopies);
      const copyCount = bookCard.querySelector('.copy-count');
      const statusBox = bookCard.querySelector('.status-box');
      const statusText = bookCard.querySelector('.status-text');

      copyCount.textContent = count;

      if (count > 0) {
        statusBox.classList.remove('checkedout');
        statusBox.classList.add('available');
        statusText.textContent = 'Available';
        const notifyBtn = statusBox.querySelector('.notify-btn');
        if (notifyBtn) notifyBtn.remove();
      } else {
        statusBox.classList.remove('available');
        statusBox.classList.add('checkedout');
        statusText.textContent = 'Checked Out';
        if (!statusBox.querySelector('.notify-btn')) {
          const notifyBtn = document.createElement('button');
          notifyBtn.className = 'notify-btn';
          notifyBtn.textContent = 'Notify Me';
          statusBox.insertBefore(notifyBtn, editBtn);
        }
      }
    }
  });

  bookList.appendChild(bookCard);
  bookForm.reset();
});

// Search functionality
searchBox.addEventListener('input', () => {
  const query = searchBox.value.toLowerCase();
  const cards = Array.from(bookList.getElementsByClassName('book-card'));

  const matched = [];
  const others = [];

  cards.forEach(card => {
    const title = card.querySelector('h4').textContent.toLowerCase();
    const authorText = card.querySelector('p').textContent.toLowerCase();

    if (title.includes(query) || authorText.includes(query)) {
      matched.push(card);
    } else {
      others.push(card);
    }
  });

  bookList.innerHTML = '';
  matched.forEach(card => bookList.appendChild(card));
  others.forEach(card => bookList.appendChild(card));
});

// Admin Dashboard
window.addEventListener('DOMContentLoaded', loadAdminDashboard);

function loadAdminDashboard() {
  loadBooks();
  loadUsers();
  loadBorrowedBooks();
  loadOverdueBooks();
}

function loadBooks() {
  fetch('/getAllBooks')
    .then(res => res.json())
    .then(books => {
      const container = document.getElementById('dashboardBooks');
      container.innerHTML = books.map(b => `
        <p><strong>${b.title}</strong> by ${b.author} - ${b.copies} copies</p>
      `).join('');
    });
}

function loadUsers() {
  fetch('/getAllUsers')
    .then(res => res.json())
    .then(users => {
      const container = document.getElementById('dashboardUsers');
      container.innerHTML = users.map(u => `
        <p>${u.name} (${u.email})</p>
      `).join('');
    });
}

function loadBorrowedBooks() {
  fetch('/getBorrowedBooks')
    .then(res => res.json())
    .then(borrows => {
      const container = document.getElementById('dashboardBorrowed');
      container.innerHTML = borrows.map(b => `
        <p>${b.title} - Borrowed by <strong>${b.user}</strong></p>
      `).join('');
    });
}

function loadOverdueBooks() {
  fetch('/getOverdueBooks')
    .then(res => res.json())
    .then(overdue => {
      const container = document.getElementById('dashboardOverdue');
      if (overdue.length === 0) {
        container.innerHTML = '<p>No overdue books</p>';
        return;
      }

      container.innerHTML = overdue.map(o => `
        <p>
          <strong>${o.title}</strong> (Due: ${o.dueDate})<br/>
          Borrowed by: ${o.user}<br/>
          <button class="notify-btn" onclick="notifyUser('${o.user}', '${o.title}')">Notify ${o.user}</button>
        </p>
      `).join('');
    });
}

function notifyUser(user, book) {
  alert(`Notification sent to ${user} for "${book}"`);
  // Optionally: send notification to backend
}

function toggleDashboardMenu() {
  const menu = document.getElementById('dashboardMenu');
  menu.style.display = (menu.style.display === 'block') ? 'none' : 'block';
}

function showDashboardSection(sectionId) {
  // Hide Add New Book and Book Records
  document.getElementById('addBookForm').style.display = 'none';
  document.getElementById('bookRecordsSection').style.display = 'none';
  document.getElementById('main').style.display = 'none';
  document.getElementById('pre').style.display = 'none';
  // Hide all dashboard sections
  document.querySelectorAll('.dashboard-section').forEach(section => {
    section.style.display = 'none';
  });

  // Show only selected section
  const selected = document.getElementById(sectionId);
  if (selected) {
    selected.style.display = 'block';
  }

  // Hide menu after click
  toggleDashboardMenu();
}

