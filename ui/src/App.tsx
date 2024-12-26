// import {useState} from "preact/hooks";
// import preactLogo from "./assets/preact.svg";
// import {invoke} from "@tauri-apps/api/core";
import "./App.css";
import {ChatPage} from "./pages/ChatPage.tsx";

function App() {
    // async function greet() {
    //     // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    //     setGreetMsg(await invoke("greet", {name}));
    // }

    return (
        <main className="h-screen w-screen flex">
            <ChatPage/>
        </main>
    );
}

export default App;
