'use client';
import Link from "next/link";
import "../style.css";
import { useEffect, useState } from "react";

interface Task {
  title: String,
  description: String,
  id: Number,
}

function toggleDescription(container: { querySelector: (arg0: string) => any; }) {
  const description = container.querySelector('.description');
  description.style.display = description.style.display === 'block' ? 'none' : 'block';
}
export default function ToDoList() {
  const [tasks, setTasks] = useState<Task[]>([]);
  const [new_task, setNewTask] = useState<Task>({ title: "title", description: "description", id: 0 });
  const postTask = async (e: { preventDefault: () => void; }) => {
    e.preventDefault();
    const response = await fetch('http://127.0.0.1:3000/tasks', {
      method: 'POST',
      body: JSON.stringify(new_task),
      headers: {
        'Content-Type': 'application/json'
      }
    })
      .then((response) => response.json())
      .then((responseJson) => {
        setTasks(responseJson);
      })
      .catch((error) => {
        console.error(error);
      });
  }
  const readTask = async () => {
    fetch('http://127.0.0.1:3000/tasks', {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json'
      }
    })
      .then((response) => response.json())
      .then((responseJson) => {
        setTasks(responseJson);
      })
      .catch((error) => {
        console.error(error);
      });
  }
  const removeTask = async (id: Number) => {
    const response = await fetch('http://127.0.0.1:3000/tasks/' + id, {
      method: 'DELETE',
      headers: {
        'Content-Type': 'application/json'
      }
    })
      .then((response) => response.json())
      .then((responseJson) => {
        setTasks(responseJson);
      })
      .catch((error) => {
        console.error(error);
      });
  }
  function onStart() {
    readTask();
  }
  function TaskComponent(title: String, description: String, id: Number, key: any) {
    return (
      <ul className="notebook_background" style={{ width: "20vw", userSelect: "none" }} key={key as any}>
        <div className="container; dark_text" onClick={(e) => toggleDescription(e.currentTarget)}>
          <div className="title">{title}</div>
          <input className="dark_text" value="x" style={{ height: "10px", width: "10px" }} type="button" onClick={e => removeTask(id)} />
          <div className="description">{description}</div>
        </div>
      </ul>
    )
  }
  useEffect(() => {
    onStart();
  }, []);
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
          <input className="dark_text" type="text" onChange={e => setNewTask({ title: e.target.value, description: new_task.description, id: new_task.id })} />
          <br></br>
          <br></br>
          <label>Description </label>
          <br></br>
          <input className="dark_text" type="text" onChange={e => setNewTask({ description: e.target.value, title: new_task.title, id: new_task.id })} />
          <br></br><br></br>
          <input type="submit" value="Submit" className="hover-highlight"></input>
          <br></br><br></br><br></br><br></br>
        </form>
      </div>
      {tasks.map((task, index) => (
        TaskComponent(task.title, task.description, task.id, index)
      ))}
    </main>
  )
}
