import React, { type ReactNode } from 'react';
import { Link, useNavigate, type NavigateFunction } from 'react-router-dom';
import { Button, DiscordLogoIcon, GearIcon, Separator, TwitterLogoIcon } from '@dyn/ui';

import { EAppRoutes } from '../../../types';

export const Footer: React.FC<TProps> = (props) => {
	const { leftContent, rightContent } = props;
	const navigate = useNavigate();

	return (
		<>
			<Separator className="mb-2" />
			<div className={`flex h-6 w-full items-center justify-between px-4`}>
				{leftContent != null && (
					<div className="flex text-left">{renderLeftContent(leftContent)}</div>
				)}
				{rightContent != null && (
					<div className="flex text-right">{renderRightContent(rightContent, navigate)}</div>
				)}
			</div>
		</>
	);
};

const renderLeftContent = (leftContent: TLeftContent) => {
	if (typeof leftContent !== 'object' || leftContent == null || !('variant' in leftContent)) {
		return leftContent;
	}

	switch (leftContent.variant) {
		case 'version':
			return (
				<Link target="_blank" to={'https://dyn.art/v/changelog/figma?source=figma'}>
					<p className="text-sm text-gray-500">
						dyn.art <span className="text-xs">v0.0.1</span>
					</p>
				</Link>
			);
		default:
			return null;
	}
};

const renderRightContent = (rightContent: TRightContent, navigate: NavigateFunction) => {
	if (typeof rightContent !== 'object' || rightContent == null || !('variant' in rightContent)) {
		return rightContent;
	}

	switch (rightContent.variant) {
		case 'socials':
			return (
				<div className="flex">
					<Button variant="ghost" size="icon" asChild>
						<Link target="_blank" to={'https://s.dyn.art/discord?source=figma'}>
							<DiscordLogoIcon className="h-4 w-4" />
						</Link>
					</Button>
					<Button variant="ghost" size="icon" asChild>
						<Link target="_blank" to={'https://s.dyn.art/twitter?source=figma'}>
							<TwitterLogoIcon className="h-4 w-4" />
						</Link>
					</Button>
				</div>
			);
		case 'settings':
			return (
				<Button variant="ghost" size="icon">
					<Link to={EAppRoutes.SETTINGS}>
						<GearIcon className="h-4 w-4" />
					</Link>
				</Button>
			);
		default:
			return null;
	}
};

interface TProps {
	leftContent?: TLeftContent;
	rightContent?: TRightContent;
}

type TLeftContent = ReactNode | TLeftContentVariants;
type TLeftContentVariants = TLeftContentVersion;
interface TLeftContentVersion {
	variant: 'version';
}

type TRightContent = ReactNode | TRightContentVariants;
type TRightContentVariants = TRightContentSocials | TRightContentSettings;
interface TRightContentSocials {
	variant: 'socials';
}
interface TRightContentSettings {
	variant: 'settings';
}
