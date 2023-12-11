import fs from 'node:fs/promises';
import path from 'node:path';
import { Flags } from '@oclif/core';
import chalk from 'chalk';
import type { PackageJson } from 'type-fest';

import { DynCommand } from '../../DynCommand';
import { doesFileExist, execaVerbose, promisifyFiglet, readJsonFile, shortId } from '../../utils';

export default class Rust extends DynCommand {
	static description = 'Bundle Rust part of dyn.art packages';

	static examples = [];

	static flags = {
		prod: Flags.boolean({
			char: 'p',
			description: 'Production mode',
			required: false,
			default: false
		}),
		analyze: Flags.boolean({
			char: 'a',
			description: 'Analyze bundle',
			required: false,
			default: false
		}),
		verbose: Flags.boolean({
			char: 'v',
			description: 'More detailed logs',
			required: false,
			default: false
		})
	};

	static args = {};

	public async run(): Promise<void> {
		const { flags } = await this.parse(Rust);
		this.isVerbose = flags.verbose;
		const startTime = Date.now();
		const tempRustOutputName = `temp-${shortId()}`;
		const rustModulesDirPath = path.join(process.cwd(), 'src', 'rust_modules');

		this.log(chalk.yellowBright(await promisifyFiglet('dyn brustler')));
		this.log(`\n`);
		this.log(
			`Started bundling Rust for ${
				flags.prod ? chalk.green('production') : chalk.blue('development')
			}`
		);
		this.log(`\n`);

		// Build WASM
		const rustInputDirPath = path.join(process.cwd(), 'rust');
		const rustOutputDirPath = path.join(rustModulesDirPath, tempRustOutputName);
		await execaVerbose(
			'wasm-pack',
			[
				'build',
				...(flags.prod ? ['--release'] : ['--dev']),
				'--target',
				'web',
				'--out-dir',
				rustOutputDirPath,
				...(flags.prod ? [] : ['--features', 'trace'])
			],
			{
				command: this,
				cwd: rustInputDirPath, // Set the cwd to the ./rust directory
				verbose: flags.verbose
			}
		);
		this.log(
			`Bundled Rust with ${chalk.underline('wasm-pack')} to ${chalk.gray(
				chalk.underline(rustOutputDirPath)
			)}`
		);

		// Generate type declarations for Typescript
		const bindingTypesOutputPath = path.join(rustOutputDirPath, './bindings.ts');
		await execaVerbose(
			'cargo',
			[
				'run',
				'--features',
				'cli',
				'--',
				'generate-ts-types',
				'--export-path',
				bindingTypesOutputPath
			],
			{
				command: this,
				cwd: rustInputDirPath,
				verbose: flags.verbose
			}
		);
		this.log(
			`Generated type declarations for Typescript to ${chalk.gray(
				chalk.underline(bindingTypesOutputPath)
			)}`
		);

		// Read in package.json and extract name of Rust module
		const rustOutputPackageJsonPath = path.join(rustOutputDirPath, 'package.json');
		const rustOutputPackageJson = await readJsonFile<PackageJson>(rustOutputPackageJsonPath);
		if (rustOutputPackageJson == null) {
			this.error(`No package.json file found at '${chalk.underline(process.cwd())}'!`, { exit: 1 });
		}
		const rustAppName = rustOutputPackageJson.name ?? 'rust-app';
		const rustAppNameUnderscored = rustAppName.replace(/-/g, '_');

		// Rename files in the output directory
		const filesToRename = [
			{ original: `${rustAppNameUnderscored}.js`, new: 'index.js' },
			{ original: `${rustAppNameUnderscored}.d.ts`, new: 'index.d.ts' },
			{ original: `${rustAppNameUnderscored}_bg.wasm`, new: 'bg.wasm' },
			{ original: `${rustAppNameUnderscored}_bg.wasm.d.ts`, new: 'bg.wasm.d.ts' }
		];
		for (const file of filesToRename) {
			await fs.rename(
				path.join(rustOutputDirPath, file.original),
				path.join(rustOutputDirPath, file.new)
			);
		}

		// Delete not required files in the output directory
		const filesToDelte = ['package.json', '.gitignore'];
		for (const file of filesToDelte) {
			const filePath = path.join(rustOutputDirPath, file);
			if (doesFileExist(filePath)) {
				await fs.rm(filePath);
			}
		}

		// Analyze wasm file size
		// https://rustwasm.github.io/docs/book/reference/code-size.html
		if (flags.analyze) {
			await execaVerbose('twiggy', ['top', '-n', '20', path.join(rustOutputDirPath, 'bg.wasm')], {
				command: this,
				cwd: rustInputDirPath,
				verbose: true
			});
		}

		// Rename output directory
		await fs.rename(rustOutputDirPath, path.join(rustModulesDirPath, rustAppName));

		this.log(`\n`);
		this.log(
			`${chalk.green('→')} Rust was bundled in ${chalk.green(
				chalk.underline(`${((Date.now() - startTime) / 1000).toFixed(2)}s`)
			)}.`
		);
		this.log(`\n`);
		this.exit(0);
	}
}
