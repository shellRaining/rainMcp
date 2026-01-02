export type Step =
  | 'select-type'
  | 'select-schema'
  | 'select-package'
  | 'configure'
  | 'custom-form'
  | 'remote-form'
  | 'clipboard-import';
export type ServerType = 'registry' | 'custom' | 'remote' | 'clipboard';
