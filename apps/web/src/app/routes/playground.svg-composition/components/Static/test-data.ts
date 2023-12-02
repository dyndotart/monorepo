import { DTIFComposition, FontWithContent, mat3, vec3 } from '@dyn/svg-composition';

export const COMPOSITION_WITH_ONE_RECT = (
	width: number,
	height: number,
	fonts: FontWithContent[]
): DTIFComposition => ({
	version: '0.0.1',
	name: 'Test',
	width,
	height,
	rootNodeId: 0,
	nodes: {
		0: {
			type: 'Frame',
			children: [1, 2],
			dimension: {
				width,
				height
			},
			relativeTransform: mat3(vec3(1, 0, 0), vec3(0, 1, 0), vec3(0, 0, 1))
		},
		1: {
			type: 'Rectangle',
			compositionMixin: { isVisible: true, isLocked: false },
			dimension: {
				width: 100,
				height: 100
			},
			relativeTransform: mat3(
				vec3(1, 0, 0),
				vec3(0, 1, 0),
				vec3((width - 100) / 2, (height - 100) / 2, 1)
			),
			rectangleCornerMixin: {
				bottomLeftRadius: 20,
				bottomRightRadius: 0,
				topLeftRadius: 0,
				topRightRadius: 0
			},
			fill: {
				paints: [5]
			}
		},
		2: {
			type: 'Text',
			text: {
				sections: [
					{
						value: 'Hello there',
						style: {
							fontHash: 123,
							fontSize: 10
						}
					}
				]
			},
			compositionMixin: { isVisible: true, isLocked: false },
			dimension: {
				width: 100,
				height: 100
			},
			relativeTransform: mat3(
				vec3(1, 0, 0),
				vec3(0, 1, 0),
				vec3((width - 100) / 2, (height - 100) / 2, 1)
			),
			fill: {
				paints: [5]
			}
		}
	},
	paints: {
		5: {
			type: 'Solid',
			blendMode: 'Normal',
			color: [189, 183, 107],
			isVisible: true,
			opacity: 1
		}
	},
	fonts,
	changes: [
		{
			type: 'EntityMoved',
			entity: 1,
			dx: 100,
			dy: -300
		}
	]
});
