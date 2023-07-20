import micromatch from "micromatch";

/**
 * Filter a list of files to only get the ones that match the given glob patterns.
 * @param {string[]} files the list of files
 * @param {string[]} patterns the glob patterns to match
 * @returns the files from the list matching the glob patterns
 */
const getFilesForPatterns = (files, patterns) =>
  micromatch(files, patterns).map((path) => path.replace(process.cwd(), "."));

/**
 * This function runs before a commit happens (pre-commit hook).
 * It runs various checks based on the type of files that are currently staged.
 * The commit will be aborted if one error occurs in at least one of the checks.
 * @param {string[]} stagedFiles the list of files staged for the current commit
 * @returns the list of commands to run in order to check the staged files
 */
const processStagedFiles = (stagedFiles) => {
  const commands = [];

  const dataFiles = getFilesForPatterns(stagedFiles, ["**/*.{json,yaml}"]);
  const htmlFiles = getFilesForPatterns(stagedFiles, ["**/*.html"]);
  const rustFiles = getFilesForPatterns(stagedFiles, ["**/*.rs"]);
  const typescriptFiles = getFilesForPatterns(stagedFiles, ["**/*.{ts,tsx}"]);
  const javascriptFiles = getFilesForPatterns(stagedFiles, ["**/*.{js,cjs,mjs}"]);

  const filesToEslint = [...typescriptFiles, ...javascriptFiles];
  const filesToPrettify = [...dataFiles, ...htmlFiles, ...typescriptFiles, ...javascriptFiles];

  if (rustFiles.length) {
    // Format the staged Rust files
    commands.push("cargo fmt --manifest-path=./src-tauri/Cargo.toml");

    // Run a compiler check to make sure there are no compile-time errors
    commands.push("cargo check --manifest-path=./src-tauri/Cargo.toml --color=always");
  }

  if (filesToPrettify.length) {
    // Format the files that can be formatted using Prettier
    commands.push(`pnpm prettier --write ${filesToPrettify.join(" ")}`);
  }

  if (typescriptFiles.length) {
    // Run a TypeScript check using the tsc compiler when at least one
    // Typecript file is staged
    commands.push("pnpm tsc --pretty -p ./tsconfig.json");
  }

  if (filesToEslint.length) {
    // Run an ESLint check on the staged TypeScript and JavaScript files
    commands.push(`pnpm eslint ${filesToEslint.join(" ")}`);
  }

  return commands;
};

export default processStagedFiles;
