'use client';

import Link from 'next/link';
import { usePathname } from 'next/navigation';
import React from 'react';
import { Button, cn, LogoIcon } from '@dyn/ui';

import { TNavLink } from './types';

export const DesktopNavContent: React.FC<TProps> = (props) => {
	const { links, className } = props;
	const pathname = usePathname();
	const lastPath = React.useMemo(() => `/${pathname.split('/').pop()}`, [pathname]);

	return (
		<nav
			className={cn(
				'border-border flex h-[50px] items-center rounded-md border px-4 backdrop-blur-xl backdrop-filter',
				className
			)}
		>
			<Link href="/">
				<span className="sr-only">dyn.art Logo</span>
				<LogoIcon className="h-6 w-6" />
			</Link>

			<ul className="mx-4 flex space-x-2 text-sm font-medium">
				{links.map(({ path, title }) => {
					const isActive = path === lastPath;

					return (
						<li key={path}>
							<Button
								asChild
								variant={'ghost'}
								className={cn('h-8 px-3 py-2', isActive && 'underline')}
							>
								<Link href={path}>{title}</Link>
							</Button>
						</li>
					);
				})}
			</ul>

			<div className="border-border h-6 border-l-[1px]" />

			<Button asChild variant={'ghost'} className="ml-4 px-3 py-2 font-medium">
				<Link href={'https://dyn.art/app'}>Join Waitlist</Link>
			</Button>
		</nav>
	);
};

interface TProps {
	links: TNavLink[];
	className?: string;
}
