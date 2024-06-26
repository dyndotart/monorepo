import React from 'react';

export function useSizeCallback<T extends HTMLElement = HTMLElement>(
	ref: React.RefObject<T>,
	callback: TSizeCallback,
	callbackDeps: unknown[]
): void {
	React.useLayoutEffect(() => {
		const target = ref.current;
		if (target == null) {
			return;
		}

		const previousSize: Partial<TSize> = {
			width: undefined,
			height: undefined
		};

		const updateSize = (): void => {
			const newSize: TSize = {
				width: target.offsetWidth,
				height: target.offsetHeight
			};
			if (newSize.width !== previousSize.width || newSize.height !== previousSize.height) {
				previousSize.width = newSize.width;
				previousSize.height = newSize.height;
				callback(newSize);
			}
		};

		updateSize();

		if (typeof ResizeObserver === 'undefined') {
			console.warn(
				"The 'useDivSize()' hook requires 'ResizeObserver'. Your browser does not support 'ResizeObserver'."
			);
			return;
		}

		const resizeObserver = new ResizeObserver(() => {
			updateSize();
		});
		resizeObserver.observe(target);

		return () => {
			resizeObserver.disconnect();
		};
	}, [ref, ...callbackDeps]);
}

export interface TSize {
	width: number;
	height: number;
}

type TSizeCallback = (size: TSize) => void;
