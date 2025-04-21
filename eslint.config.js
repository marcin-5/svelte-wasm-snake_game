// Import libraries
import js from '@eslint/js';
import eslintConfigPrettier from 'eslint-config-prettier';
import prettierPlugin from 'eslint-plugin-prettier';
import svelte from 'eslint-plugin-svelte';
import fs from 'node:fs';
import { fileURLToPath } from 'node:url';
import ts from 'typescript-eslint';

// Paths and constant definitions
const GITIGNORE_PATH = fileURLToPath(new URL('.gitignore', import.meta.url));
const SUPPORTED_FILE_TYPES = ['**/*.svelte', '**/*.ts', '**/*.js', '*.ts', '*.js'];

// Read ignore patterns from .gitignore and ensure `node_modules` is explicitly included
const GITIGNORE_PATTERNS = [
  'node_modules', // Explicitly ensure node_modules is ignored
  ...fs
    .readFileSync(GITIGNORE_PATH, 'utf-8')
    .split('\n')
    .filter((line) => line.trim() && !line.startsWith('#')), // Skip comments and empty lines
];

// ESLint configuration as flat config objects
const eslintConfig = [
  {
    // Define the ignored files and directories
    ignores: GITIGNORE_PATTERNS,
  },
  {
    files: SUPPORTED_FILE_TYPES, // Match specific file types
    languageOptions: {
      parserOptions: {
        projectService: true,
        extraFileExtensions: ['.svelte'],
        parser: ts.parser,
      },
    },
    plugins: {
      svelte,
      prettier: prettierPlugin,
    },
    rules: {
      'prettier/prettier': 'error', // Enforce Prettier's rules
    },
  },
  js.configs.recommended,
  ...ts.configs.recommended, // TypeScript ESLint recommended config
  ...svelte.configs.recommended, // Svelte-specific rules
  eslintConfigPrettier, // Disable conflicting formatting rules
];

// Export the configuration properly
export default eslintConfig;
