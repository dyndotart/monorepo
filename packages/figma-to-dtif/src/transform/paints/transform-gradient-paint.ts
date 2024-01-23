import type { COMP } from '@dyn/dtif';
import type { GradientVariant } from '@dyn/dtif/dist/types/gen/bindings';

import { mapFigmaBlendModeToDTIF, mapFigmaRGBToDTIF, mapFigmaTransformToMat3 } from '../../utils';

export function transformGradientPaint(
	paint: GradientPaint
): { type: 'Gradient' } & COMP.GradientPaintBundle {
	return {
		type: 'Gradient',
		compositionMixin: {
			isVisible: paint.visible ?? true
		},
		variant: mapFigmaGradientTypeToDTIF(paint.type),
		gradientStops: paint.gradientStops.map((stop) => ({
			color: mapFigmaRGBToDTIF(stop.color),
			position: stop.position
		})),
		transform: mapFigmaTransformToMat3(paint.gradientTransform),
		blendMixin: {
			blendMode: mapFigmaBlendModeToDTIF(paint.blendMode),
			opacity: paint.opacity ?? 1
		}
	};
}

function mapFigmaGradientTypeToDTIF(variant: GradientPaint['type']): GradientVariant {
	switch (variant) {
		case 'GRADIENT_LINEAR':
			return 'Linear';
		case 'GRADIENT_RADIAL':
			return 'Radial';
		case 'GRADIENT_ANGULAR':
			return 'Angular';
		case 'GRADIENT_DIAMOND':
			return 'Diamond';
	}
}
