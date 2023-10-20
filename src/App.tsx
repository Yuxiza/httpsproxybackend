import { useState } from "react";
// import reactLogo from "./assets/react.svg";
// import { invoke } from "@tauri-apps/api/tauri";
import { fetch } from "./proxy";
import "./App.css";

function App() {
  // const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [returnMsg, setReturnMsg] = useState("");

  // async function greet() {
  //   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  //   setGreetMsg(await invoke("greet", { name }));
  // }

  async function fetchData() {
    try{
      let res = await fetch(name,{method:'GET'})
      // console.log(res);
      setReturnMsg(JSON.stringify(res.data));
    } catch(e) {
      if(e === "fail:Here is hihuu.club from Rust :)")
      {
        setReturnMsg(e);
      } else {
        setReturnMsg("Error:need to fetch a json api");
      }
    }
  }

  return (
    <div className="container">
      <h1>Try fetch in rust</h1>

      {/* <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and React logos to learn more.</p> */}

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          fetchData();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter an url..."
        />
        <button type="submit">Fetch</button>
      </form>
      <img id="canvas_img" src="" alt=""></img>
      <p>{returnMsg}</p>
    </div>
  );
}

export default App;
