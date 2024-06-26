import type { ARB } from '@dyn/arb-dtif';

import type {
	TToTransformEffect,
	TToTransformFill,
	TToTransformStroke
} from '../../FigmaNodeTreeProcessor';
import {
	createDtifStyles,
	mapFigmaBlendModeToDtif,
	mapFigmaElementLayoutToDtif,
	mapFigmaTransformToRotation,
	mapFigmaTransformToTranslation
} from '../../utils';

export function transformRectangleNode(
	node: RectangleNode,
	config: TTransformRectangleNodeConfig
): { type: 'Rectangle' } & ARB.RectangleNode {
	const { fills, strokes, effects, autoLayout } = config;

	return {
		type: 'Rectangle',
		visible: node.visible,
		size: [node.width, node.height],
		translation: mapFigmaTransformToTranslation(node.relativeTransform),
		rotationDeg: mapFigmaTransformToRotation(node.relativeTransform),
		cornerRadii: [
			node.topLeftRadius,
			node.topRightRadius,
			node.bottomRightRadius,
			node.bottomLeftRadius
		],
		blendMode: mapFigmaBlendModeToDtif(node.blendMode),
		opacity: node.opacity,
		layoutElement: mapFigmaElementLayoutToDtif(node, autoLayout),
		styles: createDtifStyles(fills, strokes, effects)
	};
}

interface TTransformRectangleNodeConfig {
	fills: TToTransformFill[];
	strokes: TToTransformStroke[];
	effects: TToTransformEffect[];
	autoLayout: boolean;
}
