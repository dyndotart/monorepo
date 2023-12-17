import React from 'react';
import type { FigmaAppHandler, TAppCallbackRegistration } from '@dyn/figma-handler/app';

export function useAppCallback<GFigmaAppHandler extends FigmaAppHandler>(
	appHandler: GFigmaAppHandler,
	registrations:
		| TAppCallbackRegistration<ExtractPluginMessageEvent<GFigmaAppHandler>>
		| TAppCallbackRegistration<ExtractPluginMessageEvent<GFigmaAppHandler>>[],
	deps: React.DependencyList = []
): void {
	React.useEffect(() => {
		const unregisterFunctions = appHandler.register(registrations);
		return () => {
			unregisterFunctions.forEach((unregisterFunction) => {
				unregisterFunction();
			});
		};
	}, [registrations, ...deps]);
}

type ExtractPluginMessageEvent<T> = T extends FigmaAppHandler<infer U> ? U : never;
