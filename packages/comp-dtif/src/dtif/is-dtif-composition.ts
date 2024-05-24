import type { COMP } from '../comp';

export function isDtifComposition(value: unknown): value is COMP.DtifComposition {
	if (typeof value !== 'object' || value == null) {
		return false;
	}
	const obj = value as Partial<COMP.DtifComposition>;
	return (
		Array.isArray(obj.size) &&
		// eslint-disable-next-line @typescript-eslint/no-unnecessary-condition -- Necessary here as we don't know how long the size array might be
		obj.size.length === 2 &&
		typeof obj.size[0] === 'number' &&
		typeof obj.size[1] === 'number' &&
		Array.isArray(obj.nodes) &&
		obj.nodes.length > 0 &&
		obj.nodes.every((node) => typeof node === 'object') &&
		(obj.paints == null ||
			(Array.isArray(obj.paints) && obj.paints.every((paint) => typeof paint === 'object'))) &&
		(obj.assets == null ||
			(Array.isArray(obj.assets) && obj.assets.every((asset) => typeof asset === 'object')))
	);
}
