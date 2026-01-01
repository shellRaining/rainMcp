import { info, warn, error, debug } from '@tauri-apps/plugin-log';

/**
 * 格式化日志参数为字符串
 */
function formatMessage(...args: unknown[]): string {
  return args
    .map((arg) => {
      if (arg === null) return 'null';
      if (arg === undefined) return 'undefined';
      if (typeof arg === 'object') {
        try {
          return JSON.stringify(arg, null, 2);
        } catch {
          return String(arg);
        }
      }
      return String(arg);
    })
    .join(' ');
}

export const logger = {
  info: (...args: unknown[]) => {
    info(formatMessage(...args));
  },
  warn: (...args: unknown[]) => {
    warn(formatMessage(...args));
  },
  error: (...args: unknown[]) => {
    error(formatMessage(...args));
  },
  debug: (...args: unknown[]) => {
    debug(formatMessage(...args));
  },
};
