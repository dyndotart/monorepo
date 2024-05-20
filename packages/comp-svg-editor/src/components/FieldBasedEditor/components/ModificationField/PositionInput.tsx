import React from 'react';
import {
	applyModifications,
	type TModificationField,
	type TPositionModificationInput
} from '@dyn/comp-dtif';
import type { Composition } from '@dyn/comp-svg-builder';
import { AdvancedInput } from '@dyn/ui';

import { deterimeJsonFunctionExecutionEnv } from '../determine-json-function-execution-env';
import { runJsonFunction } from '../run-json-function';

export const PositionInput: React.FC<TProps> = (props) => {
	const { composition, field } = props;
	const [xValue, setXValue] = React.useState<number>(field.inputVariant.default[0]);
	const [yValue, setYValue] = React.useState<number>(field.inputVariant.default[1]);
	const [error, setError] = React.useState<string | null>(null);

	const onXChange = React.useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
		setXValue(parseFloat(e.target.value));
	}, []);

	const onYChange = React.useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
		setYValue(parseFloat(e.target.value));
	}, []);

	const onFocus = React.useCallback(
		(focus: boolean) => {
			if (focus) {
				return;
			}
			setError(null);

			// eslint-disable-next-line @typescript-eslint/no-floating-promises -- ok
			(async () => {
				// eslint-disable-next-line @typescript-eslint/await-thenable -- idk
				const processedActions = await applyModifications(
					field,
					{
						[field.key]: [xValue, yValue]
					},
					async (jsonFunction, args) =>
						runJsonFunction(jsonFunction, args, deterimeJsonFunctionExecutionEnv(jsonFunction))
				);

				for (const processedAction of processedActions) {
					if (processedAction.resolved) {
						composition.emitInputEvents('Dtif', processedAction.events);
						composition.update();
					} else {
						setError(processedAction.notMetConditions[0]?.message ?? null);
					}
				}
			})();
		},
		[xValue, yValue, field, composition]
	);

	return (
		<fieldset className="w-full rounded-lg border p-4">
			<legend className="-ml-1 px-1 text-sm font-medium">{field.displayName}</legend>
			<div className="grid grid-cols-2 gap-4">
				<AdvancedInput
					childrenAfter={<div />}
					className="pl-7"
					defaultValue={xValue}
					id="x"
					max={field.inputVariant.max?.[0]}
					min={field.inputVariant.min?.[0]}
					onBlur={() => {
						onFocus(false);
					}}
					onChange={onXChange}
					onFocus={() => {
						onFocus(true);
					}}
					type="number"
					variant={error != null ? 'destructive' : 'default'}
				>
					<div className="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
						<p className="text-gray-400">x</p>
					</div>
				</AdvancedInput>

				<AdvancedInput
					childrenAfter={<div />}
					className="pl-7"
					defaultValue={yValue}
					id="y"
					max={field.inputVariant.max?.[1]}
					min={field.inputVariant.min?.[1]}
					onBlur={() => {
						onFocus(false);
					}}
					onChange={onYChange}
					onFocus={() => {
						onFocus(true);
					}}
					type="number"
					variant={error != null ? 'destructive' : 'default'}
				>
					<div className="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
						<p className="text-gray-400">y</p>
					</div>
				</AdvancedInput>
			</div>
			{error != null ? (
				<p className="mt-2 text-sm text-red-600" id="email-error">
					{error}
				</p>
			) : null}
		</fieldset>
	);
};

interface TProps {
	composition: Composition;
	field: TModificationField<string, TPositionModificationInput>;
}
