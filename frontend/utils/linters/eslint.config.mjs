import { defineConfig } from 'eslint/config'
import js from '@eslint/js'
import globals from 'globals'
import eslintReact from 'eslint-plugin-react'
import eslintReactHooks from 'eslint-plugin-react-hooks'
import eslintReactRefresh from 'eslint-plugin-react-refresh'
import prettierPlugin from 'eslint-plugin-prettier'
import tseslint from 'typescript-eslint'
import eslintConfigPrettier from 'eslint-config-prettier'
import eslintPluginPrettierRecommended from 'eslint-plugin-prettier/recommended'
import fs from 'fs'
import path from 'path'

const PACKAGE_DIRS = ['app', 'packages']

function findTsConfigs() {
  const tsConfigs = []

  for (const baseDir of PACKAGE_DIRS) {
    if (!fs.existsSync(baseDir)) continue

    if (baseDir === 'app') {
      const tsConfigPath = path.join(baseDir, 'tsconfig.json')
      if (fs.existsSync(tsConfigPath)) {
        tsConfigs.push({
          files: [`${baseDir}/**/*.{ts,tsx}`],
          languageOptions: {
            parserOptions: {
              project: [`./${tsConfigPath}`],
              tsconfigRootDir: process.cwd(),
            },
          },
        })
      }
    } else {
      const packageSubdirs = fs
        .readdirSync(baseDir, { withFileTypes: true })
        .filter((dirent) => dirent.isDirectory())
        .map((dirent) => `${baseDir}/${dirent.name}`)

      for (const dir of packageSubdirs) {
        const tsConfigPath = path.join(dir, 'tsconfig.json')
        if (fs.existsSync(tsConfigPath)) {
          tsConfigs.push({
            files: [`${dir}/**/*.{ts,tsx}`],
            languageOptions: {
              parserOptions: {
                project: [`./${tsConfigPath}`],
                tsconfigRootDir: process.cwd(),
              },
            },
          })
        }
      }
    }
  }

  return tsConfigs
}

export default defineConfig([
  js.configs.recommended,
  ...tseslint.configs.recommended,
  eslintConfigPrettier,
  eslintPluginPrettierRecommended,
  {
    ignores: [
      'node_modules/**/*',
      'coverage/**/*',
      'eslint.config.mjs',
      'commitlint.config.mjs',
      'eslint.config.mjs',
      '.next/**/*',
      'out/**/*',
      'dist/**/*',
      'next-env.d.ts',
    ],
  },
  {
    files: ['**/*.{js,jsx,ts,tsx}'],
    plugins: {
      '@typescript-eslint': tseslint.plugin,
      react: eslintReact,
      'react-hooks': eslintReactHooks,
      'react-refresh': eslintReactRefresh,
      prettier: prettierPlugin,
    },
    settings: {
      react: {
        version: 'detect',
      },
    },
    languageOptions: {
      globals: {
        ...globals.browser,
        ...globals.node,
        ...globals.es2020,
      },
      parser: tseslint.parser,
    },
  },
  ...findTsConfigs(),
  {
    files: ['**/*.{js,jsx}'],
    rules: {
      'react-refresh/only-export-components': [
        'warn',
        { allowConstantExport: true },
      ],
      'prefer-const': 'error',
      'react/jsx-curly-brace-presence': [
        'warn',
        { props: 'never', children: 'never' },
      ],
      'react/function-component-definition': [
        'warn',
        { namedComponents: 'arrow-function' },
      ],
      'react/self-closing-comp': ['error', { component: true, html: true }],
      'max-lines': ['warn', { max: 200 }],
      'max-params': ['error', 3],
      '@typescript-eslint/no-unused-vars': 'off',
    },
  },
  {
    files: ['**/*.{ts,tsx}'],
    rules: {
      'react-refresh/only-export-components': [
        'warn',
        { allowConstantExport: true },
      ],
      'prefer-const': 'error',
      'react/jsx-curly-brace-presence': [
        'warn',
        { props: 'never', children: 'never' },
      ],
      'react/self-closing-comp': ['error', { component: true, html: true }],
      'max-lines': ['warn', { max: 200 }],
      'max-params': ['error', 3],
      '@typescript-eslint/no-unused-vars': [
        'warn',
        { args: 'none', ignoreRestSiblings: true },
      ],
    },
  },
])
