import { cssBundleHref } from '@remix-run/css-bundle';
import type { LinksFunction } from '@remix-run/node';
import { Links, LiveReload, Meta, Outlet, Scripts, ScrollRestoration } from '@remix-run/react';
import React from 'react';
import uiStyles from '@dyn/ui/dist/styles.css';

import styles from './tailwind.css';

export const links: LinksFunction = () => [
	{ rel: 'stylesheet', href: uiStyles },
	{ rel: 'stylesheet', href: styles },
	...(cssBundleHref ? [{ rel: 'stylesheet', href: cssBundleHref }] : [])
];

const App: React.FC = () => {
	return (
		<html lang="en">
			<head>
				<meta charSet="utf-8" />
				<meta name="viewport" content="width=device-width, initial-scale=1" />
				<Meta />
				<Links />
			</head>
			<body>
				<Outlet />
				<ScrollRestoration />
				<Scripts />
				<LiveReload />
			</body>
		</html>
	);
};

export default App;
