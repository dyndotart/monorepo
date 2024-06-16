'use client';

import { zodResolver } from '@hookform/resolvers/zod';
import Link from 'next/link';
import React from 'react';
import { useForm } from 'react-hook-form';
import * as z from 'zod';
import {
	BlockMessage,
	Button,
	Form,
	FormControl,
	FormField,
	FormItem,
	FormLabel,
	FormMessage,
	GithubIcon,
	GoogleIcon,
	Input
} from '@dyn/ui';

import { AuthFormWrapper } from '../AuthFormWrapper';
import { LoginSchema } from './controller';

export const LoginForm: React.FC = () => {
	const form = useForm<z.infer<typeof LoginSchema>>({
		resolver: zodResolver(LoginSchema),
		defaultValues: {
			email: '',
			password: ''
		}
	});

	const onSubmit = React.useCallback((values: z.infer<typeof LoginSchema>) => {
		console.log(values);
	}, []);

	return (
		<AuthFormWrapper
			headerLabel="Sign in to your account"
			backChildren={
				<p className="font-body mt-4 text-center text-sm text-gray-500">
					Not a member?{' '}
					<Link
						href="/auth/register"
						className="ml-1 font-semibold leading-6 text-indigo-600 hover:text-indigo-500"
					>
						Start for free
					</Link>
				</p>
			}
		>
			<Form {...form}>
				<form onSubmit={form.handleSubmit(onSubmit)} className="space-y-6">
					<div className="space-y-4">
						<FormField
							control={form.control}
							name="email"
							render={({ field }) => (
								<FormItem>
									<FormLabel>Email</FormLabel>
									<FormControl>
										<Input {...field} placeholder="john.doe@example.com" type="email" />
									</FormControl>
									<FormMessage />
								</FormItem>
							)}
						/>
						<FormField
							control={form.control}
							name="password"
							render={({ field }) => (
								<FormItem>
									<FormLabel>Password</FormLabel>
									<FormControl>
										<Input {...field} placeholder="*******" type="password" />
									</FormControl>
									<FormMessage />
								</FormItem>
							)}
						/>
					</div>
					<BlockMessage variant="error">This is a message in a Block Message</BlockMessage>
					<Button type="submit" className="w-full">
						Sign in
					</Button>
				</form>
			</Form>

			<div className="flex w-full items-center gap-x-2">
				<Button size="lg" className="w-full" variant="outline">
					<GoogleIcon className="h-6 w-6" />
				</Button>
				<Button size="lg" className="w-full" variant="outline">
					<GithubIcon className="h-6 w-6" />
				</Button>
			</div>
		</AuthFormWrapper>
	);
};
