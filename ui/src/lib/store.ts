import { create } from 'zustand';
import { ScanSummary, OperationPlan, ExecutionResult } from './tauri';

interface AppState {
  // Organizing flow state
  currentScan: ScanSummary | null;
  setCurrentScan: (scan: ScanSummary | null) => void;
  
  currentPlan: OperationPlan | null;
  setCurrentPlan: (plan: OperationPlan | null) => void;
  
  // Execution progress
  isExecuting: boolean;
  setIsExecuting: (isExecuting: boolean) => void;
  
  executionProgress: {
    operation_id: string;
    files_processed: number;
    bytes_processed: number;
    current_file: string;
  } | null;
  setExecutionProgress: (progress: any) => void;
}

export const useAppStore = create<AppState>((set) => ({
  currentScan: null,
  setCurrentScan: (scan) => set({ currentScan: scan }),
  
  currentPlan: null,
  setCurrentPlan: (plan) => set({ currentPlan: plan }),
  
  isExecuting: false,
  setIsExecuting: (isExecuting) => set({ isExecuting }),
  
  executionProgress: null,
  setExecutionProgress: (progress) => set({ executionProgress: progress }),
}));
