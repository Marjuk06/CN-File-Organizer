import React, { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { FolderTree, FileSpreadsheet, Shield, ArrowRight } from 'lucide-react';
import { Button } from '../components/ui/Button';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '../components/ui/Card';
import { api } from '../lib/tauri';
import styles from './Dashboard.module.css';

const Dashboard = () => {
  const navigate = useNavigate();
  const [stats, setStats] = useState({
    totalOrganized: 0,
    bytesOrganized: 0,
    operations: 0
  });

  useEffect(() => {
    // Fetch stats from history
    const loadStats = async () => {
      try {
        const history = await api.getHistory();
        
        let totalFiles = 0;
        let totalBytes = 0;
        
        // Filter out undone operations
        const validOps = history.filter(h => !h.is_undone && h.status === 'Completed');
        
        validOps.forEach(op => {
          totalFiles += op.files_processed;
          totalBytes += op.bytes_processed;
        });
        
        setStats({
          totalOrganized: totalFiles,
          bytesOrganized: totalBytes,
          operations: validOps.length
        });
      } catch (err) {
        console.error("Failed to load history stats", err);
      }
    };
    
    loadStats();
  }, []);

  const formatBytes = (bytes: number) => {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  };

  return (
    <div className={styles.dashboard}>
      <div className={styles.hero}>
        <h1 className={styles.title}>
          Welcome to <span className="text-gradient">CN Organizer</span>
        </h1>
        <p className={styles.subtitle}>
          The safest and smartest way to keep your files perfectly organized.
        </p>
        <Button size="lg" className={styles.cta} onClick={() => navigate('/organize')}>
          Start Organizing <ArrowRight size={18} style={{ marginLeft: 8 }} />
        </Button>
      </div>

      <div className={styles.statsGrid}>
        <Card className="glass-panel">
          <CardContent className={styles.statCard}>
            <div className={styles.statIconWrapper}>
              <FolderTree size={24} className={styles.statIcon} />
            </div>
            <div>
              <p className={styles.statLabel}>Files Organized</p>
              <h2 className={styles.statValue}>{stats.totalOrganized.toLocaleString()}</h2>
            </div>
          </CardContent>
        </Card>
        
        <Card className="glass-panel">
          <CardContent className={styles.statCard}>
            <div className={styles.statIconWrapper}>
              <FileSpreadsheet size={24} className={styles.statIcon} />
            </div>
            <div>
              <p className={styles.statLabel}>Storage Processed</p>
              <h2 className={styles.statValue}>{formatBytes(stats.bytesOrganized)}</h2>
            </div>
          </CardContent>
        </Card>
        
        <Card className="glass-panel">
          <CardContent className={styles.statCard}>
            <div className={styles.statIconWrapper}>
              <Shield size={24} className={styles.statIcon} />
            </div>
            <div>
              <p className={styles.statLabel}>Safe Operations</p>
              <h2 className={styles.statValue}>{stats.operations}</h2>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  );
};

export default Dashboard;
