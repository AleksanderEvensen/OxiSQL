interface DatabaseProfileProps {
	profile: any;
}

const DatabaseProfile: React.FunctionComponent<DatabaseProfileProps> = ({ profile }) => {
	return (
		<div>
			<h3>{profile.name}</h3>
			<p>
				{profile.ip}:{profile.port}
			</p>
			{profile.type}
		</div>
	);
};

export default DatabaseProfile;
