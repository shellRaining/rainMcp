// This file is auto-generated from Rust types via JSON Schema.
// Do not edit manually. Run "bun run generate" to regenerate.

export type AgentServerEntry =
  | {
      args?: string[] | null;
      command: string;
      env?: {
        [k: string]: string;
      } | null;
      timeout?: number | null;
      type: "local";
    }
  | {
      headers?: {
        [k: string]: string;
      } | null;
      timeout?: number | null;
      type: "remote";
      url: string;
    };
/**
 * Supported AI coding agent types
 */
export type AgentType =
  | "ClaudeCode"
  | "Cursor"
  | "Windsurf"
  | "Cline"
  | "ClaudeDesktop"
  | "RooCode"
  | "Trae"
  | "GeminiCli"
  | "Kiro"
  | "OpenAiCodex"
  | "Comate"
  | "VsCodeCopilot"
  | "CopilotCli"
  | "Alma"
  | "OpenCode";
/**
 * Local transport configuration
 */
export type LocalTransport =
  | {
      type: "stdio";
    }
  | {
      headers?: EnvironmentVariable[];
      type: "sse";
      url: string;
    }
  | {
      headers?: EnvironmentVariable[];
      type: "streamable-http";
      url: string;
    };
/**
 * Remote transport configuration
 */
export type RemoteTransport =
  | {
      headers?: EnvironmentVariable[];
      type: "sse";
      url: string;
      variables?: unknown;
    }
  | {
      headers?: EnvironmentVariable[];
      type: "streamable-http";
      url: string;
      variables?: unknown;
    };
/**
 * Origin type for a user server
 */
export type OriginType = "registry" | "custom";

/**
 * 包含所有需要导出的类型
 */
export interface AgentServers {
  servers: {
    [k: string]: AgentServerEntry;
  };
}
/**
 * Schema store containing all cached server schemas
 */
export interface SchemaStore {
  /**
   * List of server schemas
   */
  servers: ServerSchema[];
  /**
   * Timestamp of last update (ISO 8601 format)
   */
  updated_at?: string | null;
}
/**
 * Main server schema structure from MCP Registry
 */
export interface ServerSchema {
  $schema?: string | null;
  /**
   * Human-readable description of server functionality
   */
  description: string;
  /**
   * Icons for UI display
   */
  icons?: Icon[];
  /**
   * Server name in reverse-DNS format (e.g., "io.github.user/weather")
   */
  name: string;
  /**
   * Package configurations for local installation
   */
  packages?: Package[];
  /**
   * Remote transport configurations
   */
  remotes?: RemoteTransport[];
  /**
   * Repository metadata
   */
  repository?: Repository | null;
  /**
   * Optional human-readable title
   */
  title?: string | null;
  /**
   * Version string (should follow semantic versioning)
   */
  version: string;
  /**
   * Optional website URL
   */
  websiteUrl?: string | null;
}
/**
 * Icon for UI display
 */
export interface Icon {
  /**
   * MIME type
   */
  mimeType?: string | null;
  /**
   * Available sizes (e.g., ["48x48", "96x96"])
   */
  sizes?: string[];
  /**
   * URL to icon resource
   */
  src: string;
  /**
   * Theme this icon is designed for
   */
  theme?: string | null;
}
/**
 * Package configuration for local installation
 */
export interface Package {
  /**
   * Environment variables
   */
  environmentVariables?: EnvironmentVariable[];
  /**
   * SHA-256 hash for integrity verification
   */
  fileSha256?: string | null;
  /**
   * Package identifier or URL
   */
  identifier: string;
  /**
   * Package arguments
   */
  packageArguments?: Argument[];
  /**
   * Base URL of the package registry
   */
  registryBaseUrl?: string | null;
  /**
   * Registry type (npm, pypi, oci, nuget, mcpb)
   */
  registryType: string;
  /**
   * Runtime arguments
   */
  runtimeArguments?: Argument[];
  /**
   * Runtime hint (npx, uvx, docker, dnx)
   */
  runtimeHint?: string | null;
  /**
   * Transport configuration
   */
  transport: LocalTransport;
  /**
   * Package version
   */
  version?: string | null;
}
/**
 * Environment variable or header configuration (corresponds to Input/KeyValueInput in schema)
 */
export interface EnvironmentVariable {
  /**
   * Possible choices
   */
  choices?: string[];
  /**
   * Default value
   */
  default?: string | null;
  /**
   * Description of the variable
   */
  description?: string | null;
  /**
   * Input format (string, number, boolean, filepath)
   */
  format?: string | null;
  /**
   * Whether the variable is required
   */
  isRequired?: boolean;
  /**
   * Whether the variable contains sensitive data
   */
  isSecret?: boolean;
  /**
   * Variable name
   */
  name: string;
  /**
   * Placeholder text for UI
   */
  placeholder?: string | null;
  /**
   * Fixed value (not user-configurable)
   */
  value?: string | null;
}
/**
 * Command-line argument (corresponds to PositionalArgument | NamedArgument in schema)
 *
 * Note: Using struct instead of enum because API may return empty or unknown type values. See: https://github.com/modelcontextprotocol/registry for edge cases.
 */
export interface Argument {
  /**
   * Possible choices for the argument value
   */
  choices?: string[];
  /**
   * Default value for the argument
   */
  default?: string | null;
  /**
   * Description of the argument
   */
  description?: string | null;
  /**
   * Input format: "string", "number", "boolean", "filepath"
   */
  format?: string | null;
  /**
   * Whether the argument can be repeated
   */
  isRepeated?: boolean;
  /**
   * Whether the argument is required
   */
  isRequired?: boolean;
  /**
   * Whether the argument contains sensitive data (e.g., password, token)
   */
  isSecret?: boolean;
  /**
   * Argument name (for named arguments, including leading dashes like "--port")
   */
  name?: string | null;
  /**
   * Placeholder text for UI display
   */
  placeholder?: string | null;
  /**
   * Argument type: "positional", "named", or may be empty
   */
  type?: string;
  /**
   * Argument value (fixed value, not user-configurable)
   */
  value?: string | null;
  /**
   * Hint for what kind of value is expected (for positional arguments)
   */
  valueHint?: string | null;
}
/**
 * Repository metadata
 */
export interface Repository {
  /**
   * Repository ID from hosting service
   */
  id?: string | null;
  /**
   * Repository hosting service (e.g., "github")
   */
  source?: string | null;
  /**
   * Optional subfolder path in monorepo
   */
  subfolder?: string | null;
  /**
   * Repository URL
   */
  url?: string | null;
}
/**
 * Information about a supported agent
 */
export interface SupportedAgent {
  agent_type: AgentType;
  config_path: string;
  enabled: boolean;
  is_configured: boolean;
  mcp_config?: AgentServers | null;
  name: string;
}
/**
 * User-configured MCP server instance
 */
export interface UserServer {
  /**
   * Server configuration (local command or remote URL)
   */
  config: AgentServerEntry;
  /**
   * Timestamp of creation
   */
  createdAt?: string | null;
  /**
   * Unique identifier for this user server
   */
  id: string;
  /**
   * Display name for the server
   */
  name: string;
  /**
   * Origin information for UI display (optional)
   */
  origin?: ServerOrigin | null;
}
/**
 * Origin information for a user server (for UI display only)
 */
export interface ServerOrigin {
  /**
   * Origin type
   */
  originType: OriginType;
  /**
   * Package identifier: "{registry_type}:{identifier}" (e.g., "npm:@jina-ai/mcp-server")
   */
  packageId?: string | null;
  /**
   * Registry schema name (e.g., "io.jina/mcp-jina")
   */
  schemaName?: string | null;
}
