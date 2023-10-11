'use client';

import React from 'react';
import * as dtom from '@dyn/dtom';

async function onClick() {
	await dtom.initWasm();
	const editor = dtom.editorFactory();

	console.log(editor);
}

export default function Page(): JSX.Element {
	const [isLoading, setIsLoading] = React.useState(false);

	return (
		<div>
			Our WASM component:
			{isLoading ? <div>Loading...</div> : null}
			<button
				onClick={() => {
					setIsLoading(true);
					onClick()
						.catch((err) => {
							console.log(err);
						})
						.finally(() => {
							setIsLoading(false);
						});
				}}
			>
				Load WASM
			</button>
		</div>
	);
}
