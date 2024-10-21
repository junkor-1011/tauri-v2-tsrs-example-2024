import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import type { GreetArgs, GreetChannelName, GreetResponse } from "@ipc-if/greet";
import type {
  RandomExampleArgs,
  RandomExampleChannelName,
  RandomExampleError,
  RandomExampleResponse,
} from "@ipc-if/random-example";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  const [randomExampleMsg, setRandomExampleMsg] = useState<string>("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

    const channel: GreetChannelName = "greet";
    const args: GreetArgs = {
      name,
    };
    const response: GreetResponse = await invoke(channel, { args });

    setGreetMsg(response.message);
  }

  async function randomExample() {
    const channel: RandomExampleChannelName = "random_example";
    const args: RandomExampleArgs = {
      requestId: self.crypto.randomUUID(),
    };
    try {
      const { message, responseId, timestamp }: RandomExampleResponse =
        await invoke(channel, { args });

      const value =
        `(success)message: ${message}, responseId: ${responseId}, timestamp: ${timestamp}`;
      setRandomExampleMsg(value);
    } catch (err: unknown) {
      const { errorMessage } = err as RandomExampleError;
      const value = `(failed)error message: ${errorMessage}`;

      setRandomExampleMsg(value);
    }
  }

  return (
    <main className="container">
      <h1>Welcome to Tauri + React</h1>

      <div className="row">
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
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p>
      <div>
        <h2>random example</h2>
        <button
          type="button"
          onClick={randomExample}
        >
          Click
        </button>
        <p>{randomExampleMsg}</p>
      </div>
    </main>
  );
}

export default App;
