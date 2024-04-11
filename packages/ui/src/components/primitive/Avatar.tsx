/**
 * -----------------------------------------------------------------------------
 * This file includes code derived from the project shadcn-ui/ui by \@shadcn.
 * Project Repository: https://github.com/shadcn-ui/ui/blob/main/apps/www/registry/new-york/ui/avatar.tsx
 *
 * Date of Import: 03 April 2024
 * -----------------------------------------------------------------------------
 * The code included in this file is licensed under the MIT License,
 * as per the original project by \@shadcn.
 * For the license text, see: https://github.com/shadcn-ui/ui/blob/main/LICENSE.md
 * -----------------------------------------------------------------------------
 */

'use client';

import * as AvatarPrimitive from '@radix-ui/react-avatar';
import * as React from 'react';
import { cn } from '@/utils';

const Avatar = React.forwardRef<
	React.ElementRef<typeof AvatarPrimitive.Root>,
	React.ComponentPropsWithoutRef<typeof AvatarPrimitive.Root>
>(({ className, ...props }, ref) => (
	<AvatarPrimitive.Root
		className={cn('relative flex h-10 w-10 shrink-0 overflow-hidden rounded-full', className)}
		ref={ref}
		{...props}
	/>
));
Avatar.displayName = AvatarPrimitive.Root.displayName;

const AvatarImage = React.forwardRef<
	React.ElementRef<typeof AvatarPrimitive.Image>,
	React.ComponentPropsWithoutRef<typeof AvatarPrimitive.Image>
>(({ className, ...props }, ref) => (
	<AvatarPrimitive.Image
		className={cn('aspect-square h-full w-full', className)}
		ref={ref}
		{...props}
	/>
));
AvatarImage.displayName = AvatarPrimitive.Image.displayName;

const AvatarFallback = React.forwardRef<
	React.ElementRef<typeof AvatarPrimitive.Fallback>,
	React.ComponentPropsWithoutRef<typeof AvatarPrimitive.Fallback>
>(({ className, ...props }, ref) => (
	<AvatarPrimitive.Fallback
		className={cn(
			'bg-muted flex h-full w-full items-center justify-center rounded-full',
			className
		)}
		ref={ref}
		{...props}
	/>
));
AvatarFallback.displayName = AvatarPrimitive.Fallback.displayName;

// export * as AvatarPrimitive from '@radix-ui/react-avatar';
export { Avatar, AvatarFallback, AvatarImage };
