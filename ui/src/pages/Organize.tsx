import React, { useState, useEffect } from 'react';
import { open } from '@tauri-apps/plugin-dialog';
import { FolderSearch, Settings2, Play, CheckCircle2, AlertTriangle, Search } from 'lucide-react';
import { Button } from '../components/ui/Button';
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '../components/ui/Card';
import { Progress } from '../components/ui/Progress';
import { api, ScanSummary, OperationPlan, OrganizeMode, listenToProgress } from '../lib/tauri';
import { useAppStore } from '../lib/store';
import { formatBytes } from '../lib/utils';
import styles from './Organize.module.css';

const Organize = () => {
  const { 
    currentScan, setCurrentScan, 
    currentPlan, setCurrentPlan,
    isExecuting, setIsExecuting,
    executionProgress, setExecutionProgress 
  } = useAppStore();

  const [scanPath, setScanPath] = useState('');
  const [isScanning, setIsScanning] = useState(false);
  const [isPlanning, setIsPlanning] = useState(false);
  const [mode, setMode] = useState<OrganizeMode>('Smart');
  const [executionResult, setExecutionResult] = useState<any>(null);

  useEffect(() => {
    const unlisten = listenToProgress((event: any) => {
      if (event.event_type === 'started') {
        setIsExecuting(true);
      } else if (event.event_type === 'progress') {
        setExecutionProgress(event);
      } else if (event.event_type === 'finished' || event.event_type === 'complete' || event.event_type === 'error') {
        setIsExecuting(false);
        setExecutionResult(event);
      }
    });
    
    return () => {
      unlisten.then(f => f());
    };
  }, [setIsExecuting, setExecutionProgress]);

  const handleSelectFolder = async () => {
    const selected = await open({
      directory: true,
      multiple: false,
    });
    if (selected && typeof selected === 'string') {
      setScanPath(selected);
      setExecutionResult(null);
      setCurrentScan(null);
      setCurrentPlan(null);
    }
  };

  const handleScan = async () => {
    if (!scanPath) return;
    setIsScanning(true);
    try {
      const summary = await api.scanDirectory(scanPath, {
        recursive: true,
        skip_hidden: true,
        follow_links: false
      });
      setCurrentScan(summary);
      
      // Auto-plan after scan
      setIsPlanning(true);
      const plan = await api.createPlan(summary, mode);
      setCurrentPlan(plan);
    } catch (err) {
      console.error(err);
      // Add toast notification later
    } finally {
      setIsScanning(false);
      setIsPlanning(false);
    }
  };

  const handleExecute = async () => {
    if (!currentPlan) return;
    try {
      setIsExecuting(true);
      await api.executePlan(currentPlan);
    } catch (err) {
      console.error(err);
    }
  };

  const calculateProgress = () => {
    if (!currentPlan || !executionProgress || !executionProgress.bytes_processed) return 0;
    return (executionProgress.bytes_processed / currentPlan.total_bytes) * 100;
  };

  return (
    <div className={styles.container}>
      <div className={styles.header}>
        <h1 className={styles.title}>Organize Files</h1>
        <p className={styles.subtitle}>Scan a directory, preview the plan, and safely organize your files.</p>
      </div>

      {!isExecuting && !executionResult && (
        <Card className="glass-panel">
          <CardContent className={styles.scanSection}>
            <div className={styles.pathSelector}>
              <div className={styles.pathInput} onClick={handleSelectFolder}>
                {scanPath || "Select a folder to organize..."}
              </div>
              <Button onClick={handleSelectFolder} variant="secondary">Browse</Button>
            </div>
            
            <div className={styles.modeSelector}>
              <span className={styles.modeLabel}>Organization Mode:</span>
              <select 
                className={styles.select} 
                value={mode} 
                onChange={(e) => setMode(e.target.value as OrganizeMode)}
              >
                <option value="Smart">Smart AI</option>
                <option value="ByCategory">By Category</option>
                <option value="ByExtension">By Extension</option>
                <option value="ByDate">By Date (Year/Month)</option>
                <option value="BySize">By Size</option>
              </select>
            </div>

            <Button 
              size="lg" 
              onClick={handleScan} 
              disabled={!scanPath || isScanning}
              className={styles.scanButton}
            >
              {isScanning ? (
                <>Scanning...</>
              ) : (
                <><Search size={18} /> Analyze Directory</>
              )}
            </Button>
          </CardContent>
        </Card>
      )}

      {isPlanning && (
        <Card className="glass-panel">
          <CardContent className={styles.loadingState}>
            <div className={styles.spinner}></div>
            <p>Building operation plan...</p>
          </CardContent>
        </Card>
      )}

      {!isExecuting && !executionResult && currentPlan && !isPlanning && (
        <Card className="glass-panel">
          <CardHeader>
            <CardTitle>Operation Plan Ready</CardTitle>
            <CardDescription>
              {currentPlan.total_files} files ({formatBytes(currentPlan.total_bytes)}) will be organized using {mode} mode.
            </CardDescription>
          </CardHeader>
          <CardContent>
            {currentPlan.conflicts.length > 0 && (
              <div className={styles.warningAlert}>
                <AlertTriangle size={20} />
                <span>Found {currentPlan.conflicts.length} conflicts that will be skipped or renamed.</span>
              </div>
            )}
            
            <div className={styles.planSummary}>
              <div className={styles.planStat}>
                <span className={styles.statKey}>Total Operations:</span>
                <span className={styles.statVal}>{currentPlan.operations.length}</span>
              </div>
              <div className={styles.planStat}>
                <span className={styles.statKey}>Root Destination:</span>
                <span className={styles.statVal}>{currentPlan.source_directory}</span>
              </div>
            </div>
            
            <Button size="lg" className={styles.executeButton} onClick={handleExecute}>
              <Play size={18} /> Execute Plan
            </Button>
          </CardContent>
        </Card>
      )}

      {isExecuting && (
        <Card className="glass-panel">
          <CardContent className={styles.executingState}>
            <h2 className={styles.executingTitle}>Organizing Files...</h2>
            <Progress value={calculateProgress()} className={styles.progressBar} />
            <div className={styles.progressText}>
              {executionProgress?.bytes_processed ? formatBytes(executionProgress.bytes_processed) : '0 B'} / {formatBytes(currentPlan?.total_bytes || 0)}
            </div>
          </CardContent>
        </Card>
      )}
      
      {executionResult && (
        <Card className="glass-panel">
          <CardContent className={styles.successState}>
            <CheckCircle2 size={64} className={styles.successIcon} />
            <h2 className={styles.successTitle}>Organization Complete</h2>
            <p className={styles.successDesc}>
              {executionResult.outcome === "Completed" 
                ? "All files were successfully organized." 
                : "Organization finished with some skipped files."}
            </p>
            <Button onClick={() => setExecutionResult(null)} variant="secondary">
              Organize Another Folder
            </Button>
          </CardContent>
        </Card>
      )}
    </div>
  );
};

export default Organize;
