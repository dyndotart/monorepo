import type { TFetchOptions, TFetchResponse } from './client';

export type TParseAs = 'json' | 'text' | 'blob' | 'arrayBuffer' | 'stream';
export type THttpMethod =
	| 'get'
	| 'put'
	| 'post'
	| 'delete'
	| 'options'
	| 'head'
	| 'patch'
	| 'trace';
export type TOkStatus = 200 | 201 | 202 | 203 | 204 | 206 | 207;
export type TErrorStatus =
	| 500
	| 400
	| 401
	| 402
	| 403
	| 404
	| 405
	| 406
	| 407
	| 408
	| 409
	| 410
	| 411
	| 412
	| 413
	| 414
	| 415
	| 416
	| 417
	| 418
	| 420
	| 421
	| 422
	| 423
	| 424
	| 425
	| 426
	| 429
	| 431
	| 444
	| 450
	| 451
	| 497
	| 498
	| 499
	| 'default';

// Http media type structure (e.g. 'application/json')
export type TMediaType = `${string}/${string}`;

export type THeadersInit = NonNullable<RequestInit['headers']>;
export type TRequestMethod = NonNullable<RequestInit['method']>;

export type TApiGet = <
	GSucessResponseBody = unknown,
	GErrorResponseBody = unknown,
	GParseAs extends TParseAs = 'json'
>(
	path: string,
	options: TFetchOptions<GParseAs>
) => Promise<TFetchResponse<GSucessResponseBody, GErrorResponseBody, GParseAs>>;

export type TApiPost = <
	GSuccessResponseBody = unknown,
	GErrorResponseBody = unknown,
	GRequestBody extends RequestInit['body'] = any,
	GParseAs extends TParseAs = 'json'
>(
	path: string,
	body: GRequestBody,
	options: TFetchOptions<GParseAs>
) => Promise<TFetchResponse<GSuccessResponseBody, GErrorResponseBody, GParseAs>>;

export type TApiPut = <
	GSuccessResponseBody = unknown,
	GErrorResponseBody = unknown,
	GRequestBody extends RequestInit['body'] = any,
	GParseAs extends TParseAs = 'json'
>(
	path: string,
	body: GRequestBody,
	options: TFetchOptions<GParseAs>
) => Promise<TFetchResponse<GSuccessResponseBody, GErrorResponseBody, GParseAs>>;

export type TApiDelete = <
	GSuccessResponseBody = unknown,
	GErrorResponseBody = unknown,
	GParseAs extends TParseAs = 'json'
>(
	path: string,
	options: TFetchOptions<GParseAs>
) => Promise<TFetchResponse<GSuccessResponseBody, GErrorResponseBody, GParseAs>>;

export interface TApiFeature {
	get: TApiGet;
	put: TApiPut;
	post: TApiPost;
	del: TApiDelete;
}
