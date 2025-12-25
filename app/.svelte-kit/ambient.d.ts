// this file is generated — do not edit it

/// <reference types="@sveltejs/kit" />

/**
 * Environment variables [loaded by Vite](https://vitejs.dev/guide/env-and-mode.html#env-files) from `.env` files and `process.env`. Like [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), this module cannot be imported into client-side code. This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://svelte.dev/docs/kit/configuration#env) (if configured).
 *
 * _Unlike_ [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), the values exported from this module are statically injected into your bundle at build time, enabling optimisations like dead code elimination.
 *
 * ```ts
 * import { API_KEY } from '$env/static/private';
 * ```
 *
 * Note that all environment variables referenced in your code should be declared (for example in an `.env` file), even if they don't have a value until the app is deployed:
 *
 * ```
 * MY_FEATURE_FLAG=""
 * ```
 *
 * You can override `.env` values from the command line like so:
 *
 * ```bash
 * MY_FEATURE_FLAG="enabled" npm run dev
 * ```
 */
declare module '$env/static/private' {
	export const _ZO_DOCTOR: string;
	export const NIX_PROFILES: string;
	export const TERM_PROGRAM: string;
	export const XDG_DATA_HOME: string;
	export const NODE: string;
	export const ASDF_INSTALL_TYPE: string;
	export const INIT_CWD: string;
	export const TMUX_TMPDIR: string;
	export const _P9K_TTY: string;
	export const ASDF_DIR: string;
	export const TERM: string;
	export const SHELL: string;
	export const HOMEBREW_REPOSITORY: string;
	export const TMPDIR: string;
	export const npm_config_npm_globalconfig: string;
	export const TERM_PROGRAM_VERSION: string;
	export const FZF_TMUX: string;
	export const TAURI_CLI_VERBOSITY: string;
	export const SPINNER_PID: string;
	export const TAURI_ENV_DEBUG: string;
	export const npm_config_registry: string;
	export const ASDF_INSTALL_PATH: string;
	export const ZSH_TMUX_TERM: string;
	export const USER: string;
	export const TAURI_ENV_TARGET_TRIPLE: string;
	export const ASDF_INSTALL_VERSION: string;
	export const COMMAND_MODE: string;
	export const PNPM_SCRIPT_SRC_DIR: string;
	export const npm_config_globalconfig: string;
	export const _ZSH_TMUX_FIXED_CONFIG: string;
	export const SSH_AUTH_SOCK: string;
	export const __CF_USER_TEXT_ENCODING: string;
	export const npm_execpath: string;
	export const FZF_DEFAULT_OPTS: string;
	export const PAGER: string;
	export const ZSH_TMUX_CONFIG: string;
	export const TAURI_ENV_PLATFORM: string;
	export const TMUX: string;
	export const WEZTERM_EXECUTABLE_DIR: string;
	export const npm_config_frozen_lockfile: string;
	export const npm_config_verify_deps_before_run: string;
	export const XDG_CONFIG_DIRS: string;
	export const PATH: string;
	export const TERMINFO_DIRS: string;
	export const npm_package_json: string;
	export const FLOAX_WIDTH: string;
	export const TAURI_ENV_FAMILY: string;
	export const TAURI_ENV_PLATFORM_VERSION: string;
	export const __CFBundleIdentifier: string;
	export const resurrect_dir: string;
	export const NIX_PATH: string;
	export const PWD: string;
	export const npm_command: string;
	export const P9K_SSH: string;
	export const npm_lifecycle_event: string;
	export const EDITOR: string;
	export const FLOAX_BORDER_COLOR: string;
	export const npm_config__jsr_registry: string;
	export const npm_package_name: string;
	export const LANG: string;
	export const P9K_TTY: string;
	export const WEZTERM_PANE: string;
	export const XDG_STATE_HOME: string;
	export const NODE_PATH: string;
	export const TMUX_PANE: string;
	export const XPC_FLAGS: string;
	export const TAURI_ENV_ARCH: string;
	export const NIX_SSL_CERT_FILE: string;
	export const FLOAX_TITLE: string;
	export const npm_config_node_gyp: string;
	export const XPC_SERVICE_NAME: string;
	export const pnpm_config_verify_deps_before_run: string;
	export const npm_package_version: string;
	export const WEZTERM_UNIX_SOCKET: string;
	export const FLOAX_HEIGHT: string;
	export const YSU_VERSION: string;
	export const SHLVL: string;
	export const HOME: string;
	export const FLOAX_CHANGE_PATH: string;
	export const XDG_CONFIG_HOME: string;
	export const ATUIN_HISTORY_ID: string;
	export const HOMEBREW_PREFIX: string;
	export const ZSH_CACHE_DIR: string;
	export const WEZTERM_CONFIG_DIR: string;
	export const XDG_CACHE_HOME: string;
	export const LOGNAME: string;
	export const MACOSX_DEPLOYMENT_TARGET: string;
	export const FLOAX_TEXT_COLOR: string;
	export const ATUIN_SESSION: string;
	export const npm_lifecycle_script: string;
	export const XDG_DATA_DIRS: string;
	export const FZF_DEFAULT_COMMAND: string;
	export const BROWSER: string;
	export const npm_config_user_agent: string;
	export const HOMEBREW_CELLAR: string;
	export const INFOPATH: string;
	export const _P9K_SSH_TTY: string;
	export const WEZTERM_EXECUTABLE: string;
	export const __HM_SESS_VARS_SOURCED: string;
	export const WEZTERM_CONFIG_FILE: string;
	export const npm_node_execpath: string;
	export const __HM_ZSH_SESS_VARS_SOURCED: string;
	export const NIX_USER_PROFILE_DIR: string;
	export const __NIX_DARWIN_SET_ENVIRONMENT_DONE: string;
	export const COLORTERM: string;
	export const NODE_ENV: string;
}

/**
 * Similar to [`$env/static/private`](https://svelte.dev/docs/kit/$env-static-private), except that it only includes environment variables that begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) (which defaults to `PUBLIC_`), and can therefore safely be exposed to client-side code.
 *
 * Values are replaced statically at build time.
 *
 * ```ts
 * import { PUBLIC_BASE_URL } from '$env/static/public';
 * ```
 */
declare module '$env/static/public' {}

/**
 * This module provides access to runtime environment variables, as defined by the platform you're running on. For example if you're using [`adapter-node`](https://github.com/sveltejs/kit/tree/main/packages/adapter-node) (or running [`vite preview`](https://svelte.dev/docs/kit/cli)), this is equivalent to `process.env`. This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://svelte.dev/docs/kit/configuration#env) (if configured).
 *
 * This module cannot be imported into client-side code.
 *
 * Dynamic environment variables cannot be used during prerendering.
 *
 * ```ts
 * import { env } from '$env/dynamic/private';
 * console.log(env.DEPLOYMENT_SPECIFIC_VARIABLE);
 * ```
 *
 * > In `dev`, `$env/dynamic` always includes environment variables from `.env`. In `prod`, this behavior will depend on your adapter.
 */
declare module '$env/dynamic/private' {
	export const env: {
		_ZO_DOCTOR: string;
		NIX_PROFILES: string;
		TERM_PROGRAM: string;
		XDG_DATA_HOME: string;
		NODE: string;
		ASDF_INSTALL_TYPE: string;
		INIT_CWD: string;
		TMUX_TMPDIR: string;
		_P9K_TTY: string;
		ASDF_DIR: string;
		TERM: string;
		SHELL: string;
		HOMEBREW_REPOSITORY: string;
		TMPDIR: string;
		npm_config_npm_globalconfig: string;
		TERM_PROGRAM_VERSION: string;
		FZF_TMUX: string;
		TAURI_CLI_VERBOSITY: string;
		SPINNER_PID: string;
		TAURI_ENV_DEBUG: string;
		npm_config_registry: string;
		ASDF_INSTALL_PATH: string;
		ZSH_TMUX_TERM: string;
		USER: string;
		TAURI_ENV_TARGET_TRIPLE: string;
		ASDF_INSTALL_VERSION: string;
		COMMAND_MODE: string;
		PNPM_SCRIPT_SRC_DIR: string;
		npm_config_globalconfig: string;
		_ZSH_TMUX_FIXED_CONFIG: string;
		SSH_AUTH_SOCK: string;
		__CF_USER_TEXT_ENCODING: string;
		npm_execpath: string;
		FZF_DEFAULT_OPTS: string;
		PAGER: string;
		ZSH_TMUX_CONFIG: string;
		TAURI_ENV_PLATFORM: string;
		TMUX: string;
		WEZTERM_EXECUTABLE_DIR: string;
		npm_config_frozen_lockfile: string;
		npm_config_verify_deps_before_run: string;
		XDG_CONFIG_DIRS: string;
		PATH: string;
		TERMINFO_DIRS: string;
		npm_package_json: string;
		FLOAX_WIDTH: string;
		TAURI_ENV_FAMILY: string;
		TAURI_ENV_PLATFORM_VERSION: string;
		__CFBundleIdentifier: string;
		resurrect_dir: string;
		NIX_PATH: string;
		PWD: string;
		npm_command: string;
		P9K_SSH: string;
		npm_lifecycle_event: string;
		EDITOR: string;
		FLOAX_BORDER_COLOR: string;
		npm_config__jsr_registry: string;
		npm_package_name: string;
		LANG: string;
		P9K_TTY: string;
		WEZTERM_PANE: string;
		XDG_STATE_HOME: string;
		NODE_PATH: string;
		TMUX_PANE: string;
		XPC_FLAGS: string;
		TAURI_ENV_ARCH: string;
		NIX_SSL_CERT_FILE: string;
		FLOAX_TITLE: string;
		npm_config_node_gyp: string;
		XPC_SERVICE_NAME: string;
		pnpm_config_verify_deps_before_run: string;
		npm_package_version: string;
		WEZTERM_UNIX_SOCKET: string;
		FLOAX_HEIGHT: string;
		YSU_VERSION: string;
		SHLVL: string;
		HOME: string;
		FLOAX_CHANGE_PATH: string;
		XDG_CONFIG_HOME: string;
		ATUIN_HISTORY_ID: string;
		HOMEBREW_PREFIX: string;
		ZSH_CACHE_DIR: string;
		WEZTERM_CONFIG_DIR: string;
		XDG_CACHE_HOME: string;
		LOGNAME: string;
		MACOSX_DEPLOYMENT_TARGET: string;
		FLOAX_TEXT_COLOR: string;
		ATUIN_SESSION: string;
		npm_lifecycle_script: string;
		XDG_DATA_DIRS: string;
		FZF_DEFAULT_COMMAND: string;
		BROWSER: string;
		npm_config_user_agent: string;
		HOMEBREW_CELLAR: string;
		INFOPATH: string;
		_P9K_SSH_TTY: string;
		WEZTERM_EXECUTABLE: string;
		__HM_SESS_VARS_SOURCED: string;
		WEZTERM_CONFIG_FILE: string;
		npm_node_execpath: string;
		__HM_ZSH_SESS_VARS_SOURCED: string;
		NIX_USER_PROFILE_DIR: string;
		__NIX_DARWIN_SET_ENVIRONMENT_DONE: string;
		COLORTERM: string;
		NODE_ENV: string;
		[key: `PUBLIC_${string}`]: undefined;
		[key: `${string}`]: string | undefined;
	};
}

/**
 * Similar to [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), but only includes variables that begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) (which defaults to `PUBLIC_`), and can therefore safely be exposed to client-side code.
 *
 * Note that public dynamic environment variables must all be sent from the server to the client, causing larger network requests — when possible, use `$env/static/public` instead.
 *
 * Dynamic environment variables cannot be used during prerendering.
 *
 * ```ts
 * import { env } from '$env/dynamic/public';
 * console.log(env.PUBLIC_DEPLOYMENT_SPECIFIC_VARIABLE);
 * ```
 */
declare module '$env/dynamic/public' {
	export const env: {
		[key: `PUBLIC_${string}`]: string | undefined;
	};
}
