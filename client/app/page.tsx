'use client';
import React, { useState } from "react";

function App() {
  const [name, setName] = useState("");
  const [text, setText] = useState("");
  const [response, setResponse] = useState("");

  async function sendMessage(e: React.FormEvent) {
    e.preventDefault();

    const res = await fetch("http://localhost:8080/api/messages", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ name, text }),
    });

    const data = await res.json();
    setResponse(data.message);
  }

  return (
    <div className="min-h-screen bg-black text-green-400 font-mono flex flex-col items-center justify-center p-4">
      <h1 className="text-2xl mb-6 pixel text-neon">💬 Retro Chat</h1>
      <form onSubmit={sendMessage} className="w-full max-w-md bg-gray-900 p-6 rounded-lg shadow-lg border border-green-500 space-y-4">
        <input
          type="text"
          placeholder="Your name"
          value={name}
          onChange={(e) => setName(e.target.value)}
          className="w-full p-2 bg-black border border-green-500 text-green-300 placeholder-green-600 focus:outline-none"
        />
        <textarea
          placeholder="Your message..."
          value={text}
          onChange={(e) => setText(e.target.value)}
          className="w-full p-2 bg-black border border-green-500 text-green-300 placeholder-green-600 focus:outline-none"
        ></textarea>
        <button
          type="submit"
          className="w-full bg-green-700 hover:bg-green-600 text-black font-bold py-2 px-4 rounded transition"
        >
          🚀 Send Message
        </button>
      </form>
      {response && <p className="mt-4 text-green-400">{response}</p>}
    </div>
  );
}

export default App;
