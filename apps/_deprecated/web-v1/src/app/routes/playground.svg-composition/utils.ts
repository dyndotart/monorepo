import { mat3, vec3 } from '@dyn/svg-composition';

export function createTransformMatrix(x: number, y: number, angleDegrees: number) {
	const angleRadians = (angleDegrees * Math.PI) / 180; // Convert angle to radians

	return mat3(
		vec3(Math.cos(angleRadians), -Math.sin(angleRadians), 0),
		vec3(Math.sin(angleRadians), Math.cos(angleRadians), 0),
		vec3(x, y, 1)
	);
}
