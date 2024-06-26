import { cva } from 'class-variance-authority';
import React from 'react';
import { cn } from '@/utils';

import { Slot } from '../Slot';

// https://tailwindui.com/components/application-ui/layout/containers
const layoutVariants = cva('mx-auto w-full overflow-y-auto overflow-x-hidden', {
	variants: {
		size: {
			default: 'px-4 sm:px-6 lg:px-8 max-w-screen-xl',
			compact: 'px-4 sm:px-6 lg:px-8 max-w-screen-lg',
			full: 'px-0 min-h-full',
			article: 'px-4 sm:px-6 lg:px-8 max-w-[65ch]'
		}
	},
	defaultVariants: {
		size: 'default'
	}
});

export const LayoutWrapper = React.forwardRef<HTMLDivElement, TProps>((props, ref) => {
	const { children, asChild, size, className, ...other } = props;
	const Comp = asChild ? Slot : 'div';

	return (
		<Comp className={cn(layoutVariants({ size }), className)} ref={ref} {...other}>
			{children}
		</Comp>
	);
});
LayoutWrapper.displayName = 'LayoutWrapper';

interface TProps extends React.HTMLAttributes<HTMLDivElement> {
	asChild?: boolean;
	size?: 'default' | 'compact' | 'full' | 'article';
}
