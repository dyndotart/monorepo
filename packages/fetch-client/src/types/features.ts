import type { TUnionToIntersection } from '@dyn/types/utility';

import type { TApiFeature, TRawApiFeature } from './api-client';
import type { TOpenApiFeature } from './openapi-client';

export type TFeatures<GPaths extends {} = {}> = {
	base: { _: null }; // TODO: Placeholder Feature: Figure out how to make the TS infer work with [] (empty array -> no feature)
	api: TApiFeature;
	rapi: TRawApiFeature;
	openapi: TOpenApiFeature<GPaths>;
} & TThirdPartyFeatures;

// Global registry for third party features
// eslint-disable-next-line @typescript-eslint/no-empty-interface -- Overwritten by third party libraries
export interface TThirdPartyFeatures {}

export type TFeatureKeys<GPaths extends {} = {}> = keyof TFeatures<GPaths>;

export type TSelectFeatureObjects<
	GPaths extends {},
	GSelectedFeatureKeys extends TFeatureKeys<GPaths>[]
> = {
	[K in GSelectedFeatureKeys[number]]: TFeatures<GPaths>[K];
};

export type TSelectFeatures<
	GSelectedFeatureKeys extends TFeatureKeys<GPaths>[],
	GPaths extends {} = {},
	GSelectedFeatureObjects extends TSelectFeatureObjects<
		GPaths,
		GSelectedFeatureKeys
	> = TSelectFeatureObjects<GPaths, GSelectedFeatureKeys>
> = TUnionToIntersection<GSelectedFeatureObjects[keyof GSelectedFeatureObjects]>;

export type TEnforceFeatures<
	GFeatureKeys extends TFeatureKeys[],
	GToEnforceFeatureKeys extends TFeatureKeys[]
> = Exclude<GToEnforceFeatureKeys, GFeatureKeys> extends never
	? GFeatureKeys
	: GFeatureKeys | Exclude<GToEnforceFeatureKeys, GFeatureKeys>;
