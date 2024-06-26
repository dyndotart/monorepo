import React from 'react';
import type { COMP, Composition } from '@dyn/svg-composition';

import { useInteractionMode, useSelectedNodes } from '../../../../hooks';
import { InnerSelectionBox } from './InnerSelectionBox';
import type { EHandleSide } from './InnerSelectionBox/controller';

export const SelectionBox: React.FC<TProps> = (props) => {
	const {
		composition,
		onResizeHandlePointerDown,
		onResizeHandlePointerUp,
		onRotateHandlePointerDown,
		onRotateHandlePointerUp
	} = props;
	const selectedEntities = useSelectedNodes(composition);
	const interactionMode = useInteractionMode(composition);

	// TODO: Improve and centralize those events
	React.useEffect(() => {
		const handleKeyPressed = (event: KeyboardEvent) => {
			if (event.key === 'Delete') {
				selectedEntities.forEach((entity) => {
					composition.emitCoreEvents([{ type: 'EntityDeleted', entity }]);
					composition.update();
				});
			}
		};

		window.addEventListener('keydown', handleKeyPressed);

		return () => {
			window.removeEventListener('keydown', handleKeyPressed);
		};
	}, [selectedEntities]);

	return (
		<>
			{selectedEntities.map((selectedEntity) => (
				<InnerSelectionBox
					composition={composition}
					entity={selectedEntity}
					key={selectedEntity}
					onResizeHandlePointerDown={onResizeHandlePointerDown}
					onResizeHandlePointerUp={onResizeHandlePointerUp}
					onRotateHandlePointerDown={onRotateHandlePointerDown}
					onRotateHandlePointerUp={onRotateHandlePointerUp}
					showHandles={
						interactionMode.type !== 'Translating' && interactionMode.type !== 'Rotating'
					}
				/>
			))}
		</>
	);
};

interface TProps {
	composition: Composition;
	onResizeHandlePointerDown: (
		corner: EHandleSide,
		initialBounds: COMP.XYWH,
		rotationInRadians: number
	) => void;
	onResizeHandlePointerUp: (position: COMP.Vec2) => void;
	onRotateHandlePointerDown: (corner: EHandleSide, rotationInRadians: number) => void;
	onRotateHandlePointerUp: (position: COMP.Vec2) => void;
}
