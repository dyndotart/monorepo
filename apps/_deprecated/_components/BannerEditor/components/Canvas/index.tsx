'use client';

import dynamic from 'next/dynamic';
import React from 'react';
import {
	applyCanvasDimensions,
	createSVGComposition,
	type COMP,
	type Composition
} from '@dyn/svg-composition';
import { Skeleton } from '@dyn/ui';

import { CanvasInner } from './CanvasInner';

export const Canvas = dynamic(
	async () => {
		// Load and initialize WASM before the component is returned
		const { initWasm } = await import('@dyn/svg-composition');
		await initWasm();

		// eslint-disable-next-line react/function-component-definition, react/display-name -- Inline return
		return (props: TCanvasProps) => {
			const { width, height, dtif, onLoadedComposition, ...other } = props;
			const svgContainerRef = React.useRef<HTMLDivElement>(null);
			const [composition, setComposition] = React.useState<Composition | null>(null);

			let isMounted = true; // https://github.com/facebook/react/issues/24502
			React.useEffect(() => {
				if (svgContainerRef.current != null && composition == null && isMounted) {
					const newComposition = createSVGComposition({
						render: {
							domElement: svgContainerRef.current
						},
						dtif: applyCanvasDimensions(dtif, { width, height })
					});
					console.log('Loaded', { newComposition, dtif });
					setComposition(newComposition);
					newComposition.update();
					onLoadedComposition(newComposition);
				}

				return () => {
					isMounted = false;
					if (composition != null) {
						composition.unmount();
					}
				};
			}, [composition, width, height, dtif]);

			// TODO: Rn that is not possible as we rely on the svgContainerRef
			//  -> We can work around that by decoupeling SVGComposition from SVGRenderer
			// if (composition == null) {
			// 	return <Skeleton className="h-full w-full" />;
			// }

			return (
				<CanvasInner
					composition={composition ?? undefined}
					svgContainerRef={svgContainerRef}
					{...other}
				/>
			);
		};
	},
	{
		loading: () => <Skeleton className="h-full w-full" />,
		ssr: false
	}
);

export type TCanvasProps = {
	width: number;
	height: number;
	dtif: COMP.DTIFComposition;
	onLoadedComposition: (composition: Composition) => void;
} & React.HTMLAttributes<HTMLDivElement>;
