'use client';
import Link from "next/link";
import "../style.css";
import { useState } from "react";


export default function ToDoList() {
  const [data, setData] = useState({});
  
  const postTask = async (e: { preventDefault: () => void; }) => {
    e.preventDefault();
    const response = await fetch('http://127.0.0.1:3000/tasks', {
      method: 'POST',
      body: JSON.stringify(data),
      headers: {
        'Content-Type': 'application/json'
      }
    });
  }
  return (
    <main className="background">
      <div className="navigation_bar">
        <Link href="/" style={{ margin: "10vh" }} className="hover-highlight">Home</Link>
        <Link href="/ToDoList" style={{ margin: "10vh" }} className="hover-highlight">To Do List</Link>
      </div>
      <div>
        <form onSubmit={postTask}>
          <label>Title </label>
          <br></br>
          <input type="text" onChange={e => setData({...data, title: e.target.value})} />
          <br></br>
          <br></br>
          <label>Description </label>
          <br></br>
          <input type="text" onChange={e => setData({...data, description: e.target.value})} />
          <br></br><br></br>
          <input type="submit" value="Submit" className="hover-highlight"></input>
        </form>
      </div>
    </main>
  )
}
