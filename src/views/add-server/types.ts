export type Step =
  | 'select-type'
  | 'select-schema'
  | 'select-package'
  | 'configure'
  | 'custom-form'
  | 'remote-form'
  | 'clipboard-import'
  | 'ai-chat';
export type ServerType = 'registry' | 'custom' | 'remote' | 'clipboard' | 'ai';
