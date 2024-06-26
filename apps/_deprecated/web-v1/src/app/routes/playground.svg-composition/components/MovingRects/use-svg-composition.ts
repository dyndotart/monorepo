import React from 'react';
import { Composition, createSVGComposition, Entity, initWasm } from '@dyn/svg-composition';

export const useSVGComposition = (props: UseSVGCompositionProps) => {
	const { width, height, count = 50 } = props;
	const svgContainerRef = React.useRef<HTMLDivElement>(null);
	const [composition, setComposition] = React.useState<Composition | null>(null);
	const [selectedEntities, setSelectedEntities] = React.useState<Entity[]>([]);
	const [prevSelectedEntities, setPrevSelectedEntities] = React.useState<Entity[]>([]);
	const [selectedEntityData, setSelectedEntityData] = React.useState<{
		x: number;
		y: number;
	} | null>(null);

	let isMounted = true; // https://github.com/facebook/react/issues/24502
	React.useEffect(() => {
		(async () => {
			if (svgContainerRef.current != null && composition == null && isMounted) {
				const newComposition = await createComposition({
					width,
					height,
					element: svgContainerRef.current
				});
				setComposition(newComposition);
				startLoop({ composition: newComposition, count });

				newComposition.onSelectionChange((selected) => {
					setSelectedEntities(selected);
				});
			}
		})();
		return () => {
			isMounted = false;
			if (composition != null) {
				composition.clear();
			}
		};
	}, [width, height, count, svgContainerRef.current]);

	React.useEffect(() => {
		const selectedEntity = selectedEntities.length > 0 ? (selectedEntities[0] as Entity) : null;
		const prevSelectedEntity =
			prevSelectedEntities.length > 0 ? (prevSelectedEntities[0] as Entity) : null;

		// Unwatch previous selected entity
		if (prevSelectedEntity != null) {
			composition?.unwatchEntity(prevSelectedEntity);
		}

		// Watch newly selected entity
		if (selectedEntity != prevSelectedEntity && selectedEntity != null) {
			// TODO: when selecting a new entity that is static (-> no transform event)
			//  it won't update x and y (e.g. Root Frame)
			composition?.watchEntity(selectedEntity, ['RelativeTransform'], (_, changes) => {
				for (const change of changes) {
					switch (change.type) {
						case 'RelativeTransform':
							setSelectedEntityData({
								x: Math.round(change.relativeTransform[6]),
								y: Math.round(change.relativeTransform[7])
							});
							break;
						default:
						// do nothing
					}
				}
			});
			setPrevSelectedEntities(selectedEntities);
		}
	}, [selectedEntities, composition]);

	return { svgContainerRef, composition, selectedEntityData, selectedEntities };
};

async function createComposition(config: {
	width: number;
	height: number;
	element: Element;
}): Promise<Composition> {
	const { width, height, element } = config;
	await initWasm();

	const composition = createSVGComposition({
		width,
		height,
		renderer: {
			domElement: element
		},
		isCallbackBased: false
	});

	return composition;
}

function startLoop(config: { count: number; composition: Composition }) {
	const { count, composition } = config;

	// Set up your rectangles
	const rects: Record<
		string,
		{ x: number; y: number; size: number; speed: number; entity: Entity }
	> = {};
	for (let i = 0; i < count; i++) {
		const x = Math.random() * composition.width;
		const y = Math.random() * composition.height;
		const size = 10 + Math.random() * 40;
		const speed = 1 + Math.random();

		rects[i] = {
			x,
			y,
			size,
			speed,
			entity: composition.createRectangle({
				x,
				y,
				width: size,
				height: size,
				color: generateRandomRGBColor()
			})
		};
	}

	// Animation loop
	const animate = () => {
		const rectKeysToRemove: string[] = [];

		for (const key in rects) {
			const rect = rects[key];
			if (rect == null) {
				continue;
			}

			rect.x -= rect.speed;
			composition.setEntityPosition(rect.entity, rect.x, rect.y);

			if (rect.x + rect.size / 2 < 0) {
				rectKeysToRemove.push(key);
			}
		}

		rectKeysToRemove.forEach((key) => {
			const rect = rects[key];
			if (rect != null) {
				rect.x = composition.width + rect.size / 2;
			}
		});

		composition.update();

		requestAnimationFrame(animate);
	};

	// Start animation loop
	requestAnimationFrame(animate);
}

function generateRandomRGBColor(): [number, number, number] {
	// Generate a random integer between 0 and 255 for each color component
	const r = Math.floor(Math.random() * 256); // Red
	const g = Math.floor(Math.random() * 256); // Green
	const b = Math.floor(Math.random() * 256); // Blue

	return [r, g, b];
}

type UseSVGCompositionProps = {
	width: number;
	height: number;
	count?: number;
};
