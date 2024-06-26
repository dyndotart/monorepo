import type { ARB } from '@dyn/arb-dtif';

import type {
	TToTransformEffect,
	TToTransformFill,
	TToTransformStroke
} from '../../FigmaNodeTreeProcessor';
import { mapFigmaRGBToDtif } from '../mapper';
import { mapFigmaBlendModeToDtif } from '../mapper/map-figma-blend-mode-to-dtif';

export function createDtifStyles(
	fills: TToTransformFill[],
	strokes: TToTransformStroke[],
	effects: TToTransformEffect[]
): ARB.Style[] {
	const fillStyles = fills.map(
		(fill) =>
			({
				type: 'Fill',
				paintId: { type: 'ReferenceId', referenceId: `p${fill.paintId}` },
				blendMode: mapFigmaBlendModeToDtif(fill.blendMode),
				opacity: fill.opacity,
				visible: fill.visible
			}) as ARB.Style
	);

	const centerStrokeStyles: ARB.Style[] = [];
	const outsideStrokeStyles: ARB.Style[] = [];
	for (const stroke of strokes) {
		const strokeStyle: ARB.Style = {
			type: 'Stroke',
			width: stroke.width,
			paintId: { type: 'ReferenceId', referenceId: `p${stroke.paintId}` },
			blendMode: mapFigmaBlendModeToDtif(stroke.blendMode),
			opacity: stroke.opacity,
			visible: stroke.visible
		};
		switch (stroke.strokeAlign) {
			case 'CENTER':
				centerStrokeStyles.push(strokeStyle);
				break;
			case 'OUTSIDE':
				outsideStrokeStyles.push(strokeStyle);
				break;
			default:
				console.warn(`Unsuported stroke align: ${stroke.strokeAlign}`);
		}
	}

	const effectStyles: ARB.Style[] = [];
	for (const effect of effects) {
		switch (effect.variant.type) {
			case 'DROP_SHADOW': {
				const dropShadow = effect.variant;
				effectStyles.push({
					type: 'DropShadow',
					color: mapFigmaRGBToDtif(dropShadow.color),
					position: [dropShadow.offset.x, dropShadow.offset.y],
					blur: dropShadow.radius,
					spread: dropShadow.spread,
					visible: dropShadow.visible,
					blendMode: mapFigmaBlendModeToDtif(dropShadow.blendMode),
					opacity: dropShadow.color.a
				});
				break;
			}
			default:
				console.warn(`Unsuported stroke align: ${effect.variant.type}`);
		}
	}

	return [...centerStrokeStyles, ...fillStyles, ...outsideStrokeStyles, ...effectStyles];
}
