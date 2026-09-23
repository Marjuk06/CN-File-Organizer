import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export type OrganizeMode = 'Smart' | 'ByCategory' | 'ByExtension' | 'ByDate' | 'BySize';

export interface ScanOptions {
  recursive: boolean;
  max_depth?: number;
  skip_hidden: boolean;
  follow_links: boolean;
}

export interface ScanSummary {
  root_path: string;
  total_files: number;
  total_size_bytes: number;
  categories: Record<string, number>;
  extensions: Record<string, number>;
}

export interface OperationPlan {
  id: string;
  mode: OrganizeMode;
  source_directory: string;
  operations: FileOperation[];
  conflicts: any[];
  total_files: number;
  total_bytes: number;
}

export interface FileOperation {
  source_path: string;
  target_path: string;
  category: string;
  file_size: number;
}

export interface ExecutionResult {
  operation_id: string;
  status: 'Completed' | 'CompletedWithErrors' | 'Cancelled' | 'Failed';
  started_at: string;
  completed_at: string;
  files_processed: number;
  bytes_processed: number;
  errors: string[];
}

export interface HistoryEntry {
  operation_id: string;
  timestamp: string;
  mode: string;
  source_dir: string;
  files_processed: number;
  bytes_processed: number;
  status: string;
  is_undone: boolean;
  plan_snapshot?: OperationPlan;
}

export const api = {
  scanDirectory: (path: string, options?: ScanOptions) => 
    invoke<ScanSummary>('scan_directory', { path, options }),
    
  createPlan: (summary: ScanSummary, mode: OrganizeMode, destination?: string) =>
    invoke<OperationPlan>('create_plan', { summary, mode, destination }),
    
  executePlan: (plan: OperationPlan) =>
    invoke<ExecutionResult>('execute_plan_command', { plan }),
    
  cancelExecution: (operationId: string) =>
    invoke<void>('cancel_execution', { operationId }),
    
  getHistory: () =>
    invoke<HistoryEntry[]>('get_history'),
    
  clearHistory: () =>
    invoke<void>('clear_history'),
    
  undoOperation: (result: ExecutionResult) =>
    invoke<number>('undo_operation', { result }),
    
  getSettings: () => invoke<any>('get_settings'),
  saveSettings: (settings: any) => invoke<void>('save_settings', { settings }),
  
  getRules: () => invoke<any[]>('get_rules'),
  saveRules: (rules: any[]) => invoke<void>('save_rules', { rules }),
  
  getDiagnostics: () => invoke<any>('get_diagnostics'),
};

export function listenToProgress(callback: (event: any) => void) {
  return listen('execute-progress', (event) => {
    callback(event.payload);
  });
}
