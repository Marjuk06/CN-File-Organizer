import React, { useEffect, useState } from 'react';
import { History as HistoryIcon, RotateCcw, Search, Calendar, FileType, CheckCircle2, XCircle } from 'lucide-react';
import { api, HistoryEntry } from '../lib/tauri';
import { Button } from '../components/ui/Button';
import { Card, CardContent } from '../components/ui/Card';
import { formatBytes } from '../lib/utils';
import styles from './History.module.css';

const History = () => {
  const [history, setHistory] = useState<HistoryEntry[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [undoing, setUndoing] = useState<string | null>(null);

  useEffect(() => {
    loadHistory();
  }, []);

  const loadHistory = async () => {
    setIsLoading(true);
    try {
      const data = await api.getHistory();
      setHistory(data.reverse()); // Show newest first
    } catch (err) {
      console.error('Failed to load history', err);
    } finally {
      setIsLoading(false);
    }
  };

  const handleUndo = async (entry: HistoryEntry) => {
    if (entry.is_undone) return;
    
    // Create a mock execution result to pass to undo (since the backend expects one)
    // Real implementation would either store full results or fetch by ID
    setUndoing(entry.operation_id);
    try {
      await api.undoOperation({
        operation_id: entry.operation_id,
        status: entry.status as any,
        started_at: entry.timestamp,
        completed_at: entry.timestamp,
        files_processed: entry.files_processed,
        bytes_processed: entry.bytes_processed,
        errors: []
      });
      await loadHistory();
    } catch (err) {
      console.error('Failed to undo', err);
      alert('Failed to undo operation: ' + err);
    } finally {
      setUndoing(null);
    }
  };

  const handleClearHistory = async () => {
    if (confirm('Are you sure you want to clear the history? This cannot be undone.')) {
      try {
        await api.clearHistory();
        await loadHistory();
      } catch (err) {
        console.error('Failed to clear history', err);
      }
    }
  };

  return (
    <div className={styles.container}>
      <div className={styles.header}>
        <div className={styles.titleArea}>
          <HistoryIcon size={32} className={styles.icon} />
          <div>
            <h1 className={styles.title}>Operation History</h1>
            <p className={styles.subtitle}>Review past operations and undo them if needed.</p>
          </div>
        </div>
        <Button variant="ghost" onClick={handleClearHistory} disabled={history.length === 0}>
          Clear History
        </Button>
      </div>

      <div className={styles.historyList}>
        {isLoading ? (
          <div className={styles.emptyState}>Loading history...</div>
        ) : history.length === 0 ? (
          <Card className="glass-panel">
            <CardContent className={styles.emptyState}>
              <Search size={48} className={styles.emptyIcon} />
              <h3>No History Yet</h3>
              <p>You haven't organized any files yet. Go to the Organize tab to get started.</p>
            </CardContent>
          </Card>
        ) : (
          history.map((entry) => (
            <Card key={entry.operation_id} className={`glass-panel ${entry.is_undone ? styles.undoneCard : ''}`}>
              <CardContent className={styles.historyEntry}>
                <div className={styles.entryMain}>
                  <div className={styles.entryStatus}>
                    {entry.status === 'Completed' ? (
                      <CheckCircle2 size={24} className={styles.statusSuccess} />
                    ) : (
                      <XCircle size={24} className={styles.statusError} />
                    )}
                  </div>
                  <div className={styles.entryDetails}>
                    <h3 className={styles.entryMode}>{entry.mode} Organization</h3>
                    <div className={styles.entryMeta}>
                      <span className={styles.metaItem}>
                        <Calendar size={14} /> 
                        {new Date(entry.timestamp).toLocaleString()}
                      </span>
                      <span className={styles.metaItem}>
                        <FileType size={14} /> 
                        {entry.files_processed} files ({formatBytes(entry.bytes_processed)})
                      </span>
                    </div>
                    <div className={styles.entryPath}>{entry.source_dir}</div>
                  </div>
                </div>
                <div className={styles.entryActions}>
                  {entry.is_undone ? (
                    <span className={styles.undoneBadge}>Undone</span>
                  ) : (
                    <Button 
                      variant="secondary" 
                      onClick={() => handleUndo(entry)}
                      disabled={undoing === entry.operation_id || entry.status !== 'Completed'}
                    >
                      {undoing === entry.operation_id ? 'Undoing...' : <><RotateCcw size={16} style={{marginRight: '6px'}}/> Undo</>}
                    </Button>
                  )}
                </div>
              </CardContent>
            </Card>
          ))
        )}
      </div>
    </div>
  );
};

export default History;
