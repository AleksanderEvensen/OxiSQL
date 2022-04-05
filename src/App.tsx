import { useState } from "react";
import "./App.css";

import { invoke } from "@tauri-apps/api";

function App() {
	const [count, setCount] = useState(0);

	async function getSchemas() {
		let data = await invoke("get_schemas");

		console.log(data);
	}

	return (
		<div className="App">
			<button onClick={getSchemas}>Get Schemas</button>
		</div>
	);
}

export default App;
