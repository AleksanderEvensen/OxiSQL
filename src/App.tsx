import { invoke } from "@tauri-apps/api";
import { Link, useParams } from "react-router-dom";

type AppProps = {
	page: "home" | "database";
};

function App({ page }: AppProps) {
	const { databaseId } = useParams();
	async function getSchemas() {
		let data = await invoke("get_schemas");

		console.log(data);
	}

	return (
		<div>
			<p>Page: {page}</p>
			{databaseId && <div>Database: {databaseId}</div>}
			<button onClick={getSchemas}>Get Schemas</button>
			<br />
			<br />
			<br />
			<Link to={`/mysql-2`}>Goto Database</Link>
			<br />
			<Link to={`/`}>Goto Home</Link>
		</div>
	);
}

export default App;
