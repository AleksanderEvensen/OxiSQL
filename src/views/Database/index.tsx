import { Link, useParams } from "react-router-dom";

export default () => {
	const { databaseId } = useParams();

	return (
		<div>
			<h1>Database page: {databaseId}</h1>
			<Link to="/">Go to Home</Link>
		</div>
	);
};
