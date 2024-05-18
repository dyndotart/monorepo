import React from 'react';
import {
	applyModifications,
	type TModificationField,
	type TTextModificationInput
} from '@dyn/comp-dtif';
import type { Composition } from '@dyn/comp-svg-builder';
import { AdvancedInput, AdvancedTextarea } from '@dyn/ui';

export const TextInput: React.FC<TProps> = (props) => {
	const { composition, field } = props;
	const [value, setValue] = React.useState<string>(field.inputType.default);
	const [error, setError] = React.useState<string | null>(null);

	const onChange = React.useCallback(
		(e: React.ChangeEvent<HTMLInputElement> | React.ChangeEvent<HTMLTextAreaElement>) => {
			setValue(e.target.value);
		},
		[]
	);

	const onFocus = React.useCallback(
		(focus: boolean) => {
			if (focus) {
				return;
			}
			setError(null);

			const processedActions = applyModifications(field, {
				[field.key]: value
			});

			for (const processedAction of processedActions) {
				if (processedAction.resolved) {
					composition.emitInputEvents('Dtif', processedAction.events);
					composition.update();
				} else {
					setError(processedAction.notMetConditions[0]?.message ?? null);
				}
			}
		},
		[value, field, composition]
	);

	return (
		<fieldset className="w-full rounded-lg border p-4">
			<legend className="-ml-1 px-1 text-sm font-medium">{field.displayName}</legend>
			{field.inputType.area ? (
				<AdvancedTextarea
					childrenAfter={<div />}
					defaultValue={value}
					onBlur={() => {
						onFocus(false);
					}}
					onChange={onChange}
					onFocus={() => {
						onFocus(true);
					}}
					variant={error != null ? 'destructive' : 'default'}
				/>
			) : (
				<AdvancedInput
					childrenAfter={<div />}
					defaultValue={value}
					onBlur={() => {
						onFocus(false);
					}}
					onChange={onChange}
					onFocus={() => {
						onFocus(true);
					}}
					variant={error != null ? 'destructive' : 'default'}
				/>
			)}
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
	field: TModificationField<string, TTextModificationInput>;
}
