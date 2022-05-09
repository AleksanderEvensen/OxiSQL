import { invoke } from "@tauri-apps/api";
import { useEffect, useState } from "react";
import { Link } from "react-router-dom";
import DatabaseProfile from "./components/DatabaseProfile";

export default () => {
	const [config, setConfig] = useState<any[]>([]);

	useEffect(() => {
		(async () => {
			const data: any = await invoke("get_config");
			setConfig(data);
		})();
	}, []);

	return (
		<div>
			<h1>Home page</h1>
			<Link to="/database/mysql-2">Go to database</Link>
			<br></br>
			<Link to="/createNew">Go to create new</Link>
			{config && config.map((profile) => <DatabaseProfile profile={profile} />)}
		</div>
	);
};
