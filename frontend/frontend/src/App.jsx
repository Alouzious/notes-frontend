import { useEffect, useState } from "react";
import axios from "axios";
import "./App.css";

const API_URL = import.meta.env.VITE_API_URL;

function App() {
  const [notes, setNotes] = useState([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    axios.get(`${API_URL}/api/notes`)
      .then(res => {
        setNotes(res.data);
        setLoading(false);
      })
      .catch(err => {
        console.error("Error fetching notes:", err);
        setLoading(false);
      });
  }, []);

  return (
    <div className="user-notes-container">
      {/* Header */}
      <header className="header">
        <div className="header-content">
          <h1 className="header-title">📝 Notes</h1>
          <p className="header-subtitle">
            Explore our collection of thoughts and ideas
          </p>
        </div>
      </header>

      {/* Main Content */}
      <main className="main-content">
        {loading ? (
          <div className="loading-container">
            <div className="spinner"></div>
          </div>
        ) : notes.length === 0 ? (
          <div className="empty-state">
            <div className="empty-icon">📭</div>
            <h2 className="empty-title">No notes yet</h2>
            <p className="empty-text">Check back later for new content</p>
          </div>
        ) : (
          <div className="notes-grid">
            {notes.map((note, index) => (
              <div
                key={note.id}
                className="note-card"
                style={{ animationDelay: `${index * 0.1}s` }}
              >
                <div className="note-card-accent"></div>
                <div className="note-card-content">
                  <h3 className="note-title">{note.title}</h3>
                  <p className="note-content">{note.content}</p>
                </div>
                <div className="note-card-footer">
                  <button className="read-more-btn">
                    Read more <span className="arrow">→</span>
                  </button>
                </div>
              </div>
            ))}
          </div>
        )}
      </main>

      {/* Footer */}
      <footer className="footer">
        <div className="footer-content">
          <p>© 2025 Notes App. All rights reserved.</p>
        </div>
      </footer>
    </div>
  );
}

export default App;
