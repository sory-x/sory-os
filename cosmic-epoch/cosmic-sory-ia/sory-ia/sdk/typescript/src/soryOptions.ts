export type soryConfigValue = string | number | boolean | soryConfigValue[] | soryConfigObject;

export type soryConfigObject = { [key: string]: soryConfigValue };

export type soryOptions = {
  soryPathOverride?: string;
  baseUrl?: string;
  apiKey?: string;
  /**
   * Additional `--config key=value` overrides to pass to the sory CLI.
   *
   * Provide a JSON object and the SDK will flatten it into dotted paths and
   * serialize values as TOML literals so they are compatible with the CLI's
   * `--config` parsing.
   */
  config?: soryConfigObject;
  /**
   * Environment variables passed to the sory CLI process. When provided, the SDK
   * will not inherit variables from `process.env`.
   */
  env?: Record<string, string>;
};
