import { readText } from '@tauri-apps/plugin-clipboard-manager';
import type { AgentServerEntry } from '@/types/mcp';

export interface ParsedServer {
  name: string;
  config: AgentServerEntry;
  description?: string;
}

export interface ParseSuccess {
  ok: true;
  servers: ParsedServer[];
}

export interface ParseError {
  ok: false;
  message: string;
  hint?: string;
}

export type ParseResult = ParseSuccess | ParseError;

export function useClipboardParser() {
  async function parseFromClipboard(): Promise<ParseResult> {
    const text = await readText();
    if (!text?.trim()) {
      return { ok: false, message: 'Clipboard is empty' };
    }

    return parseContent(text);
  }

  function parseContent(input: string): ParseResult {
    const trimmed = input.trim();

    // Try direct JSON parse
    let json = tryParseJson(trimmed);

    // If failed, try to fix common fragment issues
    if (!json) {
      json = tryFixAndParse(trimmed);
    }

    if (!json) {
      return {
        ok: false,
        message: 'Failed to parse as JSON',
        hint: 'Please ensure the copied content is a valid MCP configuration',
      };
    }

    // Identify structure and extract servers
    return identifyAndExtract(json);
  }

  return { parseFromClipboard, parseContent };
}

function tryParseJson(text: string): unknown | null {
  try {
    return JSON.parse(text);
  } catch {
    return null;
  }
}

function tryFixAndParse(text: string): unknown | null {
  // Attempt 1: Wrap as object { ... }
  // Detect "key": { pattern
  if (/^\s*"[^"]+"\s*:\s*\{/.test(text)) {
    // Remove trailing comma if exists
    let wrapped = text.trim();
    if (wrapped.endsWith(',')) {
      wrapped = wrapped.slice(0, -1);
    }
    wrapped = `{${wrapped}}`;
    const result = tryParseJson(wrapped);
    if (result) return result;
  }

  // Attempt 2: Remove trailing comma
  if (text.endsWith(',')) {
    const result = tryParseJson(text.slice(0, -1));
    if (result) return result;
  }

  return null;
}

function identifyAndExtract(value: unknown): ParseResult {
  if (typeof value !== 'object' || value === null) {
    return { ok: false, message: 'Invalid configuration format' };
  }

  const obj = value as Record<string, unknown>;

  // Case 1: { "mcpServers": { ... } }
  if (obj.mcpServers && typeof obj.mcpServers === 'object') {
    return extractServerMap(obj.mcpServers as Record<string, unknown>);
  }

  // Case 2: { "url": "..." } - bare remote config
  if (typeof obj.url === 'string') {
    const config = extractRemoteConfig(obj);
    if (config) {
      return {
        ok: true,
        servers: [
          {
            name: suggestServerName({ type: 'remote', ...config }),
            config: { type: 'remote', ...config },
            description: obj.description as string | undefined,
          },
        ],
      };
    }
  }

  // Case 3: { "command": "..." } - bare local config
  if (typeof obj.command === 'string') {
    const config = extractLocalConfig(obj);
    if (config) {
      return {
        ok: true,
        servers: [
          {
            name: suggestServerName({ type: 'local', ...config }),
            config: { type: 'local', ...config },
          },
        ],
      };
    }
  }

  // Case 4: { "name": { ... }, "name2": { ... } } - server map
  // Check: all values are objects containing url or command
  if (looksLikeServerMap(obj)) {
    return extractServerMap(obj);
  }

  return { ok: false, message: 'Cannot recognize as MCP configuration' };
}

function looksLikeServerMap(obj: Record<string, unknown>): boolean {
  const values = Object.values(obj);
  if (values.length === 0) return false;

  return values.every((v) => {
    if (typeof v !== 'object' || v === null) return false;
    const entry = v as Record<string, unknown>;
    return typeof entry.url === 'string' || typeof entry.command === 'string';
  });
}

function extractServerMap(obj: Record<string, unknown>): ParseResult {
  const servers: ParsedServer[] = [];

  for (const [name, value] of Object.entries(obj)) {
    if (typeof value !== 'object' || value === null) continue;
    const entry = value as Record<string, unknown>;

    if (typeof entry.url === 'string') {
      const config = extractRemoteConfig(entry);
      if (config) {
        servers.push({
          name,
          config: { type: 'remote', ...config },
          description: entry.description as string | undefined,
        });
      }
    } else if (typeof entry.command === 'string') {
      const config = extractLocalConfig(entry);
      if (config) {
        servers.push({ name, config: { type: 'local', ...config } });
      }
    }
  }

  if (servers.length === 0) {
    return { ok: false, message: 'No valid server configurations found' };
  }

  return { ok: true, servers };
}

interface RemoteConfigFields {
  url: string;
  headers?: Record<string, string> | null;
  timeout?: number | null;
}

function extractRemoteConfig(obj: Record<string, unknown>): RemoteConfigFields | null {
  if (typeof obj.url !== 'string') return null;
  return {
    url: obj.url,
    headers: isStringRecord(obj.headers) ? obj.headers : null,
    timeout: typeof obj.timeout === 'number' ? obj.timeout : null,
  };
}

interface LocalConfigFields {
  command: string;
  args?: string[] | null;
  env?: Record<string, string> | null;
  timeout?: number | null;
}

function extractLocalConfig(obj: Record<string, unknown>): LocalConfigFields | null {
  if (typeof obj.command !== 'string') return null;
  return {
    command: obj.command,
    args: Array.isArray(obj.args) && obj.args.every((a) => typeof a === 'string') ? obj.args : null,
    env: isStringRecord(obj.env) ? obj.env : null,
    timeout: typeof obj.timeout === 'number' ? obj.timeout : null,
  };
}

function isStringRecord(value: unknown): value is Record<string, string> {
  if (typeof value !== 'object' || value === null) return false;
  return Object.entries(value).every(([k, v]) => typeof k === 'string' && typeof v === 'string');
}

function suggestServerName(config: AgentServerEntry): string {
  if (config.type === 'remote') {
    try {
      const url = new URL(config.url);
      const pathPart = url.pathname.replace(/^\/+|\/+$/g, '').replace(/\//g, '-');
      if (url.hostname === 'localhost' || url.hostname === '127.0.0.1') {
        return pathPart || `local-${url.port}`;
      }
      const hostPart = url.hostname.replace(/^(www|api)\./, '').split('.')[0];
      return pathPart ? `${hostPart}-${pathPart}` : `${hostPart}-mcp`;
    } catch {
      return 'remote-server';
    }
  }

  if (config.type === 'local') {
    const args = config.args || [];
    for (const arg of args) {
      // @scope/package-name -> package-name
      const scopedMatch = arg.match(/@[\w-]+\/([\w-]+)/);
      if (scopedMatch) return scopedMatch[1];
      // Regular package name (exclude flags like -y)
      if (/^[\w-]+$/.test(arg) && !arg.startsWith('-')) {
        return arg;
      }
    }
    return (
      config.command
        .split(/[/\\]/)
        .pop()
        ?.replace(/\.\w+$/, '') || 'local-server'
    );
  }

  return 'mcp-server';
}
