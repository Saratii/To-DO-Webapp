import "./style.css";
import Link from 'next/link';


export default function Index() {
  return (
    <main className="background">
        <div className="navigation_bar">
          <Link href="/" style={{margin: "10vh"}} className="hover-highlight">Home</Link>
          <Link href="/ToDoList" style={{margin: "10vh"}} className="hover-highlight">To Do List</Link>
        </div>
        <div className="light_text center">
          <p>To Do!</p>
          <Link href="/ToDoList">
            <input type="image" name="Note Pad Button" src="assets/note_pad.png" style={{width: "40wh", height: "40vh"}} className="hover-highlight"></input>
          </Link>
          <input type="image" name="Bucket Button" src="assets/bucket.png" style={{width: "40wh", height: "40vh"}} className="hover-highlight"></input>
        </div>
    </main>
  )
}
