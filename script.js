// REGISTER USER
document.getElementById('registerForm').addEventListener('submit', async (e) => {
    e.preventDefault();
    const form = e.target;
    const data = {
      username: form.username.value,
      password: form.password.value,
      role: form.role.value
    };
  
    const res = await fetch("http://localhost:3000/register", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(data)
    });
  
    alert(res.ok ? "✅ User registered!" : "❌ Failed to register user");
  });
  
  // ADD BOOK
  document.getElementById('bookForm').addEventListener('submit', async (e) => {
    e.preventDefault();
    const form = e.target;
    const data = {
      title: form.title.value,
      author: form.author.value,
      isbn: form.isbn.value,
      publication_year: parseInt(form.publication_year.value),
      genre: form.genre.value,
      copies: parseInt(form.copies.value)
    };
  
    const res = await fetch("http://localhost:3000/books", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(data)
    });
  
    alert(res.ok ? "📘 Book added!" : "❌ Failed to add book");
  });
  